use futures::{Future, Stream};
use reqwest::{RequestBuilder, Response, StatusCode};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::Sleep;

use crate::rate_limits::RateLimitInfo;

const USER_ID_KEY: &str = "user_id";
const COUNT_KEY: &str = "count";
const CURSOR_KEY: &str = "cursor";
const MAX_PAGE_SIZE: usize = 5000;

const ERROR_WAIT_BASE_S: i64 = 30;
const ERROR_WAIT_MULTIPLIER: f32 = 1.2;

const RETRYABLE_STATUS_CODES: [StatusCode; 2] = [
    StatusCode::INTERNAL_SERVER_ERROR,
    StatusCode::SERVICE_UNAVAILABLE,
];

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP client error")]
    HttpClient(#[from] reqwest::Error),
    #[error("User unavailable")]
    Unavailable(UnavailableReason),
    #[error("Unexpected status code")]
    UnexpectedStatus(StatusCode),
    #[error("Too many requests")]
    TooManyRequests,
    #[error("Rate limit error")]
    RateLimits(#[from] super::rate_limits::Error),
}

#[derive(Clone, serde::Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseBody {
    pub ids: Vec<u64>,
    pub next_cursor: i64,
    pub previous_cursor: i64,
}

pub struct FollowsStream<'a> {
    client: &'a reqwest::Client,
    endpoint: &'a str,
    bearer_token: String,
    user_id: u64,
    max_retries: usize,
    state: Option<FollowsStreamState>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnavailableReason {
    Deactivated,
    Suspended,
}

enum FollowsStreamState {
    Iterating {
        page: Vec<u64>,
        rate_limits: RateLimitInfo,
        index: usize,
        next_cursor: i64,
    },
    Calling {
        result: Pin<Box<dyn Future<Output = Result<Response, reqwest::Error>>>>,
        current_cursor: i64,
        current_retry: usize,
    },
    Decoding {
        result: Pin<Box<dyn Future<Output = Result<ResponseBody, reqwest::Error>>>>,
        rate_limits: RateLimitInfo,
        current_cursor: i64,
        current_retry: usize,
    },
    Waiting {
        sleep: Pin<Box<Sleep>>,
        next_cursor: i64,
        current_retry: usize,
    },
}

impl<'a> FollowsStream<'a> {
    pub fn new(
        client: &'a reqwest::Client,
        endpoint: &'a str,
        bearer_token: &str,
        user_id: u64,
    ) -> Self {
        let result = Box::pin(Self::make_url(client, endpoint, bearer_token, user_id, -1).send());

        Self {
            client,
            endpoint,
            bearer_token: bearer_token.to_string(),
            user_id,
            max_retries: 50,
            state: Some(FollowsStreamState::Calling {
                result,
                current_cursor: -1,
                current_retry: 0,
            }),
        }
    }

    fn make_url(
        client: &reqwest::Client,
        base: &str,
        bearer_token: &str,
        user_id: u64,
        cursor: i64,
    ) -> RequestBuilder {
        client.get(base).bearer_auth(bearer_token).query(&[
            (USER_ID_KEY, user_id.to_string().as_str()),
            (COUNT_KEY, MAX_PAGE_SIZE.to_string().as_str()),
            (CURSOR_KEY, cursor.to_string().as_str()),
        ])
    }

    fn call(&mut self, cursor: i64, retry: usize) {
        self.state = Some(FollowsStreamState::Calling {
            result: Box::pin(
                Self::make_url(
                    self.client,
                    self.endpoint,
                    &self.bearer_token,
                    self.user_id,
                    cursor,
                )
                .send(),
            ),
            current_cursor: cursor,
            current_retry: retry,
        });
    }

    fn wait(&mut self, duration: std::time::Duration, next_cursor: i64, retry: usize) {
        log::info!(
            "Waiting {:?} on attempt {} for {}",
            duration,
            retry,
            next_cursor
        );
        self.state = Some(FollowsStreamState::Waiting {
            sleep: Box::pin(tokio::time::sleep(duration)),
            next_cursor,
            current_retry: retry,
        });
    }

    fn error_wait(&mut self, next_cursor: i64, retry: usize) {
        self.wait(Self::error_wait_duration(retry), next_cursor, retry + 1)
    }

    fn error_wait_duration(retry: usize) -> Duration {
        let scale = ERROR_WAIT_MULTIPLIER.powi(retry as i32);
        // We define these constants above, so this should always be safe.
        Duration::try_from_secs_f32(ERROR_WAIT_BASE_S as f32 * scale).unwrap()
    }
}

impl Stream for FollowsStream<'_> {
    type Item = Result<u64, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.state.take() {
            Some(state) => match state {
                FollowsStreamState::Iterating {
                    page,
                    index,
                    next_cursor,
                    rate_limits,
                } => {
                    if index < page.len() {
                        let value = page[index];
                        self.state = Some(FollowsStreamState::Iterating {
                            page,
                            index: index + 1,
                            next_cursor,
                            rate_limits,
                        });

                        Poll::Ready(Some(Ok(value)))
                    } else {
                        if next_cursor != 0 {
                            match rate_limits.wait_from_now() {
                                None => {
                                    self.call(next_cursor, 0);
                                }
                                Some(duration) => {
                                    self.wait(duration, next_cursor, 0);
                                }
                            }
                        }

                        self.poll_next(cx)
                    }
                }
                FollowsStreamState::Calling {
                    mut result,
                    current_cursor,
                    current_retry,
                } => match Pin::new(&mut result).poll(cx) {
                    Poll::Pending => {
                        self.state = Some(FollowsStreamState::Calling {
                            result,
                            current_cursor,
                            current_retry,
                        });
                        Poll::Pending
                    }
                    Poll::Ready(Ok(response)) => match response.status() {
                        StatusCode::OK => match RateLimitInfo::try_from(response.headers()) {
                            Ok(rate_limits) => {
                                self.state = Some(FollowsStreamState::Decoding {
                                    result: Box::pin(response.json::<ResponseBody>()),
                                    rate_limits,
                                    current_cursor,
                                    current_retry,
                                });
                                self.poll_next(cx)
                            }
                            Err(error) => {
                                if current_retry == self.max_retries {
                                    Poll::Ready(Some(Err(error.into())))
                                } else {
                                    self.error_wait(current_cursor, current_retry);
                                    self.poll_next(cx)
                                }
                            }
                        },
                        StatusCode::UNAUTHORIZED => {
                            Poll::Ready(Some(Err(Error::Unavailable(UnavailableReason::Suspended))))
                        }
                        StatusCode::NOT_FOUND => Poll::Ready(Some(Err(Error::Unavailable(
                            UnavailableReason::Deactivated,
                        )))),
                        StatusCode::TOO_MANY_REQUESTS => {
                            if current_retry == self.max_retries {
                                Poll::Ready(Some(Err(Error::TooManyRequests)))
                            } else {
                                match response
                                    .headers()
                                    .get("retry-after")
                                    .and_then(|value| value.to_str().ok())
                                    .and_then(|retry_after| retry_after.parse::<u64>().ok())
                                {
                                    Some(retry_after_s) => {
                                        log::info!("Found Retry-After header: {}", retry_after_s);
                                        self.wait(
                                            Duration::from_secs(retry_after_s),
                                            current_cursor,
                                            current_retry + 1,
                                        );
                                        self.poll_next(cx)
                                    }
                                    None => {
                                        self.error_wait(current_cursor, current_retry);
                                        self.poll_next(cx)
                                    }
                                }
                            }
                        }
                        other if RETRYABLE_STATUS_CODES.contains(&other) => {
                            if current_retry == self.max_retries {
                                Poll::Ready(Some(Err(Error::UnexpectedStatus(other))))
                            } else {
                                self.error_wait(current_cursor, current_retry);
                                self.poll_next(cx)
                            }
                        }
                        other => Poll::Ready(Some(Err(Error::UnexpectedStatus(other)))),
                    },
                    Poll::Ready(Err(error)) => {
                        if current_retry == self.max_retries {
                            Poll::Ready(Some(Err(error.into())))
                        } else {
                            self.error_wait(current_cursor, current_retry);
                            self.poll_next(cx)
                        }
                    }
                },
                FollowsStreamState::Decoding {
                    mut result,
                    rate_limits,
                    current_cursor,
                    current_retry,
                } => match Pin::new(&mut result).poll(cx) {
                    Poll::Pending => {
                        self.state = Some(FollowsStreamState::Decoding {
                            result,
                            rate_limits,
                            current_cursor,
                            current_retry,
                        });
                        Poll::Pending
                    }
                    Poll::Ready(Ok(response_body)) => {
                        self.state = Some(FollowsStreamState::Iterating {
                            page: response_body.ids,
                            rate_limits,
                            index: 0,
                            next_cursor: response_body.next_cursor,
                        });
                        self.poll_next(cx)
                    }
                    Poll::Ready(Err(error)) => {
                        if current_retry == self.max_retries {
                            Poll::Ready(Some(Err(error.into())))
                        } else {
                            self.error_wait(current_cursor, current_retry);
                            self.poll_next(cx)
                        }
                    }
                },
                FollowsStreamState::Waiting {
                    mut sleep,
                    next_cursor,
                    current_retry,
                } => match Pin::new(&mut sleep).poll(cx) {
                    Poll::Pending => {
                        self.state = Some(FollowsStreamState::Waiting {
                            sleep,
                            next_cursor,
                            current_retry,
                        });
                        Poll::Pending
                    }
                    Poll::Ready(()) => {
                        self.call(next_cursor, current_retry);
                        self.poll_next(cx)
                    }
                },
            },
            None => Poll::Ready(None),
        }
    }
}

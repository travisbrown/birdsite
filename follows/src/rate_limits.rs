use chrono::{DateTime, Duration, TimeZone, Utc};

const RATE_LIMIT_RESET_HEADER: &str = "x-rate-limit-reset";
const RATE_LIMIT_REMAINING_HEADER: &str = "x-rate-limit-remaining";
const RATE_LIMIT_AVAILABILITY_BUFFER_S: i64 = 30;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Missing header")]
    MissingHeader(String),
    #[error("Invalid header value")]
    InvalidHeader(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitInfo {
    pub remaining: usize,
    pub reset: DateTime<Utc>,
}

impl RateLimitInfo {
    pub fn wait_from_now(&self) -> Option<std::time::Duration> {
        if self.remaining > 0 {
            None
        } else {
            let until_reset =
                self.reset + Duration::seconds(RATE_LIMIT_AVAILABILITY_BUFFER_S) - Utc::now();

            // The standard library `Duration` doesn't support negative values, so this will be
            // `None` if the reset (with buffer) is past, as intended.
            until_reset.to_std().ok()
        }
    }
}

impl TryFrom<&reqwest::header::HeaderMap> for RateLimitInfo {
    type Error = Error;
    fn try_from(value: &reqwest::header::HeaderMap) -> Result<Self, Self::Error> {
        let remaining_value =
            value
                .get(RATE_LIMIT_REMAINING_HEADER)
                .ok_or(Error::MissingHeader(
                    RATE_LIMIT_REMAINING_HEADER.to_string(),
                ))?;

        let reset_value = value
            .get(RATE_LIMIT_RESET_HEADER)
            .ok_or(Error::MissingHeader(RATE_LIMIT_RESET_HEADER.to_string()))?;

        let remaining = remaining_value
            .to_str()
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or(Error::InvalidHeader(
                RATE_LIMIT_REMAINING_HEADER.to_string(),
            ))?;

        let reset = reset_value
            .to_str()
            .ok()
            .and_then(|value| {
                value
                    .parse::<i64>()
                    .ok()
                    .and_then(|seconds| Utc.timestamp_opt(seconds, 0).single())
            })
            .ok_or(Error::InvalidHeader(RATE_LIMIT_RESET_HEADER.to_string()))?;

        Ok(Self { remaining, reset })
    }
}

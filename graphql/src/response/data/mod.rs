use crate::request::variables::Variables;
use birdsite::model::graphql::{tweet::TweetResult, unavailable::TweetUnavailableReason};
use bounded_static::IntoBoundedStatic;

mod tweet;
mod user;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    TweetResultsByRestIds(Vec<TweetResult<'static>>),
}

impl bounded_static::IntoBoundedStatic for Data {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

impl<'a> crate::archive::response::ParseWithVariables<'a, Variables> for Data {
    fn parse(input: &'a str, variables: &Variables) -> Result<Self, crate::archive::response::Error>
    where
        Self: Sized + 'a,
    {
        match variables {
            Variables::TweetResultsByRestIds(variables) => {
                let tweet_results =
                    serde_json::from_str::<tweet_results_by_rest_ids::Top<'_>>(input)?.tweet_result;

                if tweet_results.len() != variables.tweet_ids.len() {
                    Err(crate::archive::response::Error::InvalidResultLength {
                        expected: variables.tweet_ids.len(),
                        returned: tweet_results.len(),
                    })
                } else {
                    let tweet_results = tweet_results
                        .into_iter()
                        .zip(variables.tweet_ids.iter())
                        .map(|(tweet_result, tweet_id)| match tweet_result.result {
                            Some(tweet_result) => tweet_result.complete(*tweet_id).into_static(),
                            None => TweetResult::Unavailable {
                                id: *tweet_id,
                                reason: TweetUnavailableReason::Missing,
                            },
                        })
                        .collect::<Vec<_>>();

                    Ok(Data::TweetResultsByRestIds(tweet_results))
                }
            }
        }
    }
}

mod tweet_results_by_rest_ids {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Top<'a> {
        #[serde(borrow, rename = "tweetResult")]
        pub tweet_result: Vec<TweetResult<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TweetResult<'a> {
        #[serde(borrow)]
        pub result: Option<super::tweet::TweetResult<'a>>,
    }
}

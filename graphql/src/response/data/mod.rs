use crate::request::variables::{self, Variables};
use birdsite::model::graphql::{tweet::TweetResult, unavailable::TweetUnavailableReason};
use bounded_static::IntoBoundedStatic;

mod tweet;
mod user;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    AboutAccountQuery(Option<birdsite::model::graphql::user::about_account::UserResult<'static>>),
    BirdwatchFetchOneNote(Option<birdsite::model::graphql::birdwatch::note::Note<'static>>),
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
            Variables::AboutAccountQuery(variables) => {
                let user_result = serde_json::from_str::<about_account_query::Top<'_>>(input)?
                    .user_result_by_screen_name
                    .map(|user_result_by_screen_name| user_result_by_screen_name.result);

                Ok(Data::AboutAccountQuery(user_result.map(|user_result| {
                    user_result
                        .complete(variables.screen_name.clone())
                        .into_static()
                })))
            }
            Variables::BirdwatchFetchOneNote(_) => {
                let note = serde_json::from_str::<birdwatch_fetch_one_note::Top<'_>>(input)?
                    .birdwatch_note_by_rest_id;

                Ok(Data::BirdwatchFetchOneNote(note.into_static()))
            }
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
                                reason: Some(TweetUnavailableReason::Missing),
                            },
                        })
                        .collect::<Vec<_>>();

                    Ok(Data::TweetResultsByRestIds(tweet_results))
                }
            }
        }
    }
}

mod about_account_query {
    use birdsite::model::graphql::user::about_account::partial::UserResult;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Top<'a> {
        #[serde(borrow)]
        pub user_result_by_screen_name: Option<UserResultByScreenName<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct UserResultByScreenName<'a> {
        #[serde(rename = "id")]
        _id: Cow<'a, str>,
        #[serde(borrow)]
        pub result: UserResult<'a>,
    }
}

mod birdwatch_fetch_one_note {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Top<'a> {
        #[serde(borrow)]
        pub birdwatch_note_by_rest_id: Option<birdsite::model::graphql::birdwatch::note::Note<'a>>,
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
        pub result: Option<birdsite::model::graphql::tweet::partial::TweetResult<'a>>,
    }
}

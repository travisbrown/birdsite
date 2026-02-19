use crate::request::variables::Variables;
use birdsite::model::graphql::{
    tweet::TweetResult,
    unavailable::{TweetUnavailableReason, UserUnavailableReason},
    user::UserResult,
};
use bounded_static::{IntoBoundedStatic, ToBoundedStatic};

mod tweet;
mod user;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    AboutAccountQuery(Option<birdsite::model::graphql::user::about_account::UserResult<'static>>),
    BirdwatchFetchOneNote(Option<birdsite::model::graphql::birdwatch::note::Note<'static>>),
    TweetResultsByRestIds(Vec<TweetResult<'static>>),
    UserResultByRestId(birdsite::model::graphql::user::UserResult<'static>),
    BirdwatchFetchPublicData(birdsite::model::graphql::birdwatch::manifest::Bundle),
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
                let user_result = serde_json::from_str::<about_account_query::Data<'_>>(input)?
                    .complete(variables.screen_name.clone());

                Ok(Self::AboutAccountQuery(
                    user_result.map(|user_result| user_result.into_static()),
                ))
            }
            Variables::BirdwatchFetchOneNote(_) => {
                let note = serde_json::from_str::<birdwatch_fetch_one_note::Data<'_>>(input)?
                    .birdwatch_note_by_rest_id;

                Ok(Self::BirdwatchFetchOneNote(note.into_static()))
            }
            Variables::BirdwatchFetchPublicData(_) => {
                let bundle = serde_json::from_str::<birdwatch_fetch_public_data::Data>(input)?
                    .birdwatch_latest_public_data_file_bundle;

                Ok(Self::BirdwatchFetchPublicData(bundle))
            }
            Variables::TweetResultsByRestIds(variables) => {
                let tweet_results =
                    serde_json::from_str::<tweet_results_by_rest_ids::Data<'_>>(input)?
                        .tweet_result;

                if tweet_results.len() == variables.tweet_ids.len() {
                    let tweet_results = tweet_results
                        .into_iter()
                        .zip(variables.tweet_ids.iter())
                        .map(|(tweet_result, tweet_id)| {
                            tweet_result.result.map_or(
                                TweetResult::Unavailable {
                                    id: *tweet_id,
                                    reason: Some(TweetUnavailableReason::Missing),
                                },
                                |tweet_result| tweet_result.complete(*tweet_id).into_static(),
                            )
                        })
                        .collect::<Vec<_>>();

                    Ok(Self::TweetResultsByRestIds(tweet_results))
                } else {
                    Err(crate::archive::response::Error::InvalidResultLength {
                        expected: variables.tweet_ids.len(),
                        returned: tweet_results.len(),
                    })
                }
            }
            Variables::UserByRestId(variables) => {
                let user_result = serde_json::from_str::<user_by_rest_id::Data<'_>>(input)?
                    .user
                    .result;

                Ok(Self::UserResultByRestId(user_result.map_or_else(
                    || UserResult::Unavailable {
                        id: variables.user_id,
                        reason: UserUnavailableReason::Deactivated,
                    },
                    |user_result| user_result.complete(variables.user_id).to_static(),
                )))
            }
        }
    }
}

mod about_account_query {
    use birdsite::model::graphql::user::about_account::partial::UserResult;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data<'a> {
        #[serde(borrow)]
        user_result_by_screen_name: Option<UserResultByScreenName<'a>>,
    }

    impl<'a> Data<'a> {
        pub fn complete(
            self,
            screen_name: Cow<'a, str>,
        ) -> Option<birdsite::model::graphql::user::about_account::UserResult<'a>> {
            let user_result = self
                .user_result_by_screen_name
                .and_then(|user_result_by_screen_name| user_result_by_screen_name.result);

            user_result.map(|user_result| user_result.complete(screen_name))
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct UserResultByScreenName<'a> {
        #[serde(rename = "id")]
        _id: Cow<'a, str>,
        #[serde(borrow)]
        result: Option<UserResult<'a>>,
    }
}

mod birdwatch_fetch_one_note {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data<'a> {
        #[serde(borrow)]
        pub birdwatch_note_by_rest_id: Option<birdsite::model::graphql::birdwatch::note::Note<'a>>,
    }
}

mod birdwatch_fetch_public_data {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data {
        pub birdwatch_latest_public_data_file_bundle:
            birdsite::model::graphql::birdwatch::manifest::Bundle,
    }
}

mod tweet_results_by_rest_ids {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data<'a> {
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

mod user_by_rest_id {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data<'a> {
        #[serde(borrow)]
        pub user: User<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct User<'a> {
        #[serde(borrow)]
        pub result: Option<birdsite::model::graphql::user::partial::UserResult<'a>>,
    }
}

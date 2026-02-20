use crate::request::variables::Variables;
use birdsite::model::graphql::{
    tweet::TweetResult,
    unavailable::{TweetUnavailableReason, UserUnavailableReason},
    user::{User, UserResult},
};
use bounded_static::{IntoBoundedStatic, ToBoundedStatic};
use std::borrow::Cow;

mod tweet;
mod user;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommunityMembersResponse<'a> {
    pub members: Vec<birdsite::model::graphql::user::community::CommunityUser<'static>>,
    pub cursor: Option<Cow<'a, str>>,
}

// Note that we can't use `UserResult` because we do not have the user ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserByScreenNameResponse<'a> {
    Available(User<'a>),
    Suspended,
    /// Indicates either deactivation or a screen name change.
    NotFound,
    Incomplete,
    /// Another reason is given for unavailabilty (we do not expect to see this).
    OtherUnavailable(UserUnavailableReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {
    AboutAccountQuery(Option<birdsite::model::graphql::user::about_account::UserResult<'static>>),
    BirdwatchFetchOneNote(Option<birdsite::model::graphql::birdwatch::note::Note<'static>>),
    /// An empty value may indicate either suspension or deactivation.
    MembersSliceTimelineQuery(Option<CommunityMembersResponse<'static>>),
    TweetResultsByRestIds(Vec<TweetResult<'static>>),
    UserByScreenName(UserByScreenNameResponse<'static>),
    UserResultByRestId(UserResult<'static>),
    UsersByRestIds(Vec<UserResult<'static>>),
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
                    user_result.map(bounded_static::IntoBoundedStatic::into_static),
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
            Variables::MembersSliceTimelineQuery(_) => {
                let data = serde_json::from_str::<members_slice_timeline_query::Data<'_>>(input)?;

                Ok(Self::MembersSliceTimelineQuery(
                    data.into_community_response(),
                ))
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
            Variables::UsersByRestIds(variables) => {
                let user_results =
                    serde_json::from_str::<users_by_rest_ids::Data<'_>>(input)?.users;

                if user_results.len() == variables.user_ids.len() {
                    let user_results = user_results
                        .into_iter()
                        .zip(variables.user_ids.iter())
                        .map(|(wrapper, &user_id)| {
                            wrapper.result.map_or(
                                UserResult::Unavailable {
                                    id: user_id,
                                    reason: UserUnavailableReason::Deactivated,
                                },
                                |user_result| user_result.complete(user_id).into_static(),
                            )
                        })
                        .collect::<Vec<_>>();

                    Ok(Self::UsersByRestIds(user_results))
                } else {
                    Err(crate::archive::response::Error::InvalidResultLength {
                        expected: variables.user_ids.len(),
                        returned: user_results.len(),
                    })
                }
            }
            Variables::UserByScreenName(_) => {
                let user_result = serde_json::from_str::<user_by_screen_name::Data<'_>>(input)?
                    .user
                    .map(|user| user.result);

                let response =
                    user_result.map_or(UserByScreenNameResponse::NotFound, |user_result| {
                        match user_result.into_result() {
                            Ok(user) => UserByScreenNameResponse::Available(user.into_static()),
                            Err(None) => UserByScreenNameResponse::Incomplete,
                            Err(Some(UserUnavailableReason::Suspended)) => {
                                UserByScreenNameResponse::Suspended
                            }
                            Err(Some(other)) => UserByScreenNameResponse::OtherUnavailable(other),
                        }
                    });

                Ok(Self::UserByScreenName(response))
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

mod members_slice_timeline_query {
    use super::CommunityMembersResponse;
    use birdsite::model::graphql::user::community::CommunityUser;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data<'a> {
        #[serde(borrow, rename = "communityResults")]
        community_results: CommunityResults<'a>,
    }

    impl Data<'_> {
        pub fn into_community_response(self) -> Option<CommunityMembersResponse<'static>> {
            match self.community_results.result {
                Community::Community { members_slice, .. } => {
                    let cursor = members_slice
                        .slice_info
                        .next_cursor
                        .map(|s| Cow::Owned(s.to_string()));
                    let members = members_slice
                        .items_results
                        .into_iter()
                        .filter_map(|item| item.result)
                        .map(bounded_static::IntoBoundedStatic::into_static)
                        .collect::<Vec<_>>();
                    Some(CommunityMembersResponse { members, cursor })
                }
                Community::CommunityUnavailable => None,
            }
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct CommunityResults<'a> {
        id: &'a str,
        result: Community<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(tag = "__typename")]
    enum Community<'a> {
        Community {
            #[serde(rename = "id")]
            _id: &'a str,
            members_slice: MembersSlice<'a>,
        },
        CommunityUnavailable,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct MembersSlice<'a> {
        #[serde(borrow)]
        items_results: Vec<ItemResult<'a>>,
        slice_info: SliceInfo<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ItemResult<'a> {
        #[serde(rename = "id")]
        _id: &'a str,
        result: Option<CommunityUser<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct SliceInfo<'a> {
        next_cursor: Option<Cow<'a, str>>,
    }
}

mod users_by_rest_ids {
    use serde_field_attributes::integer_str;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data<'a> {
        #[serde(borrow)]
        pub users: Vec<UserResultWrapper<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct UserResultWrapper<'a> {
        #[serde(borrow)]
        pub result: Option<UserResult<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(tag = "__typename")]
    pub enum UserResult<'a> {
        User {
            #[serde(with = "integer_str")]
            rest_id: u64,
            #[serde(borrow)]
            core: Core<'a>,
            super_follow_eligible: Option<bool>,
        },
        UserUnavailable {
            reason: birdsite::model::graphql::unavailable::UserUnavailableReason,
        },
    }

    impl<'a> UserResult<'a> {
        pub fn complete(self, id: u64) -> birdsite::model::graphql::user::UserResult<'a> {
            match self {
                Self::User {
                    rest_id,
                    core,
                    super_follow_eligible,
                } => birdsite::model::graphql::user::UserResult::Available(
                    birdsite::model::graphql::user::User {
                        id: rest_id,
                        screen_name: core.screen_name,
                        name: core.name,
                        super_follow_eligible,
                        subscribers_count: None,
                        creator_subscriptions_count: None,
                    },
                ),
                Self::UserUnavailable { reason } => {
                    birdsite::model::graphql::user::UserResult::Unavailable { id, reason }
                }
            }
        }
    }

    #[derive(serde::Deserialize)]
    pub struct Core<'a> {
        #[serde(borrow)]
        pub screen_name: Cow<'a, str>,
        #[serde(borrow)]
        pub name: Cow<'a, str>,
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

mod user_by_screen_name {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Data<'a> {
        #[serde(borrow)]
        pub user: Option<User<'a>>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct User<'a> {
        #[serde(borrow)]
        pub result: birdsite::model::graphql::user::partial::UserResult<'a>,
    }
}

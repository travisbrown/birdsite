use crate::model::graphql::unavailable::TweetUnavailableReason;
use serde_field_attributes::integer_str;
use std::borrow::Cow;

#[derive(serde::Deserialize)]
#[serde(tag = "__typename")]
pub enum TweetResult<'a> {
    Tweet {
        #[serde(flatten)]
        tweet: Tweet<'a>,
    },
    TweetWithVisibilityResults {
        #[serde(borrow)]
        tweet: Tweet<'a>,
        // TODO: Extract this data.
        #[serde(rename = "tweetInterstitial")]
        tweet_interstitial: Option<serde::de::IgnoredAny>,
        #[serde(rename = "limitedActionResults")]
        limited_action_results: Option<serde::de::IgnoredAny>,
        #[serde(rename = "softInterventionPivot")]
        soft_intervention_pivot: Option<serde::de::IgnoredAny>,
        #[serde(rename = "mediaVisibilityResults")]
        media_visibility_results: Option<serde::de::IgnoredAny>,
    },
    TweetPreviewDisplay {
        #[serde(borrow)]
        tweet: super::preview::TweetPreview<'a>,
        limited_action_results: serde::de::IgnoredAny,
        cta: serde::de::IgnoredAny,
    },
    TweetUnavailable {
        reason: TweetUnavailableReason,
    },
    TweetTombstone {},
}

impl<'a> TweetResult<'a> {
    pub fn complete(self, id: u64) -> crate::model::graphql::tweet::TweetResult<'a> {
        match self {
            Self::Tweet { tweet } => tweet.into_tweet_result(),
            Self::TweetWithVisibilityResults { tweet, .. } => tweet.into_tweet_result(),
            Self::TweetPreviewDisplay { tweet, .. } => {
                crate::model::graphql::tweet::TweetResult::Preview(tweet)
            }
            Self::TweetUnavailable { reason } => {
                crate::model::graphql::tweet::TweetResult::Unavailable {
                    id,
                    reason: Some(reason),
                }
            }
            Self::TweetTombstone {} => {
                crate::model::graphql::tweet::TweetResult::Unavailable { id, reason: None }
            }
        }
    }
}

#[derive(serde::Deserialize)]
//#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    #[serde(with = "integer_str")]
    pub rest_id: u64,
    #[serde(borrow)]
    core: Option<UserCore<'a>>,
    legacy: Option<Legacy<'a>>,
}

impl<'a> Tweet<'a> {
    fn into_tweet_result(self) -> crate::model::graphql::tweet::TweetResult<'a> {
        match self
            .legacy
            .zip(self.core.and_then(|core| core.user_results.result))
        {
            Some((legacy, user_result)) => crate::model::graphql::tweet::TweetResult::Full(
                crate::model::graphql::tweet::Tweet {
                    id: self.rest_id,
                    user: user_result.complete(legacy.user_id),
                    full_text: legacy.full_text,
                },
            ),
            None => crate::model::graphql::tweet::TweetResult::Incomplete { id: self.rest_id },
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
//#[serde(deny_unknown_fields)]
struct Legacy<'a> {
    #[serde(rename = "id_str", with = "integer_str")]
    _id: u64,
    pub full_text: Cow<'a, str>,
    #[serde(rename = "user_id_str", with = "integer_str")]
    pub user_id: u64,
}

#[derive(serde::Deserialize)]
struct UserCore<'a> {
    #[serde(borrow)]
    user_results: UserResults<'a>,
}

#[derive(serde::Deserialize)]
struct UserResults<'a> {
    #[serde(borrow)]
    result: Option<crate::model::graphql::user::partial::UserResult<'a>>,
}

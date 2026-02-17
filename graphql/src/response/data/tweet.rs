use birdsite::model::graphql::unavailable::TweetUnavailableReason;
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
    TweetUnavailable {
        reason: TweetUnavailableReason,
    },
    /*TweetPreviewDisplay {
        #[serde(borrow)]
        tweet: Box<Option<super::super::TweetPreview<'a>>>,
        limited_action_results: Option<serde::de::IgnoredAny>,
        cta: Option<serde::de::IgnoredAny>,
    },
    TweetTombstone {
        tombstone: Option<TweetTombstone<'a>>,
    },*/
}

impl<'a> TweetResult<'a> {
    pub fn complete(self, id: u64) -> birdsite::model::graphql::tweet::TweetResult<'a> {
        match self {
            Self::Tweet { tweet } => birdsite::model::graphql::tweet::TweetResult::Available(
                birdsite::model::graphql::tweet::Tweet {
                    id: tweet.rest_id,
                    full_text: tweet.legacy.full_text,
                },
            ),
            Self::TweetWithVisibilityResults { tweet, .. } => {
                birdsite::model::graphql::tweet::TweetResult::Available(
                    birdsite::model::graphql::tweet::Tweet {
                        id: tweet.rest_id,
                        full_text: tweet.legacy.full_text,
                    },
                )
            }
            TweetResult::TweetUnavailable { reason } => {
                birdsite::model::graphql::tweet::TweetResult::Unavailable { id, reason }
            }
        }
    }
}

#[derive(serde::Deserialize)]
//#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    #[serde(with = "integer_str")]
    pub rest_id: u64,
    pub legacy: Legacy<'a>,
}

#[derive(Clone, Debug, serde::Deserialize)]
//#[serde(deny_unknown_fields)]
struct Legacy<'a> {
    #[serde(rename = "id_str", with = "integer_str")]
    pub id: u64,
    pub full_text: Cow<'a, str>,
}

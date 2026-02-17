use crate::model::{
    graphql::{
        unavailable::UserUnavailableReason,
        user::{User, mention::MentionResult},
    },
    metrics::TweetPublicMetrics,
};
use chrono::{DateTime, Utc};
use serde::de::Deserialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TweetPreview<'a> {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub user: Result<User<'a>, Option<UserUnavailableReason>>,
    pub text: Cow<'a, str>,
    pub entities: Entities<'a>,
    pub counts: TweetPublicMetrics,
    pub view_count: Option<usize>,
}

impl<'de: 'a, 'a> Deserialize<'de> for TweetPreview<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tweet_preview = internal::TweetPreview::deserialize(deserializer)?;

        Ok(Self {
            id: tweet_preview.rest_id,
            created_at: tweet_preview.created_at,
            user: tweet_preview.core.user_results.result.into_result(),
            text: tweet_preview.text,
            entities: tweet_preview.entities,
            counts: tweet_preview.counts,
            view_count: tweet_preview.view_count.count,
        })
    }
}

impl bounded_static::IntoBoundedStatic for TweetPreview<'_> {
    type Static = TweetPreview<'static>;

    fn into_static(self) -> Self::Static {
        Self::Static {
            id: self.id,
            created_at: self.created_at,
            user: match self.user {
                Ok(user) => Ok(user.into_static()),
                Err(reason) => Err(reason),
            },
            text: self.text.into_static(),
            entities: self.entities.into_static(),
            counts: self.counts,
            view_count: self.view_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, bounded_static_derive_more::ToStatic)]
pub struct Entities<'a> {
    #[serde(borrow)]
    pub user_mentions: Option<Vec<MentionResult<'a>>>,
}

mod internal {
    use crate::model::{
        attributes::text_timestamp, graphql::user::partial::UserResult, metrics::TweetPublicMetrics,
    };
    use chrono::{DateTime, Utc};
    use serde_field_attributes::{integer_str, optional_integer_str};
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TweetPreview<'a> {
        #[serde(with = "integer_str")]
        pub rest_id: u64,
        #[serde(with = "text_timestamp")]
        pub created_at: DateTime<Utc>,
        #[serde(borrow)]
        pub text: Cow<'a, str>,
        pub core: Core<'a>,
        pub entities: super::Entities<'a>,
        #[serde(flatten)]
        pub counts: TweetPublicMetrics,
        pub view_count: ViewCount,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Core<'a> {
        #[serde(borrow)]
        pub user_results: UserResults<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct UserResults<'a> {
        pub result: UserResult<'a>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ViewCount {
        // Missing on 2024-06-08.
        #[serde(with = "optional_integer_str", default)]
        pub count: Option<usize>,
    }
}

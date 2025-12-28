//! This data format appears for tweets in the Wayback Machine from around 9 December 2022 until into 2025.
use crate::model::{
    country::Country,
    lang::Lang,
    metrics::{TweetPublicMetrics, UserPublicMetrics},
    place::TypedCoordinates,
    source::SourceName,
};
use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use serde_field_attributes::{integer_str, optional_integer_str, optional_integer_str_array};
use std::borrow::Cow;

pub mod context;
pub mod entity;
pub mod error;
pub mod media;
pub mod place;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FormatError {
    #[error("Multiple referenced IDs")]
    MultipleReferencedIds(Vec<u64>),
    #[error("Missing referenced tweet")]
    MissingReferencedTweet(u64),
    #[error("Missing user")]
    MissingUser(u64),
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetSnapshot<'a> {
    #[serde(borrow)]
    pub data: Tweet<'a>,
    pub includes: TweetIncludes<'a>,
    pub errors: Option<Vec<error::TweetError<'a>>>,
}

impl<'a> TweetSnapshot<'a> {
    #[must_use]
    pub fn lookup_user(&self, id: u64) -> Option<&User<'a>> {
        self.includes.users.iter().find(|user| user.id == id)
    }

    #[must_use]
    pub fn lookup_tweet(&self, id: u64) -> Option<Tweet<'a>> {
        self.includes
            .tweets
            .as_ref()
            .and_then(|tweets| tweets.iter().find(|tweet| tweet.id == id))
            .cloned()
    }

    pub fn retweeted(&self) -> Result<Option<Tweet<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::Retweeted)
    }

    pub fn replied_to(&self) -> Result<Option<Tweet<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::RepliedTo)
    }

    pub fn quoted(&self) -> Result<Option<Tweet<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::Quoted)
    }

    /// Find referenced tweet.
    fn referenced_tweet(
        &self,
        reference_type: ReferenceType,
    ) -> Result<Option<Tweet<'a>>, FormatError> {
        self.data
            .referenced_tweet_id(reference_type)?
            .map(|id| {
                self.lookup_tweet(id)
                    .ok_or(FormatError::MissingReferencedTweet(id))
            })
            .map_or(Ok(None), |v| v.map(Some))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    #[serde(borrow)]
    pub article: Option<Article<'a>>,
    pub attachments: Option<Attachments>,
    #[serde(with = "integer_str")]
    pub id: u64,
    #[serde(with = "integer_str")]
    pub author_id: u64,
    pub context_annotations: Option<Vec<context::ContextAnnotation<'a>>>,
    #[serde(with = "integer_str")]
    pub conversation_id: u64,
    pub created_at: DateTime<Utc>,
    pub edit_controls: Option<EditControls>,
    #[serde(with = "optional_integer_str_array", default)]
    pub edit_history_tweet_ids: Option<Vec<u64>>,
    pub lang: Lang,
    pub entities: Option<entity::TweetEntities<'a>>,
    pub geo: Option<Geo<'a>>,
    pub note_tweet: Option<NoteTweet<'a>>,
    pub possibly_sensitive: bool,
    pub public_metrics: TweetPublicMetrics,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    pub reply_settings: ReplySettings,
    pub text: Cow<'a, str>,
    #[serde(with = "optional_integer_str", default)]
    pub in_reply_to_user_id: Option<u64>,
    pub source: Option<SourceName>,
    pub withheld: Option<Withheld>,
}

impl Tweet<'_> {
    pub fn retweeted_id(&self) -> Result<Option<u64>, FormatError> {
        self.referenced_tweet_id(ReferenceType::Retweeted)
    }

    pub fn replied_to_id(&self) -> Result<Option<u64>, FormatError> {
        self.referenced_tweet_id(ReferenceType::RepliedTo)
    }

    pub fn quoted_id(&self) -> Result<Option<u64>, FormatError> {
        self.referenced_tweet_id(ReferenceType::Quoted)
    }

    /// Find referenced tweet.
    pub fn referenced_tweet_id(
        &self,
        reference_type: ReferenceType,
    ) -> Result<Option<u64>, FormatError> {
        self.referenced_tweets
            .as_ref()
            .and_then(|referenced_tweets| {
                let mut ids = referenced_tweets.iter().filter_map(|referenced_tweet| {
                    if referenced_tweet.reference_type == reference_type {
                        Some(referenced_tweet.id)
                    } else {
                        None
                    }
                });

                ids.next().map(|id| {
                    ids.next().map_or(Ok(id), |multiple_id| {
                        let mut bad_ids = vec![id, multiple_id];
                        bad_ids.extend(ids);

                        Err(FormatError::MultipleReferencedIds(bad_ids))
                    })
                })
            })
            .map_or(Ok(None), |v| v.map(Some))
    }

    /*pub fn mention_ids(&self) -> Vec<u64> {
        self.entities
            .as_ref()
            .and_then(|entities| entities.mentions.as_ref())
            .map(|mentions| {
                mentions
                    .iter()
                    .filter_map(|mention| mention.id.and_then(|id| id.0))
                    .collect()
            })
            .unwrap_or_default()
    }*/
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attachments {
    pub media_keys: Option<Vec<String>>,
    #[serde(
        with = "optional_integer_str_array",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_tweet_id: Option<Vec<u64>>,
    #[serde(
        with = "optional_integer_str_array",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub poll_ids: Option<Vec<u64>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetIncludes<'a> {
    #[serde(borrow)]
    pub users: Vec<User<'a>>,
    pub tweets: Option<Vec<Tweet<'a>>>,
    pub media: Option<Vec<media::Media<'a>>>,
    pub polls: Option<Vec<Poll<'a>>>,
    pub places: Option<Vec<place::Place<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Poll<'a> {
    #[serde(with = "integer_str")]
    pub id: u64,
    pub voting_status: PollVotingStatus,
    pub duration_minutes: usize,
    pub end_datetime: DateTime<Utc>,
    pub options: Vec<PollOption<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PollVotingStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PollOption<'a> {
    pub position: usize,
    pub label: Cow<'a, str>,
    pub votes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Geo<'a> {
    pub place_id: Option<Cow<'a, str>>,
    pub coordinates: Option<TypedCoordinates>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EditControls {
    pub edits_remaining: isize,
    pub is_edit_eligible: bool,
    pub editable_until: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoteTweet<'a> {
    #[serde(borrow)]
    pub entities: Option<entity::TweetEntities<'a>>,
    pub text: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub reference_type: ReferenceType,
    #[serde(with = "integer_str")]
    pub id: u64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReferenceType {
    #[serde(rename = "retweeted")]
    Retweeted,
    #[serde(rename = "replied_to")]
    RepliedTo,
    #[serde(rename = "quoted")]
    Quoted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReplySettings {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "following")]
    Following,
    #[serde(rename = "mentionedUsers")]
    MentionedUsers,
    #[serde(rename = "subscribers")]
    Subscribers,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "integer_str")]
    pub id: u64,
    #[serde(borrow)]
    pub username: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub description: Cow<'a, str>,
    pub location: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub profile_image_url: Cow<'a, str>,
    #[serde(with = "optional_integer_str", default)]
    pub pinned_tweet_id: Option<u64>,
    pub entities: Option<entity::UserEntities<'a>>,
    pub verified: bool,
    pub protected: bool,
    pub public_metrics: UserPublicMetrics,
    pub withheld: Option<Withheld>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Withheld {
    pub copyright: Option<bool>,
    pub country_codes: Vec<Country>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Article<'a> {
    pub title: Option<Cow<'a, str>>,
}

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
use std::borrow::Cow;

pub mod context;
pub mod entity;
pub mod error;
pub mod media;
pub mod place;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FormatError {
    #[error("Multiple referenced IDs: {0:?}")]
    MultipleReferencedIds(Vec<u64>),
    #[error("Missing referenced tweet: {0}")]
    MissingReferencedTweet(u64),
    #[error("Missing user: {0}")]
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
        self.includes.users().find(|user| user.id == id)
    }

    #[must_use]
    pub fn lookup_tweet(&self, id: u64) -> Option<&Tweet<'a>> {
        self.includes
            .tweets
            .as_ref()
            .and_then(|tweets| tweets.iter().find(|tweet| tweet.id == id))
    }

    pub fn retweeted(&self) -> Result<Option<&Tweet<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::Retweeted)
    }

    pub fn replied_to(&self) -> Result<Option<&Tweet<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::RepliedTo)
    }

    pub fn quoted(&self) -> Result<Option<&Tweet<'a>>, FormatError> {
        self.referenced_tweet(ReferenceType::Quoted)
    }

    /// Find referenced tweet.
    fn referenced_tweet(
        &self,
        reference_type: ReferenceType,
    ) -> Result<Option<&Tweet<'a>>, FormatError> {
        self.data
            .referenced_tweet_id(reference_type)?
            .map(|id| {
                self.lookup_tweet(id)
                    .ok_or(FormatError::MissingReferencedTweet(id))
            })
            .transpose()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    #[serde(borrow)]
    pub article: Option<Article<'a>>,
    pub attachments: Option<Attachments>,
    #[serde(with = "crate::model::attributes::id_str")]
    pub id: u64,
    #[serde(with = "crate::model::attributes::id_str")]
    pub author_id: u64,
    pub context_annotations: Option<Vec<context::ContextAnnotation<'a>>>,
    #[serde(with = "crate::model::attributes::id_str")]
    pub conversation_id: u64,
    #[serde(with = "crate::model::attributes::millisecond_timestamp")]
    pub created_at: DateTime<Utc>,
    pub edit_controls: Option<EditControls>,
    #[serde(with = "crate::model::attributes::optional_ids_str", default)]
    pub edit_history_tweet_ids: Option<Cow<'a, [u64]>>,
    pub lang: Lang,
    pub entities: Option<entity::TweetEntities<'a>>,
    pub geo: Option<Geo<'a>>,
    pub note_tweet: Option<NoteTweet<'a>>,
    pub possibly_sensitive: bool,
    pub public_metrics: TweetPublicMetrics,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    pub reply_settings: ReplySettings,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
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
            .transpose()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attachments {
    pub media_keys: Option<Vec<String>>,
    #[serde(
        with = "crate::model::attributes::optional_ids_str",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_tweet_id: Option<Cow<'static, [u64]>>,
    #[serde(
        with = "crate::model::attributes::optional_ids_str",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub poll_ids: Option<Cow<'static, [u64]>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetIncludes<'a> {
    #[serde(borrow)]
    pub users: Vec<UserEntry<'a>>,
    pub tweets: Option<Vec<Tweet<'a>>>,
    pub media: Option<Vec<media::Media<'a>>>,
    pub polls: Option<Vec<Poll<'a>>>,
    pub places: Option<Vec<place::Place<'a>>>,
}

impl<'a> TweetIncludes<'a> {
    pub fn users(&self) -> impl Iterator<Item = &User<'a>> {
        self.users.iter().filter_map(UserEntry::user)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Poll<'a> {
    #[serde(with = "crate::model::attributes::id_str")]
    pub id: u64,
    pub voting_status: PollVotingStatus,
    pub duration_minutes: usize,
    #[serde(with = "crate::model::attributes::millisecond_timestamp")]
    pub end_datetime: DateTime<Utc>,
    #[serde(borrow)]
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
    #[serde(borrow)]
    pub label: Cow<'a, str>,
    pub votes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Geo<'a> {
    #[serde(borrow)]
    pub place_id: Option<Cow<'a, str>>,
    pub coordinates: Option<TypedCoordinates>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EditControls {
    pub edits_remaining: isize,
    pub is_edit_eligible: bool,
    #[serde(with = "crate::model::attributes::millisecond_timestamp")]
    pub editable_until: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoteTweet<'a> {
    #[serde(borrow)]
    pub entities: Option<entity::TweetEntities<'a>>,
    #[serde(borrow)]
    pub text: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub reference_type: ReferenceType,
    #[serde(with = "crate::model::attributes::id_str")]
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
    #[serde(rename = "other")]
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum UserEntry<'a> {
    User(#[serde(borrow)] Box<User<'a>>),
    DerivedUser(DerivedUser<'a>),
}

impl<'a> UserEntry<'a> {
    #[must_use]
    pub fn id(&self) -> u64 {
        match self {
            Self::User(user) => user.id,
            Self::DerivedUser(user) => user.id,
        }
    }

    #[must_use]
    pub fn user(&self) -> Option<&User<'a>> {
        match self {
            Self::User(user) => Some(user.as_ref()),
            Self::DerivedUser(_) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "crate::model::attributes::id_str")]
    pub id: u64,
    #[serde(borrow)]
    pub username: Cow<'a, str>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(with = "crate::model::attributes::millisecond_timestamp")]
    pub created_at: DateTime<Utc>,
    #[serde(borrow)]
    pub description: Cow<'a, str>,
    #[serde(borrow)]
    pub location: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub url: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub profile_image_url: Cow<'a, str>,
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
    pub pinned_tweet_id: Option<u64>,
    pub entities: Option<entity::UserEntities<'a>>,
    pub verified: bool,
    pub protected: bool,
    pub public_metrics: UserPublicMetrics,
    pub withheld: Option<Withheld>,
    #[serde(borrow)]
    pub derived: Option<Derived<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DerivedUser<'a> {
    #[serde(with = "crate::model::attributes::id_str")]
    pub id: u64,
    #[serde(borrow)]
    pub derived: Option<Derived<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Derived<'a> {
    #[serde(borrow)]
    pub locations: Vec<Location<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Location<'a> {
    #[serde(borrow)]
    pub full_name: Cow<'a, str>,
    pub country_code: Country,
    #[serde(borrow)]
    pub region: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub sub_region: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub locality: Option<Cow<'a, str>>,
    pub geo: TypedCoordinates,
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
    #[serde(borrow)]
    pub title: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests {
    use crate::test_support::{local_corpus, round_trip_jsonl};

    const FIXTURE: &str = include_str!("../../../../tests/data/wxj/data.jsonl");

    #[test]
    fn round_trips_tweet_snapshot_fixture() {
        round_trip_jsonl::<super::TweetSnapshot<'_>>("wxj/data fixture", FIXTURE);
    }

    #[test]
    fn round_trips_tweet_snapshot_corpus() {
        for (path, contents) in local_corpus("wxj/data") {
            round_trip_jsonl::<super::TweetSnapshot<'_>>(&path, &contents);
        }
    }

    /// Regression: `Tweet::text` must deserialize as `Cow::Borrowed` for escape-free input,
    /// confirming the `#[serde(borrow)]` attribute preserves the crate's zero-copy design.
    #[test]
    fn tweet_text_borrows_when_unescaped() {
        let json = concat!(
            r#"{"data":{"author_id":"1","conversation_id":"2","#,
            r#""created_at":"2026-05-19T13:42:45.000Z","id":"2","lang":"en","#,
            r#""possibly_sensitive":false,"public_metrics":{"retweet_count":0,"#,
            r#""reply_count":0,"like_count":0,"quote_count":0},"#,
            r#""reply_settings":"everyone","text":"hello world"},"#,
            r#""includes":{"users":[]}}"#
        );

        let snapshot = serde_json::from_str::<super::TweetSnapshot<'_>>(json).unwrap();

        assert!(matches!(snapshot.data.text, std::borrow::Cow::Borrowed(_)));
    }
}

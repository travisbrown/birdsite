//! This data format appears for tweets in the Wayback Machine from around 9 December 2022 onward.
use std::borrow::Cow;
use std::ops::Range;

use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use serde_field_attributes::optional_range;

use crate::model::country::Country;
use crate::model::lang::Lang;
use crate::model::metrics::{TweetPublicMetrics, UserPublicMetrics};
use crate::model::place::TypedCoordinates;
use crate::model::source::SourceName;

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
    #[error("Missing author for tweet: {0}")]
    MissingAuthor(u64),
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
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
    pub author_id: Option<u64>,
    #[serde(borrow)]
    pub card_uri: Option<media::Card<'a>>,
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
    pub community_id: Option<u64>,
    pub context_annotations: Option<Vec<context::ContextAnnotation<'a>>>,
    #[serde(with = "crate::model::attributes::id_str")]
    pub conversation_id: u64,
    pub created_at: DateTime<Utc>,
    #[serde(with = "optional_range", default)]
    pub display_text_range: Option<Range<usize>>,
    pub edit_controls: Option<EditControls>,
    #[serde(with = "crate::model::attributes::optional_ids_str", default)]
    pub edit_history_tweet_ids: Option<Cow<'a, [u64]>>,
    pub lang: Lang,
    pub entities: Option<entity::TweetEntities<'a>>,
    pub geo: Option<Geo<'a>>,
    pub media_metadata: Option<Vec<media::TweetMediaMetadata<'a>>>,
    pub note_request_suggestions: Option<Vec<NoteRequestSuggestion<'a>>>,
    pub note_tweet: Option<NoteTweet<'a>>,
    pub paid_partnership: Option<bool>,
    pub possibly_sensitive: bool,
    pub public_metrics: TweetPublicMetrics,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    pub reply_settings: ReplySettings,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
    pub in_reply_to_user_id: Option<u64>,
    pub source: Option<SourceName>,
    pub suggested_source_links: Option<Vec<Cow<'a, str>>>,
    pub suggested_source_links_with_counts: Option<Vec<SuggestedSourceLink<'a>>>,
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
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
    pub id: Option<u64>,
    pub voting_status: Option<PollVotingStatus>,
    pub duration_minutes: Option<usize>,
    pub end_datetime: Option<DateTime<Utc>>,
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

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoteRequestSuggestion<'a> {
    #[serde(borrow)]
    pub source_link: Option<Cow<'a, str>>,
    #[serde(borrow, default, deserialize_with = "deserialize_optional_text")]
    pub suggestion: Option<Cow<'a, str>>,
    #[serde(with = "crate::model::attributes::id_str")]
    pub suggestion_id: u64,
}

// Preserve borrowing of unescaped text even when the field is optional.
fn deserialize_optional_text<'de, D: serde::de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Cow<'de, str>>, D::Error> {
    use serde::de::Deserialize;

    #[derive(serde::Deserialize)]
    struct Text<'a>(#[serde(borrow)] Cow<'a, str>);

    Option::<Text<'de>>::deserialize(deserializer).map(|text| text.map(|Text(value)| value))
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SuggestedSourceLink<'a> {
    #[serde(with = "serde_field_attributes::integer_str")]
    pub count: usize,
    #[serde(borrow)]
    pub url: Cow<'a, str>,
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
    #[serde(borrow)]
    pub affiliation: Option<Affiliation<'a>>,
    #[serde(with = "crate::model::attributes::id_str")]
    pub id: u64,
    #[serde(borrow)]
    pub username: Cow<'a, str>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    #[serde(borrow)]
    pub description: Cow<'a, str>,
    #[serde(borrow)]
    pub location: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub url: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub profile_image_url: Cow<'a, str>,
    #[serde(borrow)]
    pub profile_banner_url: Option<Cow<'a, str>>,
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
    pub most_recent_tweet_id: Option<u64>,
    #[serde(with = "crate::model::attributes::optional_id_str", default)]
    pub pinned_tweet_id: Option<u64>,
    pub entities: Option<entity::UserEntities<'a>>,
    pub verified: bool,
    pub verified_type: Option<VerifiedType>,
    pub is_identity_verified: Option<bool>,
    pub protected: bool,
    pub public_metrics: UserPublicMetrics,
    pub withheld: Option<Withheld>,
    #[serde(borrow)]
    pub derived: Option<Derived<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VerifiedType {
    None,
    Blue,
    Business,
    Government,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Affiliation<'a> {
    #[serde(borrow)]
    pub badge_url: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub description: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub url: Option<Cow<'a, str>>,
    #[serde(with = "crate::model::attributes::optional_ids_str", default)]
    pub user_id: Option<Cow<'a, [u64]>>,
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
    pub cover_media: Option<Cow<'a, str>>,
    pub entities: Option<entity::ArticleEntities<'a>>,
    pub media_entities: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub plain_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub preview_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub title: Option<Cow<'a, str>>,
}

#[cfg(test)]
mod tests;

//! This data format appears for tweets in the Wayback Machine from around 9 December 2022 until into 2025.

use crate::model::attributes::{integer_str, integer_str_array_opt, integer_str_opt};
use crate::model::{
    country::Country,
    lang::Lang,
    media::{MediaType, MediaVariant},
    metrics::{MediaPublicMetrics, UserPublicMetrics},
};
use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

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
    pub errors: Option<Vec<TweetError<'a>>>,
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
//#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    #[serde(borrow)]
    pub article: Option<Article<'a>>,
    pub attachments: Option<Attachments>,
    #[serde(with = "integer_str")]
    pub id: u64,
    #[serde(with = "integer_str")]
    pub author_id: u64,
    pub context_annotations: Option<Vec<ContextAnnotation<'a>>>,
    #[serde(with = "integer_str")]
    pub conversation_id: u64,
    pub created_at: DateTime<Utc>,
    //pub edit_controls: Option<EditControls>,
    //#[serde(with = "id_str_array_optional")]
    //#[serde(default)]
    //pub edit_history_tweet_ids: Option<Vec<u64>>,
    pub lang: Lang,
    //pub entities: Option<TweetEntities<'a>>,
    //pub geo: Option<Geo<'a>>,
    //pub note_tweet: Option<NoteTweet<'a>>,
    pub possibly_sensitive: bool,
    //pub public_metrics: TweetPublicMetrics,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    pub reply_settings: ReplySettings,
    pub text: Cow<'a, str>,
    #[serde(with = "integer_str_opt")]
    #[serde(default)]
    pub in_reply_to_user_id: Option<u64>,
    //pub source: Option<TweetSource>,
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
        with = "integer_str_array_opt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_source_tweet_id: Option<Vec<u64>>,
    #[serde(
        with = "integer_str_array_opt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub poll_ids: Option<Vec<u64>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextAnnotation<'a> {
    #[serde(borrow)]
    pub domain: ContextDomain<'a>,
    pub entity: ContextEntity<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextDomain<'a> {
    #[serde(with = "integer_str")]
    pub id: u64,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextEntity<'a> {
    #[serde(with = "integer_str")]
    pub id: u64,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetIncludes<'a> {
    #[serde(borrow)]
    pub users: Vec<User<'a>>,
    pub tweets: Option<Vec<Tweet<'a>>>,
    pub media: Option<Vec<Media<'a>>>,
    pub polls: Option<Vec<Poll<'a>>>,
    pub places: Option<Vec<Place>>,
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Place {}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Media<'a> {
    #[serde(rename = "photo")]
    Photo {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        url: Cow<'a, str>,
        alt_text: Option<Cow<'a, str>>,
    },
    #[serde(rename = "video")]
    Video {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        variants: Vec<MediaVariant<'a>>,
        duration_ms: Option<usize>,
        preview_image_url: Cow<'a, str>,
    },
    #[serde(rename = "animated_gif")]
    AnimatedGif {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        variants: Vec<MediaVariant<'a>>,
        preview_image_url: Cow<'a, str>,
    },
}

impl bounded_static::IntoBoundedStatic for Media<'_> {
    type Static = Media<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Photo {
                metadata,
                url,
                alt_text,
            } => Media::Photo {
                metadata: metadata.into_static(),
                url: url.to_string().into(),
                alt_text: alt_text.map(|alt_text| alt_text.to_string().into()),
            },
            Self::Video {
                metadata,
                variants,
                duration_ms,
                preview_image_url,
            } => Media::Video {
                metadata: metadata.into_static(),
                variants: variants
                    .into_iter()
                    .map(bounded_static::IntoBoundedStatic::into_static)
                    .collect(),
                duration_ms,
                preview_image_url: preview_image_url.to_string().into(),
            },
            Self::AnimatedGif {
                metadata,
                variants,
                preview_image_url,
            } => Media::AnimatedGif {
                metadata: metadata.into_static(),
                variants: variants
                    .into_iter()
                    .map(bounded_static::IntoBoundedStatic::into_static)
                    .collect(),
                preview_image_url: preview_image_url.to_string().into(),
            },
        }
    }
}

impl bounded_static::ToBoundedStatic for Media<'_> {
    type Static = Media<'static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::Photo {
                metadata,
                url,
                alt_text,
            } => Media::Photo {
                metadata: metadata.to_static(),
                url: url.to_static(),
                alt_text: alt_text
                    .as_ref()
                    .map(bounded_static::ToBoundedStatic::to_static),
            },
            Self::Video {
                metadata,
                variants,
                duration_ms,
                preview_image_url,
            } => Media::Video {
                metadata: metadata.to_static(),
                variants: variants
                    .iter()
                    .map(bounded_static::ToBoundedStatic::to_static)
                    .collect(),
                duration_ms: *duration_ms,
                preview_image_url: preview_image_url.to_string().into(),
            },
            Self::AnimatedGif {
                metadata,
                variants,
                preview_image_url,
            } => Media::AnimatedGif {
                metadata: metadata.to_static(),
                variants: variants
                    .iter()
                    .map(bounded_static::ToBoundedStatic::to_static)
                    .collect(),
                preview_image_url: preview_image_url.to_string().into(),
            },
        }
    }
}

impl<'a> Media<'a> {
    #[must_use]
    pub const fn metadata(&self) -> &MediaMetadata<'a> {
        match self {
            Self::Photo { metadata, .. }
            | Self::Video { metadata, .. }
            | Self::AnimatedGif { metadata, .. } => metadata,
        }
    }

    #[must_use]
    pub const fn media_type(&self) -> MediaType {
        match self {
            Self::Photo { .. } => MediaType::Photo,
            Self::Video { .. } => MediaType::Video,
            Self::AnimatedGif { .. } => MediaType::AnimatedGif,
        }
    }

    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Photo { url, .. } => Some(url),
            Self::Video { .. } | Self::AnimatedGif { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaMetadata<'a> {
    pub media_key: Cow<'a, str>,
    pub public_metrics: Option<MediaPublicMetrics>,
    pub height: usize,
    pub width: usize,
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
//#[serde(deny_unknown_fields)]
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
    #[serde(with = "integer_str_opt")]
    #[serde(default)]
    pub pinned_tweet_id: Option<u64>,
    //pub entities: Option<UserEntities<'a>>,
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

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetError<'a> {
    pub resource_id: Cow<'a, str>,
    pub parameter: Cow<'a, str>,
    pub resource_type: TweetErrorResourceType,
    pub section: Option<TweetErrorSection>,
    pub title: Cow<'a, str>,
    pub value: Cow<'a, str>,
    pub detail: Cow<'a, str>,
    #[serde(rename = "type")]
    pub error_type: TweetErrorType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorResourceType {
    #[serde(rename = "tweet")]
    Tweet,
    #[serde(rename = "user")]
    User,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorSection {
    #[serde(rename = "includes")]
    Includes,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorType {
    #[serde(rename = "https://api.twitter.com/2/problems/not-authorized-for-resource")]
    NotAuthorizedForResource,
    #[serde(rename = "https://api.twitter.com/2/problems/resource-not-found")]
    ResourceNotFound,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_media_examples() {
        let lines = include_str!("../../../../examples/wxj/media.ndjson")
            .split("\n")
            .filter(|line| !line.is_empty());

        for (i, line) in lines.enumerate() {
            let result = serde_json::from_str::<super::Media<'_>>(line);

            if let Err(error) = &result {
                println!(
                    "Line {}: {line:?} is an invalid media object: {error}",
                    i + 1
                );
            }

            assert!(result.is_ok());
        }
    }
}

use crate::model::{graphql::unavailable::UserUnavailableReason, user::properties::VerifiedType};
use chrono::{DateTime, Utc};
use std::borrow::Cow;

pub mod about_account;
pub mod common;
pub mod community;
pub mod mention;
pub mod partial;
pub mod repr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserResult<'a> {
    Available(User<'a>),
    Unavailable {
        id: u64,
        reason: UserUnavailableReason,
    },
    Incomplete {
        id: u64,
    },
}

impl UserResult<'_> {
    #[must_use]
    pub const fn id(&self) -> u64 {
        match self {
            Self::Available(tweet) => tweet.id,
            Self::Unavailable { id, .. } => *id,
            Self::Incomplete { id } => *id,
        }
    }
}

impl bounded_static::IntoBoundedStatic for UserResult<'_> {
    type Static = UserResult<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Available(tweet) => Self::Static::Available(tweet.into_static()),
            Self::Unavailable { id, reason } => Self::Static::Unavailable { id, reason },
            Self::Incomplete { id } => Self::Static::Incomplete { id },
        }
    }
}

impl bounded_static::ToBoundedStatic for UserResult<'_> {
    type Static = UserResult<'static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::Available(tweet) => Self::Static::Available(tweet.to_static()),
            Self::Unavailable { id, reason } => Self::Static::Unavailable {
                id: *id,
                reason: *reason,
            },
            Self::Incomplete { id } => Self::Static::Incomplete { id: *id },
        }
    }
}

// TODO: Fill this in.
#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct User<'a> {
    pub id: u64,
    pub screen_name: Cow<'a, str>,
    /// Name may not be provided for accounts with `XX` withholding, for example.
    pub name: Option<Cow<'a, str>>,
    pub created_at: DateTime<Utc>,
    /// Name may not be provided for accounts with `XX` withholding, for example.
    pub profile_image_url: Option<Cow<'a, str>>,
    pub protected: Option<bool>,
    pub is_blue_verified: bool,
    pub verified: bool,
    pub verified_type: Option<VerifiedType>,
    pub super_follow_eligible: Option<bool>,
    pub subscribers_count: Option<usize>,
    pub creator_subscriptions_count: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Verification {
    pub verified: bool,
    pub verified_type: Option<VerifiedType>,
}

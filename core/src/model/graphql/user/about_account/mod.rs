use crate::model::{
    graphql::{affiliation::Affiliation, unavailable::UserUnavailableReason, user::Verification},
    user::properties::ProfileImageShape,
};
use bounded_static::IntoBoundedStatic;
use chrono::{DateTime, Utc};
use serde_field_attributes::{integer_str, optional_timestamp_millis_str};
use std::borrow::Cow;

pub mod location;
pub mod partial;
pub mod source;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserResult<'a> {
    Available(User<'a>),
    Unavailable {
        screen_name: Cow<'a, str>,
        reason: UserUnavailableReason,
    },
}

impl IntoBoundedStatic for UserResult<'_> {
    type Static = UserResult<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Available(user) => Self::Static::Available(user.into_static()),
            Self::Unavailable {
                screen_name,
                reason,
            } => Self::Static::Unavailable {
                screen_name: screen_name.into_static(),
                reason,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct User<'a> {
    pub id: u64,
    pub screen_name: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
    pub created_at: DateTime<Utc>,
    pub about_profile: Option<AboutProfile<'a>>,
    pub affiliation: Option<Affiliation<'a>>,
    pub identity_affiliation: Option<Affiliation<'a>>,
    pub protected: bool,
    pub is_blue_verified: bool,
    pub verification: Verification,
    pub verified_since: Option<DateTime<Utc>>,
    pub override_verified_year: Option<i32>,
    pub profile_image_url: Option<Cow<'a, str>>,
    pub profile_image_shape: Option<ProfileImageShape>,
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    bounded_static_derive_more::ToStatic,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(deny_unknown_fields)]
pub struct AboutProfile<'a> {
    pub account_based_in: Option<location::Location>,
    pub learn_more_url: LearnMoreUrl,
    pub location_accurate: Option<bool>,
    pub source: Option<source::Source>,
    pub username_changes: UsernameChanges,
    pub affiliate_username: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum LearnMoreUrl {
    #[serde(
        rename = "https://help.twitter.com/managing-your-account/about-twitter-verified-accounts"
    )]
    AboutTwitterVerifiedAccounts,
    #[serde(rename = "https://help.twitter.com/rules-and-policies/profile-labels")]
    ProfileLabels,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UsernameChanges {
    #[serde(with = "integer_str")]
    pub count: usize,
    #[serde(
        rename = "last_changed_at_msec",
        with = "optional_timestamp_millis_str",
        default
    )]
    pub last_changed_at: Option<DateTime<Utc>>,
}

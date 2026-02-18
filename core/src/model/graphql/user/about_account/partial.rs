use crate::model::{
    attributes::text_timestamp,
    graphql::{affiliation::AffiliationResult, unavailable::UserUnavailableReason},
    user::properties::ProfileImageShape,
};
use chrono::{DateTime, Utc};
use serde_field_attributes::{integer_str, optional_timestamp_millis_str};
use std::borrow::Cow;

#[derive(serde::Deserialize)]
#[serde(tag = "__typename")]
pub enum UserResult<'a> {
    User {
        #[serde(flatten, borrow)]
        user: User<'a>,
    },
    UserUnavailable {
        unavailable_reason: UserUnavailableReason,
    },
}

impl<'a> UserResult<'a> {
    pub fn complete(self, screen_name: Cow<'a, str>) -> super::UserResult<'a> {
        match self {
            Self::User { user } => super::UserResult::Available(super::User {
                id: user.rest_id,
                screen_name: user.core.screen_name,
                name: user.core.name,
                created_at: user.core.created_at,
                about_profile: user.about_profile,
                affiliation: user.affiliates_highlighted_label.into(),
                identity_affiliation: user.identity_profile_labels_highlighted_label.into(),
                protected: user.privacy.protected,
                is_blue_verified: user.is_blue_verified,
                verification: user.verification,
                verified_since: user
                    .verification_info
                    .reason
                    .and_then(|reason| reason.verified_since_msec),
                profile_image_url: user.avatar.image_url,
                profile_image_shape: user.profile_image_shape,
            }),
            Self::UserUnavailable { unavailable_reason } => super::UserResult::Unavailable {
                screen_name,
                reason: unavailable_reason,
            },
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    about_profile: Option<super::AboutProfile>,
    #[serde(borrow)]
    affiliates_highlighted_label: AffiliationResult<'a>,
    avatar: Avatar<'a>,
    core: Core<'a>,
    #[serde(rename = "id")]
    _id: Cow<'a, str>,
    identity_profile_labels_highlighted_label: AffiliationResult<'a>,
    is_blue_verified: bool,
    privacy: Privacy,
    profile_image_shape: ProfileImageShape,
    #[serde(with = "integer_str")]
    rest_id: u64,
    verification: super::Verification,
    verification_info: VerificationInfo<'a>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Avatar<'a> {
    image_url: Option<Cow<'a, str>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Core<'a> {
    #[serde(with = "text_timestamp")]
    created_at: DateTime<Utc>,
    name: Option<Cow<'a, str>>,
    screen_name: Cow<'a, str>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Privacy {
    protected: bool,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationInfo<'a> {
    #[serde(rename = "id")]
    _id: Cow<'a, str>,
    reason: Option<VerificationReason>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct VerificationReason {
    #[serde(with = "optional_timestamp_millis_str", default)]
    verified_since_msec: Option<DateTime<Utc>>,
}

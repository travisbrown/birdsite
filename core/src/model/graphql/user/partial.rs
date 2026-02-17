use crate::model::{
    attributes::{optional_text_timestamp, text_timestamp},
    graphql::{unavailable::UserUnavailableReason, user::Verification},
    user::properties::VerifiedType,
};
use chrono::{DateTime, Utc};
use serde_field_attributes::integer_str;
use std::borrow::Cow;

#[derive(serde::Deserialize)]
#[serde(tag = "__typename")]
pub enum UserResult<'a> {
    User {
        #[serde(flatten)]
        user: User<'a>,
    },
    UserUnavailable {
        reason: UserUnavailableReason,
    },
}

impl<'a> UserResult<'a> {
    #[must_use]
    pub fn complete(self, id: u64) -> super::UserResult<'a> {
        match self {
            Self::User { user } => user.into_user().map_or_else(
                || super::UserResult::Incomplete { id },
                |user| super::UserResult::Available(user),
            ),
            Self::UserUnavailable { reason } => super::UserResult::Unavailable { id, reason },
        }
    }

    /// Note that an empty error result indicates an incomplete response.
    pub fn into_result(self) -> Result<super::User<'a>, Option<UserUnavailableReason>> {
        match self {
            Self::User { user } => user.into_user().map_or(Err(None), Ok),
            Self::UserUnavailable { reason } => Err(Some(reason)),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
//#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "integer_str")]
    rest_id: u64,
    legacy: Option<Legacy<'a>>,
    core: Option<Core<'a>>,
    avatar: Option<Avatar<'a>>,
    protected: Option<bool>,
    // Should only be missing in the case of an error.
    is_blue_verified: Option<bool>,
    verification: Option<Verification>,
    super_follow_eligible: Option<bool>,
    subscribers_count: Option<usize>,
    creator_subscriptions_count: Option<usize>,
}

impl<'a> User<'a> {
    fn into_user(self) -> Option<super::User<'a>> {
        let created_at = self
            .legacy
            .as_ref()
            .and_then(|legacy| legacy.created_at)
            .or_else(|| self.core.as_ref().map(|core| core.created_at))?;

        let verified = self.legacy.as_ref().and_then(|legacy| {
            legacy
                .verified
                .or_else(|| self.verification.map(|verification| verification.verified))
        })?;

        let verified_type = self.legacy.as_ref().and_then(|legacy| {
            legacy.verified_type.or_else(|| {
                self.verification
                    .and_then(|verification| verification.verified_type)
            })
        });

        let screen_name = self
            .legacy
            .as_ref()
            .and_then(|legacy| legacy.screen_name.clone())
            .or_else(|| self.core.as_ref().map(|core| core.screen_name.clone()))?;

        let name = self
            .legacy
            .as_ref()
            .and_then(|legacy| legacy.name.clone())
            .or_else(|| self.core.as_ref().and_then(|core| core.name.clone()));

        let profile_image_url = self
            .legacy
            .as_ref()
            .and_then(|legacy| legacy.profile_image_url.clone())
            .or_else(|| {
                self.avatar
                    .as_ref()
                    .and_then(|avatar| avatar.image_url.clone())
            });

        Some(super::User {
            id: self.rest_id,
            screen_name,
            name,
            created_at,
            profile_image_url,
            protected: self.protected,
            is_blue_verified: self.is_blue_verified?,
            verified,
            verified_type,
            super_follow_eligible: self.super_follow_eligible,
            subscribers_count: self.subscribers_count,
            creator_subscriptions_count: self.creator_subscriptions_count,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
//#[serde(deny_unknown_fields)]
struct Legacy<'a> {
    pub screen_name: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    #[serde(with = "optional_text_timestamp", default)]
    created_at: Option<DateTime<Utc>>,
    #[serde(rename = "profile_image_url_https")]
    profile_image_url: Option<Cow<'a, str>>,
    verified: Option<bool>,
    verified_type: Option<VerifiedType>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Core<'a> {
    pub screen_name: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
    #[serde(with = "text_timestamp")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Avatar<'a> {
    image_url: Option<Cow<'a, str>>,
}

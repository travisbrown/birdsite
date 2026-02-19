use crate::model::graphql::unavailable::UserUnavailableReason;
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

    pub fn into_result(self) -> Result<super::User<'a>, Option<UserUnavailableReason>> {
        match self {
            Self::User { user } => user.into_user().map_or(Err(None), Ok),
            Self::UserUnavailable { reason } => Err(Some(reason)),
        }
    }
}

#[derive(serde::Deserialize)]
//#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "integer_str")]
    pub rest_id: u64,
    legacy: Option<Legacy<'a>>,
    pub super_follow_eligible: Option<bool>,
    pub subscribers_count: Option<usize>,
    pub creator_subscriptions_count: Option<usize>,
}

impl<'a> User<'a> {
    fn into_user(self) -> Option<super::User<'a>> {
        self.legacy.map(|legacy| super::User {
            id: self.rest_id,
            screen_name: legacy.screen_name,
            name: legacy.name,
            super_follow_eligible: self.super_follow_eligible,
            subscribers_count: self.subscribers_count,
            creator_subscriptions_count: self.creator_subscriptions_count,
        })
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
//#[serde(deny_unknown_fields)]
struct Legacy<'a> {
    pub screen_name: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

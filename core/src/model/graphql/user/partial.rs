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
    pub fn complete(self, id: u64) -> super::UserResult<'a> {
        match self {
            Self::User { user } => user.legacy.map_or_else(
                || super::UserResult::Incomplete { id },
                |legacy| {
                    super::UserResult::Available(super::User {
                        id: user.rest_id,
                        screen_name: legacy.screen_name,
                        name: legacy.name,
                    })
                },
            ),
            Self::UserUnavailable { reason } => super::UserResult::Unavailable { id, reason },
        }
    }
}

#[derive(serde::Deserialize)]
//#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "integer_str")]
    pub rest_id: u64,
    //#[serde(borrow)]
    legacy: Option<Legacy<'a>>,
}

#[derive(Clone, Debug, serde::Deserialize)]
//#[serde(deny_unknown_fields)]
struct Legacy<'a> {
    pub screen_name: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

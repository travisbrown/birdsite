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
        reason: birdsite::model::graphql::unavailable::UserUnavailableReason,
    },
}

impl<'a> UserResult<'a> {
    pub fn into_user_result(
        self,
        id: u64,
    ) -> Option<birdsite::model::graphql::user::UserResult<'a>> {
        match self {
            Self::User { user } => user.legacy.map(|legacy| {
                birdsite::model::graphql::user::UserResult::Available(
                    birdsite::model::graphql::user::User {
                        id: user.rest_id,
                        screen_name: legacy.screen_name,
                        name: legacy.name,
                    },
                )
            }),
            Self::UserUnavailable { reason } => {
                Some(birdsite::model::graphql::user::UserResult::Unavailable { id, reason })
            }
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

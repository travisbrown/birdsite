use serde_field_attributes::integer_str;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "__typename")]
pub enum UserResult<'a, U> {
    User {
        #[serde(flatten)]
        user: U,
    },
    UserUnavailable {
        reason: crate::model::graphql::unavailable::UserUnavailableReason,
        /// This fields seems to have been added as mandatory around 16 September 2023.
        #[serde(borrow)]
        message: Option<Cow<'a, str>>,
        unavailable_message: Option<crate::model::graphql::text::Text<'a>>,
    },
}

impl<U: bounded_static::IntoBoundedStatic> bounded_static::IntoBoundedStatic for UserResult<'_, U> {
    type Static = UserResult<'static, U::Static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::User { user } => Self::Static::User {
                user: user.into_static(),
            },
            Self::UserUnavailable {
                reason,
                message,
                unavailable_message,
            } => Self::Static::UserUnavailable {
                reason,
                message: message.into_static(),
                unavailable_message: unavailable_message.into_static(),
            },
        }
    }
}

impl<U: bounded_static::ToBoundedStatic> bounded_static::ToBoundedStatic for UserResult<'_, U> {
    type Static = UserResult<'static, U::Static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::User { user } => Self::Static::User {
                user: user.to_static(),
            },
            Self::UserUnavailable {
                reason,
                message,
                unavailable_message,
            } => Self::Static::UserUnavailable {
                reason: *reason,
                message: message.to_static(),
                unavailable_message: unavailable_message.to_static(),
            },
        }
    }
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
//#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "integer_str")]
    pub rest_id: u64,
    pub legacy: Legacy<'a>,
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
//#[serde(deny_unknown_fields)]
pub struct Legacy<'a> {
    pub screen_name: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScreenNameUserResults<'a> {
    #[serde(borrow)]
    pub result: Option<ScreenNameUserResult<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "__typename", deny_unknown_fields)]
pub enum ScreenNameUserResult<'a> {
    User {
        /// Seems to have switched to `core` around 22 May 2025.
        #[serde(alias = "core", borrow)]
        legacy: ScreenNameUserResultLegacy<'a>,
    },
    UserUnavailable {},
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScreenNameUserResultLegacy<'a> {
    pub screen_name: &'a str,
}

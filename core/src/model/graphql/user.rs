use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "__typename", deny_unknown_fields)]
pub enum UserResult<'a, U> {
    User {
        #[serde(flatten)]
        user: U,
    },
    UserUnavailable {
        reason: crate::model::graphql::unavailable::UserUnavailableReason,
        /// This fields seems to have been added as mandatory around 16 September 2023.
        message: Option<&'a str>,
        unavailable_message: Option<crate::model::graphql::text::Text<'a>>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct User<'a> {
    #[serde(with = "crate::model::attributes::integer_str")]
    pub rest_id: u64,
    pub legacy: Legacy<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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

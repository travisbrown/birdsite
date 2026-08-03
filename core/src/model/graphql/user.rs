use std::borrow::Cow;

// `deny_unknown_fields` cannot be used here: it is unsupported in combination with the flattened
// variant payload, and rejects every field of any record (serde matches keys against the named
// variant fields only, so even the flattened variant's own fields count as unknown).
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
        message: Option<&'a str>,
        unavailable_message: Option<crate::model::graphql::text::Text<'a>>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "crate::model::attributes::id_str")]
    pub rest_id: u64,
    #[serde(borrow)]
    pub legacy: Legacy<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//#[serde(deny_unknown_fields)]
pub struct Legacy<'a> {
    #[serde(borrow)]
    pub screen_name: Cow<'a, str>,
    #[serde(borrow)]
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

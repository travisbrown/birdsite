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
        message: &'a str,
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

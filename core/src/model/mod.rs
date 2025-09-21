use bounded_static_derive_more::ToStatic;
use std::borrow::Cow;

pub mod attributes;
pub mod color;
pub mod country;
pub mod entity;
pub mod graphql;
pub mod lang;
pub mod media;
pub mod metrics;
pub mod place;
pub mod properties;
pub mod snowflake;
pub mod time_zone;
pub mod timestamp;
pub mod url;
pub mod user;
pub mod wxj;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct KeyValuePair<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

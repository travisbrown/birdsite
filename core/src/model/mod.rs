use std::borrow::Cow;

pub mod attributes;
pub mod color;
pub mod country;
pub mod entity;
pub mod lang;
pub mod place;
pub mod properties;
pub mod snowflake;
pub mod time_zone;
pub mod timestamp;
pub mod url;
pub mod user;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct KeyValuePair<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> KeyValuePair<'a> {
    pub fn into_owned(self) -> KeyValuePair<'static> {
        KeyValuePair {
            key: self.key.into_owned().into(),
            value: self.value.into_owned().into(),
        }
    }
}

use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request<'a, V> {
    pub name: crate::request::name::RequestName,
    pub version: Option<Cow<'a, str>>,
    pub timestamp: DateTime<Utc>,
    pub variables: V,
}

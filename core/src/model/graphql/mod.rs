pub mod ads;
pub mod birdwatch;
pub mod community;
pub mod image;
pub mod properties;
pub mod shapes;
pub mod text;
pub mod timeline;
pub mod trends;
pub mod unavailable;
pub mod user;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResultWrapper<A> {
    pub result: Option<A>,
}

/// Wrapper for object fields that may appear as completely empty objects.
///
/// Some GraphQL responses replace an object with a `{}` placeholder (observed for
/// `affiliates_highlighted_label` generally, and for most fields of "ghost" user records in
/// `UsersByRestIds` responses from April 2026).
///
/// The `deny_unknown_fields` attribute ensures that `Empty` matches only `{}`, so unexpected
/// fields in the wrapped value still fail deserialization instead of being silently dropped.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum EmptyOr<T> {
    Value(T),
    Empty {},
}

impl<T> From<EmptyOr<T>> for Option<T> {
    fn from(value: EmptyOr<T>) -> Self {
        match value {
            EmptyOr::Value(value) => Some(value),
            EmptyOr::Empty {} => None,
        }
    }
}

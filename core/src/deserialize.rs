use std::borrow::Cow;

use archivindex_serde::borrowable_str;
use serde::de::{Deserialize, Deserializer};

/// Borrow strings in an optional sequence, whether stored in a vector or a Cow slice.
pub fn optional_strings<'de: 'a, 'a, D: Deserializer<'de>, T: From<Vec<Cow<'a, str>>>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    #[derive(serde::Deserialize)]
    struct Strings<'a>(#[serde(borrow, with = "borrowable_str::seq")] Vec<Cow<'a, str>>);

    Option::<Strings<'a>>::deserialize(deserializer)
        .map(|strings| strings.map(|Strings(values)| values.into()))
}

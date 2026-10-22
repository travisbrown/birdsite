use std::borrow::Cow;

use archivindex_serde::borrowable_str;
use bounded_static_derive_more::ToStatic;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextAnnotation<'a> {
    #[serde(borrow)]
    pub domain: ContextDomain<'a>,
    pub entity: ContextEntity<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextDomain<'a> {
    #[serde(with = "crate::model::attributes::id_str")]
    pub id: u64,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(default, with = "borrowable_str::option")]
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextEntity<'a> {
    #[serde(with = "crate::model::attributes::id_str")]
    pub id: u64,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(default, with = "borrowable_str::option")]
    pub description: Option<Cow<'a, str>>,
}

use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrendMetadata<'a> {
    pub domain_context: Option<Cow<'a, str>>,
    pub meta_description: Option<Cow<'a, str>>,
    pub url: Option<crate::model::entity::Url<'a>>,
}

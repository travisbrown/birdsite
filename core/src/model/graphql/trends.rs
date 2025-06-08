use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrendMetadata<'a> {
    #[serde(borrow)]
    pub domain_context: Option<Cow<'a, str>>,
    pub meta_description: Option<Cow<'a, str>>,
    pub url: Option<TrendUrl<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrendUrl<'a> {
    #[serde(borrow, flatten)]
    pub url: crate::model::url::Url<'a>,
    #[serde(rename = "urtEndpointOptions")]
    pub urt_endpoint_options: Option<crate::model::url::UrtEndpointOptions<'a>>,
}

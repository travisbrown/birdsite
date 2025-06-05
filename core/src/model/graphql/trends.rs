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
    #[serde(rename = "urlType")]
    pub url_type: crate::model::entity::UrlType,
    #[serde(borrow)]
    pub url: Cow<'a, str>,
    #[serde(rename = "urtEndpointOptions")]
    pub urt_endpoint_options: Option<UrtEndpointOptions<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UrtEndpointOptions<'a> {
    #[serde(rename = "requestParams", borrow)]
    pub request_params: Vec<super::KeyValuePair<'a>>,
}

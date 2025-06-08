use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UrlType {
    ExternalUrl,
    DeepLink,
    UrtEndpoint,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    #[serde(rename = "urlType")]
    pub url_type: UrlType,
    pub url: Cow<'a, str>,
}

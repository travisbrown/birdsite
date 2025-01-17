use super::id_str_optional;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UrlDetails<'a> {
    pub expanded_url: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Urls<'a> {
    #[serde(borrow)]
    pub urls: Vec<UrlDetails<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptionEntities<'a> {
    #[serde(borrow)]
    pub urls: Option<Vec<UrlDetails<'a>>>,
    #[serde(borrow)]
    pub mentions: Option<Vec<Mention<'a>>>,
    #[serde(borrow)]
    pub hashtags: Option<Vec<Hashtag<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntities<'a> {
    #[serde(borrow)]
    pub description: Option<DescriptionEntities<'a>>,
    #[serde(borrow)]
    pub url: Option<Urls<'a>>,
}

pub struct ProfileUrls<'a> {
    pub url: Option<Cow<'a, str>>,
    pub description_urls: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TweetEntities<'a> {
    pub annotations: Option<Vec<Annotation<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Annotation<'a> {
    pub normalized_text: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mention<'a> {
    pub start: usize,
    pub end: usize,
    pub username: Cow<'a, str>,
    #[serde(with = "id_str_optional")]
    #[serde(default)]
    pub id: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hashtag<'a> {
    pub start: usize,
    pub end: usize,
    pub tag: Cow<'a, str>,
}

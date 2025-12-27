use bounded_static_derive_more::ToStatic;
use serde_field_attributes::{optional_integer_str, range};
use std::borrow::Cow;
use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetEntities<'a> {
    pub hashtags: Vec<Hashtag<'a>>,
    pub urls: Vec<Url<'a>>,
    pub user_mentions: Vec<Mention<'a>>,
    pub symbols: Vec<Symbol>,
    #[serde(borrow)]
    pub media: Option<Vec<super::media::Media<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hashtag<'a> {
    pub text: Cow<'a, str>,
    #[serde(with = "range")]
    pub indices: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    pub expanded_url: Option<Cow<'a, str>>,
    pub url: Cow<'a, str>,
    pub display_url: Option<Cow<'a, str>>,
    #[serde(with = "range")]
    pub indices: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mention<'a> {
    pub id: Option<u64>,
    #[serde(with = "optional_integer_str")]
    id_str: Option<u64>,
    pub screen_name: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
    #[serde(with = "range")]
    pub indices: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Symbol {
    pub text: crate::model::cashtag::Cashtag,
    #[serde(with = "range")]
    pub indices: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweetExtendedEntities<'a> {
    #[serde(borrow)]
    pub media: Vec<super::media::Media<'a>>,
}

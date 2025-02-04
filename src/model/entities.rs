use super::{
    id_str,
    media::{MediaSizes, MediaSourceMetadata, MediaType, VideoInfo},
    probability, PossibleId,
};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UrlDetails<'a> {
    pub url: Cow<'a, str>,
    pub display_url: Option<Cow<'a, str>>,
    pub expanded_url: Option<Cow<'a, str>>,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
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
    pub hashtags: Option<Vec<Hashtag<'a>>>,
    pub cashtags: Option<Vec<Cashtag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntities<'a> {
    #[serde(borrow)]
    pub description: Option<DescriptionEntities<'a>>,
    #[serde(borrow)]
    pub url: Option<Urls<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetEntities<'a> {
    pub annotations: Option<Vec<Annotation<'a>>>,
    #[serde(borrow)]
    pub mentions: Option<Vec<Mention<'a>>>,
    pub urls: Option<Vec<Url<'a>>>,
    pub hashtags: Option<Vec<Hashtag<'a>>>,
    pub cashtags: Option<Vec<Cashtag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweetEntities<'a> {
    pub hashtags: Vec<ExtendedTweetHashtag<'a>>,
    pub urls: Vec<ExtendedTweetUrl<'a>>,
    pub user_mentions: Vec<ExtendedTweetMention<'a>>,
    pub symbols: Vec<ExtendedTweetSymbol>,
    #[serde(borrow)]
    pub media: Option<Vec<Media<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweetExtendedEntities<'a> {
    #[serde(borrow)]
    pub media: Vec<Media<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Media<'a> {
    pub id: u64,
    #[serde(with = "id_str")]
    id_str: u64,
    pub indices: (usize, usize),
    #[serde(borrow)]
    pub additional_media_info: Option<AdditionalMediaInfo<'a>>,
    #[serde(rename = "media_url")]
    media_url_http: Cow<'a, str>,
    #[serde(rename = "media_url_https")]
    pub media_url: Cow<'a, str>,
    pub url: Cow<'a, str>,
    pub display_url: Cow<'a, str>,
    pub expanded_url: Cow<'a, str>,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub video_info: Option<VideoInfo<'a>>,
    pub sizes: MediaSizes,
    #[serde(flatten)]
    pub source_metadata: Option<MediaSourceMetadata>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdditionalMediaInfo<'a> {
    pub title: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub embeddable: Option<bool>,
    pub monetizable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Annotation<'a> {
    #[serde(rename = "type")]
    pub annotation_type: AnnotationType,
    // There is currently one known case where this is -1 (1814274947960709489).
    pub start: isize,
    // There is currently one known case where this is -2 (1833402059414245783).
    pub end: isize,
    #[serde(with = "probability")]
    pub probability: num_rational::Ratio<u64>,
    pub normalized_text: Cow<'a, str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AnnotationType {
    Organization,
    Person,
    Place,
    Product,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mention<'a> {
    pub start: usize,
    pub end: usize,
    pub username: &'a str,
    pub id: Option<PossibleId>,
}

// Older form.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweetMention<'a> {
    pub id: u64,
    #[serde(with = "id_str")]
    id_str: u64,
    pub screen_name: &'a str,
    pub name: Cow<'a, str>,
    pub indices: (usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    pub start: usize,
    pub end: usize,
    pub title: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub url: &'a str,
    pub expanded_url: &'a str,
    pub display_url: Cow<'a, str>,
    pub media_key: Option<&'a str>,
    // TODO: Use a proper status code representation here.
    pub status: Option<usize>,
    pub unwound_url: Option<&'a str>,
    pub images: Option<Vec<UrlImage<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweetUrl<'a> {
    pub url: Cow<'a, str>,
    pub expanded_url: Cow<'a, str>,
    pub display_url: Cow<'a, str>,
    pub indices: (usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UrlImage<'a> {
    pub url: &'a str,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hashtag<'a> {
    pub start: usize,
    pub end: usize,
    pub tag: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweetHashtag<'a> {
    pub text: Cow<'a, str>,
    pub indices: (usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cashtag {
    pub start: usize,
    pub end: usize,
    pub tag: super::cashtag::Cashtag,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweetSymbol {
    pub text: super::cashtag::Cashtag,
    pub indices: (usize, usize),
}

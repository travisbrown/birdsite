use bounded_static_derive_more::ToStatic;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ContentType {
    #[serde(rename = "application/x-mpegURL")]
    MpegUrl,
    #[serde(rename = "video/mp4")]
    Mp4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MediaType {
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "animated_gif")]
    AnimatedGif,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaVariant<'a> {
    pub url: Cow<'a, str>,
    pub content_type: ContentType,
    // Older snapshots (v1) tend to use the unhyphenated form.
    #[serde(alias = "bitrate")]
    pub bit_rate: Option<usize>,
}

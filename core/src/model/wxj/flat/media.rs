use crate::model::media::{AspectRatio, MediaType, MediaVariant};
use bounded_static_derive_more::ToStatic;
use serde_field_attributes::range;
use std::borrow::Cow;
use std::ops::Range;

// Field names intentionally mirror the Twitter API schema.
#[allow(clippy::struct_field_names)]
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Media<'a> {
    pub id: u64,
    #[serde(with = "crate::model::attributes::id_str")]
    id_str: u64,
    #[serde(with = "range")]
    pub indices: Range<usize>,
    #[serde(borrow)]
    pub additional_media_info: Option<AdditionalMediaInfo<'a>>,
    #[serde(rename = "media_url", borrow)]
    media_url_http: Cow<'a, str>,
    #[serde(rename = "media_url_https", borrow)]
    pub media_url: Cow<'a, str>,
    #[serde(borrow)]
    pub url: Cow<'a, str>,
    #[serde(borrow)]
    pub display_url: Cow<'a, str>,
    #[serde(borrow)]
    pub expanded_url: Cow<'a, str>,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub video_info: Option<VideoInfo<'a>>,
    pub sizes: MediaSizes,
    #[serde(flatten)]
    pub source_metadata: Option<MediaSourceMetadata>,
    #[serde(borrow)]
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdditionalMediaInfo<'a> {
    #[serde(borrow)]
    pub title: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub description: Option<Cow<'a, str>>,
    pub embeddable: Option<bool>,
    pub monetizable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoInfo<'a> {
    pub aspect_ratio: AspectRatio,
    pub duration_millis: Option<usize>,
    #[serde(borrow)]
    pub variants: Vec<MediaVariant<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaSizes {
    pub thumb: MediaSize,
    pub small: MediaSize,
    pub medium: MediaSize,
    pub large: MediaSize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaSize {
    pub w: usize,
    pub h: usize,
    pub resize: Resize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Resize {
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "crop")]
    Crop,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaSourceMetadata {
    #[serde(rename = "source_status_id")]
    pub status_id: u64,
    #[serde(with = "crate::model::attributes::id_str")]
    source_status_id_str: u64,
    #[serde(rename = "source_user_id")]
    pub user_id: u64,
    #[serde(with = "crate::model::attributes::id_str")]
    source_user_id_str: u64,
}

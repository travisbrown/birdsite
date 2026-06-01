use bounded_static_derive_more::ToStatic;
use serde_field_attributes::{integer_str, range};
use std::borrow::Cow;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Resize {
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "crop")]
    Crop,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ContentType {
    #[serde(rename = "application/dash+xml")]
    DashXml,
    #[serde(rename = "application/x-mpegURL")]
    MpegUrl,
    #[serde(rename = "video/mp4")]
    Mp4,
    #[serde(rename = "video/webm")]
    Webm,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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

/// A media attachment entity from the v1.1 Twitter API.
///
/// Fields introduced after 2011 (`video_info`, `additional_media_info`, `source_metadata`,
/// `description`) are optional and absent in older streaming archives.
#[allow(clippy::struct_field_names)]
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Media<'a> {
    /// Numeric identifier of the media item.
    pub id: u64,
    /// Wire-format string form of `id`; validated to match during deserialization.
    #[serde(with = "integer_str")]
    id_str: u64,
    /// `[start, end)` byte offsets of the media URL in the tweet text.
    #[serde(with = "range")]
    pub indices: Range<usize>,
    /// Additional metadata for embeddable media (title, description, monetization).
    #[serde(borrow)]
    pub additional_media_info: Option<AdditionalMediaInfo<'a>>,
    /// HTTP URL of the media file (kept private; prefer `media_url`).
    #[serde(rename = "media_url")]
    media_url_http: Cow<'a, str>,
    /// HTTPS URL of the media file.
    #[serde(rename = "media_url_https")]
    pub media_url: Cow<'a, str>,
    /// Shortened `t.co` URL used in the tweet text.
    pub url: Cow<'a, str>,
    /// Human-readable abbreviated form of the expanded URL.
    pub display_url: Cow<'a, str>,
    /// Fully-expanded URL to the media page.
    pub expanded_url: Cow<'a, str>,
    /// Media classification.
    #[serde(rename = "type")]
    pub media_type: MediaType,
    /// Video stream variants; present only for `video` and `animated_gif` media.
    pub video_info: Option<VideoInfo<'a>>,
    /// Available thumbnail size variants.
    pub sizes: MediaSizes,
    /// Original tweet and user identifiers when this media was shared from another tweet.
    #[serde(flatten)]
    pub source_metadata: Option<MediaSourceMetadata>,
    /// Alt-text description of the media.
    pub description: Option<Cow<'a, str>>,
}

/// Embeddable-media metadata attached to certain video media items.
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdditionalMediaInfo<'a> {
    pub title: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub embeddable: Option<bool>,
    pub monetizable: bool,
}

/// Video stream information attached to `video` and `animated_gif` media.
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoInfo<'a> {
    /// Width-to-height ratio encoded as `[width, height]`.
    #[serde(with = "range")]
    pub aspect_ratio: Range<usize>,
    /// Total duration of the video in milliseconds.
    pub duration_millis: Option<usize>,
    /// Available stream variants (codec, bitrate, URL).
    pub variants: Vec<MediaVariant<'a>>,
}

/// Source-tweet provenance for media reposted from another tweet.
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[serde(deny_unknown_fields, into = "internal::MediaSourceMetadata")]
pub struct MediaSourceMetadata {
    /// Numeric identifier of the tweet this media originally appeared in.
    pub status_id: u64,
    /// Numeric identifier of the user who originally posted the media.
    pub user_id: u64,
}

impl<'de> serde::de::Deserialize<'de> for MediaSourceMetadata {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = internal::MediaSourceMetadata::deserialize(deserializer)?;

        internal.validate()
    }
}

mod internal {
    use serde::de::Unexpected;
    use serde_field_attributes::integer_str;

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct MediaSourceMetadata {
        source_status_id: u64,
        #[serde(with = "integer_str")]
        source_status_id_str: u64,
        source_user_id: u64,
        #[serde(with = "integer_str")]
        source_user_id_str: u64,
    }

    impl MediaSourceMetadata {
        pub fn validate<E: serde::de::Error>(self) -> Result<super::MediaSourceMetadata, E> {
            if self.source_status_id == self.source_status_id_str {
                if self.source_user_id == self.source_user_id_str {
                    Ok(super::MediaSourceMetadata {
                        status_id: self.source_status_id,
                        user_id: self.source_user_id,
                    })
                } else {
                    Err(E::invalid_value(
                        Unexpected::Unsigned(self.source_user_id),
                        &self.source_user_id_str.to_string().as_str(),
                    ))
                }
            } else {
                Err(E::invalid_value(
                    Unexpected::Unsigned(self.source_status_id),
                    &self.source_status_id_str.to_string().as_str(),
                ))
            }
        }
    }

    impl From<super::MediaSourceMetadata> for MediaSourceMetadata {
        fn from(value: super::MediaSourceMetadata) -> Self {
            Self {
                source_status_id: value.status_id,
                source_status_id_str: value.status_id,
                source_user_id: value.user_id,
                source_user_id_str: value.user_id,
            }
        }
    }
}

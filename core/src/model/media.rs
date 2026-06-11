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
    source_metadata: internal::MaybeMediaSourceMetadata,
    /// Alt-text description of the media.
    pub description: Option<Cow<'a, str>>,
}

impl Media<'_> {
    /// Original tweet and user identifiers when this media was shared from another tweet.
    #[must_use]
    pub const fn source_metadata(&self) -> Option<&MediaSourceMetadata> {
        self.source_metadata.0.as_ref()
    }
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MediaSourceMetadata {
    /// Numeric identifier of the tweet this media originally appeared in.
    pub status_id: u64,
    /// Numeric identifier of the user who originally posted the media; absent in older (2015-era)
    /// archives, which carry only the status pair.
    pub user_id: Option<u64>,
}

mod internal {
    use serde::de::Unexpected;
    use serde_field_attributes::optional_integer_str;

    /// Validating wrapper for the flattened `source_*` wire fields.
    ///
    /// A derived `Option<MediaSourceMetadata>` flatten would treat a partial set of keys as
    /// `None`, silently dropping the present values; this wrapper reads the four fields as
    /// individually optional and requires each `id`/`id_str` pair to be complete and consistent.
    /// The status pair may appear without the user pair (2015-era archives), but not vice versa.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct MaybeMediaSourceMetadata(pub(super) Option<super::MediaSourceMetadata>);

    #[allow(clippy::struct_field_names)]
    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    struct Fields {
        #[serde(skip_serializing_if = "Option::is_none")]
        source_status_id: Option<u64>,
        #[serde(
            with = "optional_integer_str",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        source_status_id_str: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        source_user_id: Option<u64>,
        #[serde(
            with = "optional_integer_str",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        source_user_id_str: Option<u64>,
    }

    /// Collapses an optional `id`/`id_str` pair, requiring both-or-neither and equal values.
    fn validate_id_pair<E: serde::de::Error>(
        id: Option<u64>,
        id_str: Option<u64>,
        name: &str,
    ) -> Result<Option<u64>, E> {
        match (id, id_str) {
            (None, None) => Ok(None),
            (Some(id), Some(from_str)) if id == from_str => Ok(Some(id)),
            (Some(id), Some(from_str)) => Err(E::invalid_value(
                Unexpected::Unsigned(id),
                &from_str.to_string().as_str(),
            )),
            _ => Err(E::custom(format!(
                "expected both of the {name} media fields or neither"
            ))),
        }
    }

    impl<'de> serde::de::Deserialize<'de> for MaybeMediaSourceMetadata {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let fields = Fields::deserialize(deserializer)?;

            let status_id = validate_id_pair(
                fields.source_status_id,
                fields.source_status_id_str,
                "source_status_id",
            )?;
            let user_id = validate_id_pair(
                fields.source_user_id,
                fields.source_user_id_str,
                "source_user_id",
            )?;

            match (status_id, user_id) {
                (None, None) => Ok(Self(None)),
                (Some(status_id), user_id) => Ok(Self(Some(super::MediaSourceMetadata {
                    status_id,
                    user_id,
                }))),
                (None, Some(_)) => Err(serde::de::Error::custom(
                    "expected source_status_id media fields alongside source_user_id",
                )),
            }
        }
    }

    impl serde::ser::Serialize for MaybeMediaSourceMetadata {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let fields = self.0.map_or(
                Fields {
                    source_status_id: None,
                    source_status_id_str: None,
                    source_user_id: None,
                    source_user_id_str: None,
                },
                |value| Fields {
                    source_status_id: Some(value.status_id),
                    source_status_id_str: Some(value.status_id),
                    source_user_id: value.user_id,
                    source_user_id_str: value.user_id,
                },
            );

            fields.serialize(serializer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_FIELDS: &str = concat!(
        r#""id":1,"id_str":"1","indices":[0,1],"media_url":"http://example.com/a.jpg","#,
        r#""media_url_https":"https://example.com/a.jpg","url":"https://t.co/a","#,
        r#""display_url":"pic.twitter.com/a","expanded_url":"https://twitter.com/x","type":"photo","#,
        r#""sizes":{"thumb":{"w":1,"h":1,"resize":"crop"},"small":{"w":1,"h":1,"resize":"fit"},"#,
        r#""medium":{"w":1,"h":1,"resize":"fit"},"large":{"w":1,"h":1,"resize":"fit"}}"#
    );

    fn media_json(extra: &str) -> String {
        format!("{{{BASE_FIELDS}{extra}}}")
    }

    #[test]
    fn round_trips_source_metadata() {
        let json = media_json(
            r#","source_status_id":12,"source_status_id_str":"12","source_user_id":34,"source_user_id_str":"34""#,
        );

        let media = serde_json::from_str::<Media<'_>>(&json).unwrap();
        let expected = MediaSourceMetadata {
            status_id: 12,
            user_id: Some(34),
        };
        assert_eq!(media.source_metadata(), Some(&expected));

        let reserialized = serde_json::to_string(&media).unwrap();
        let reparsed = serde_json::from_str::<Media<'_>>(&reserialized).unwrap();
        assert_eq!(reparsed, media);
    }

    #[test]
    fn omits_absent_source_metadata() {
        let json = media_json("");
        let media = serde_json::from_str::<Media<'_>>(&json).unwrap();
        assert_eq!(media.source_metadata(), None);

        let reserialized = serde_json::to_string(&media).unwrap();
        assert!(!reserialized.contains("source_status_id"));
        assert_eq!(
            serde_json::from_str::<Media<'_>>(&reserialized).unwrap(),
            media
        );
    }

    #[test]
    fn round_trips_source_metadata_without_user_pair() {
        // 2015-era archives carry only the status pair (see `examples/tsg/2015-errors.ndjson`).
        let json = media_json(r#","source_status_id":12,"source_status_id_str":"12""#);

        let media = serde_json::from_str::<Media<'_>>(&json).unwrap();
        let expected = MediaSourceMetadata {
            status_id: 12,
            user_id: None,
        };
        assert_eq!(media.source_metadata(), Some(&expected));

        let reserialized = serde_json::to_string(&media).unwrap();
        assert!(!reserialized.contains("source_user_id"));
        assert_eq!(
            serde_json::from_str::<Media<'_>>(&reserialized).unwrap(),
            media
        );
    }

    #[test]
    fn rejects_partial_source_metadata() {
        // Regression: a partial set of `source_*` fields used to deserialize successfully as
        // `None`, silently dropping the present values.
        let half_pair = media_json(r#","source_status_id":12"#);
        assert!(serde_json::from_str::<Media<'_>>(&half_pair).is_err());

        let user_without_status = media_json(r#","source_user_id":34,"source_user_id_str":"34""#);
        assert!(serde_json::from_str::<Media<'_>>(&user_without_status).is_err());
    }

    #[test]
    fn rejects_mismatched_source_metadata() {
        let json = media_json(
            r#","source_status_id":12,"source_status_id_str":"13","source_user_id":34,"source_user_id_str":"34""#,
        );
        assert!(serde_json::from_str::<Media<'_>>(&json).is_err());
    }
}

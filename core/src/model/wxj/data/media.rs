use crate::model::{
    media::{MediaType, MediaVariant},
    metrics::MediaPublicMetrics,
};
use bounded_static_derive_more::ToStatic;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Media<'a> {
    #[serde(rename = "photo")]
    Photo {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        #[serde(borrow)]
        url: Cow<'a, str>,
        #[serde(borrow)]
        alt_text: Option<Cow<'a, str>>,
    },
    #[serde(rename = "video")]
    Video {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        variants: Vec<MediaVariant<'a>>,
        duration_ms: Option<usize>,
        #[serde(borrow)]
        preview_image_url: Cow<'a, str>,
    },
    #[serde(rename = "animated_gif")]
    AnimatedGif {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        variants: Vec<MediaVariant<'a>>,
        #[serde(borrow)]
        preview_image_url: Cow<'a, str>,
    },
}

impl<'a> Media<'a> {
    #[must_use]
    pub const fn metadata(&self) -> &MediaMetadata<'a> {
        match self {
            Self::Photo { metadata, .. }
            | Self::Video { metadata, .. }
            | Self::AnimatedGif { metadata, .. } => metadata,
        }
    }

    #[must_use]
    pub const fn media_type(&self) -> MediaType {
        match self {
            Self::Photo { .. } => MediaType::Photo,
            Self::Video { .. } => MediaType::Video,
            Self::AnimatedGif { .. } => MediaType::AnimatedGif,
        }
    }

    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Photo { url, .. } => Some(url),
            Self::Video { .. } | Self::AnimatedGif { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaMetadata<'a> {
    #[serde(borrow)]
    pub media_key: Cow<'a, str>,
    pub public_metrics: Option<MediaPublicMetrics>,
    pub height: usize,
    pub width: usize,
}

#[cfg(test)]
mod tests {
    use crate::test_support::{local_corpus, round_trip_jsonl};

    const FIXTURE: &str = include_str!("../../../../tests/data/wxj/media.jsonl");

    #[test]
    fn round_trips_media_fixture() {
        round_trip_jsonl::<super::Media<'_>>("wxj/media fixture", FIXTURE);
    }

    #[test]
    fn round_trips_media_corpus() {
        for (path, contents) in local_corpus("wxj/media") {
            round_trip_jsonl::<super::Media<'_>>(&path, &contents);
        }
    }
}

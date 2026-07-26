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
    #[test]
    fn deserialize_media_examples() {
        let lines = include_str!("../../../../../examples/wxj/media.ndjson")
            .split('\n')
            .filter(|line| !line.is_empty());

        for (i, line) in lines.enumerate() {
            let result = serde_json::from_str::<super::Media<'_>>(line);

            if let Err(error) = &result {
                println!(
                    "Line {}: {line:?} is an invalid media object: {error}",
                    i + 1
                );
            }

            assert!(result.is_ok());
        }
    }
}

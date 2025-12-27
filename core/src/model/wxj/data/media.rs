use crate::model::{
    media::{MediaType, MediaVariant},
    metrics::MediaPublicMetrics,
};
use bounded_static_derive_more::ToStatic;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Media<'a> {
    #[serde(rename = "photo")]
    Photo {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        url: Cow<'a, str>,
        alt_text: Option<Cow<'a, str>>,
    },
    #[serde(rename = "video")]
    Video {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        variants: Vec<MediaVariant<'a>>,
        duration_ms: Option<usize>,
        preview_image_url: Cow<'a, str>,
    },
    #[serde(rename = "animated_gif")]
    AnimatedGif {
        #[serde(flatten)]
        metadata: MediaMetadata<'a>,
        variants: Vec<MediaVariant<'a>>,
        preview_image_url: Cow<'a, str>,
    },
}

impl bounded_static::IntoBoundedStatic for Media<'_> {
    type Static = Media<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Photo {
                metadata,
                url,
                alt_text,
            } => Media::Photo {
                metadata: metadata.into_static(),
                url: url.to_string().into(),
                alt_text: alt_text.map(|alt_text| alt_text.to_string().into()),
            },
            Self::Video {
                metadata,
                variants,
                duration_ms,
                preview_image_url,
            } => Media::Video {
                metadata: metadata.into_static(),
                variants: variants
                    .into_iter()
                    .map(bounded_static::IntoBoundedStatic::into_static)
                    .collect(),
                duration_ms,
                preview_image_url: preview_image_url.to_string().into(),
            },
            Self::AnimatedGif {
                metadata,
                variants,
                preview_image_url,
            } => Media::AnimatedGif {
                metadata: metadata.into_static(),
                variants: variants
                    .into_iter()
                    .map(bounded_static::IntoBoundedStatic::into_static)
                    .collect(),
                preview_image_url: preview_image_url.to_string().into(),
            },
        }
    }
}

impl bounded_static::ToBoundedStatic for Media<'_> {
    type Static = Media<'static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::Photo {
                metadata,
                url,
                alt_text,
            } => Media::Photo {
                metadata: metadata.to_static(),
                url: url.to_static(),
                alt_text: alt_text
                    .as_ref()
                    .map(bounded_static::ToBoundedStatic::to_static),
            },
            Self::Video {
                metadata,
                variants,
                duration_ms,
                preview_image_url,
            } => Media::Video {
                metadata: metadata.to_static(),
                variants: variants
                    .iter()
                    .map(bounded_static::ToBoundedStatic::to_static)
                    .collect(),
                duration_ms: *duration_ms,
                preview_image_url: preview_image_url.to_string().into(),
            },
            Self::AnimatedGif {
                metadata,
                variants,
                preview_image_url,
            } => Media::AnimatedGif {
                metadata: metadata.to_static(),
                variants: variants
                    .iter()
                    .map(bounded_static::ToBoundedStatic::to_static)
                    .collect(),
                preview_image_url: preview_image_url.to_string().into(),
            },
        }
    }
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
    pub media_key: Cow<'a, str>,
    pub public_metrics: Option<MediaPublicMetrics>,
    pub height: usize,
    pub width: usize,
}

mod tests {
    #[test]
    fn deserialize_media_examples() {
        let lines = include_str!("../../../../../examples/wxj/media.ndjson")
            .split("\n")
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

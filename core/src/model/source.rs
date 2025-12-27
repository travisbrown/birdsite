use regex::Regex;
use serde::de::{Deserializer, Unexpected, Visitor};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceName {
    Known(Source),
    Other(String),
}

impl SourceName {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Known(source) => source.name(),
            Self::Other(name) => name,
        }
    }

    #[must_use]
    pub const fn source(&self) -> Option<Source> {
        match self {
            Self::Known(source) => Some(*source),
            Self::Other(_) => None,
        }
    }
}

impl FromStr for SourceName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Source::from_name(s).map_or_else(|| Self::Other(s.to_string()), Self::Known))
    }
}

impl Display for SourceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(source) => source.name().fmt(f),
            Self::Other(name) => name.fmt(f),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for SourceName {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SourceNameVisitor;

        impl Visitor<'_> for SourceNameVisitor {
            type Value = SourceName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SourceName")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<Self::Value>().map_err(|()| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"source name")
                })
            }
        }

        deserializer.deserialize_str(SourceNameVisitor)
    }
}

impl serde::ser::Serialize for SourceName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceAnchor {
    Known(Source),
    Other { url: String, name: String },
    Empty,
}

impl SourceAnchor {
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Known(source) => Some(source.name()),
            Self::Other { name, .. } => Some(name),
            Self::Empty => None,
        }
    }

    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Known(source) => Some(source.url()),
            Self::Other { url, .. } => Some(url),
            Self::Empty => None,
        }
    }

    #[must_use]
    pub const fn source(&self) -> Option<Source> {
        match self {
            Self::Known(source) => Some(*source),
            Self::Other { .. } | Self::Empty => None,
        }
    }
}

impl FromStr for SourceAnchor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self::Empty)
        } else {
            Source::parse_anchor(s)
                .ok_or(())
                .map(|result| match result {
                    Ok(source) => Self::Known(source),
                    Err((url, name)) => Self::Other {
                        url: url.to_string(),
                        name: name.to_string(),
                    },
                })
        }
    }
}

impl Display for SourceAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(source) => source.anchor().fmt(f),
            Self::Other { name, url, .. } => Source::format_anchor(url, name).fmt(f),
            Self::Empty => Ok(()),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for SourceAnchor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SourceAnchorVisitor;

        impl Visitor<'_> for SourceAnchorVisitor {
            type Value = SourceAnchor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SourceAnchor")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<Self::Value>().map_err(|()| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"source anchor")
                })
            }
        }

        deserializer.deserialize_str(SourceAnchorVisitor)
    }
}

impl serde::ser::Serialize for SourceAnchor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Sources for tweets
///
/// This list is derived from a collection of real tweets, and is ordered by decreasing popularity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    TwitterForIPhone,
    TwitterForAndroid,
    TwitterWebApp,
    TwitterWebClient,
    TwitterForIPad,
    TweetDeck,
    SocialFlow,
    TwitterLite,
    Sprinklr,
    Freshdesk,
    TwitterMediaStudio,
}

impl Source {
    #[must_use]
    pub const fn url(&self) -> &str {
        SOURCE_MAPPINGS[*self as usize].1
    }

    #[must_use]
    pub const fn name(&self) -> &str {
        SOURCE_MAPPINGS[*self as usize].2
    }

    #[must_use]
    pub fn anchor(&self) -> String {
        let (_, url, name) = SOURCE_MAPPINGS[*self as usize];

        Self::format_anchor(url, name)
    }

    pub fn values() -> impl Iterator<Item = Self> {
        SOURCE_MAPPINGS.iter().map(|(source, _, _)| *source)
    }

    fn format_anchor(url: &str, name: &str) -> String {
        format!("<a href=\"{url}\" rel=\"nofollow\">{name}</a>")
    }

    fn parse_anchor(anchor: &str) -> Option<Result<Self, (&str, &str)>> {
        let (url, name) = SOURCE_ANCHOR_RE
            .captures(anchor)
            .and_then(|captures| captures.get(1).zip(captures.get(2)))?;

        let result = SOURCE_MAPPINGS
            .iter()
            .position(|(_, known_url, known_name)| {
                url.as_str() == *known_url && name.as_str() == *known_name
            })
            .map_or_else(
                || Err((url.as_str(), name.as_str())),
                |index| {
                    let source = SOURCE_MAPPINGS[index].0;

                    Ok(source)
                },
            );

        Some(result)
    }

    fn from_name(name: &str) -> Option<Self> {
        BY_NAME.get(name).copied()
    }
}

const SOURCE_MAPPINGS: [(Source, &str, &str); 11] = [
    (
        Source::TwitterForIPhone,
        "http://twitter.com/download/iphone",
        "Twitter for iPhone",
    ),
    (
        Source::TwitterForAndroid,
        "http://twitter.com/download/android",
        "Twitter for Android",
    ),
    (
        Source::TwitterWebApp,
        "https://mobile.twitter.com",
        "Twitter Web App",
    ),
    (
        Source::TwitterWebClient,
        "http://twitter.com",
        "Twitter Web Client",
    ),
    (
        Source::TwitterForIPad,
        "http://twitter.com/#!/download/ipad",
        "Twitter for iPad",
    ),
    (
        Source::TweetDeck,
        "https://about.twitter.com/products/tweetdeck",
        "TweetDeck",
    ),
    (
        Source::SocialFlow,
        "http://www.socialflow.com",
        "SocialFlow",
    ),
    (
        Source::TwitterLite,
        "https://mobile.twitter.com",
        "Twitter Lite",
    ),
    (Source::Sprinklr, "https://www.sprinklr.com", "Sprinklr"),
    (Source::Freshdesk, "https://freshdesk.com", "Freshdesk"),
    (
        Source::TwitterMediaStudio,
        "https://studio.twitter.com",
        "Twitter Media Studio",
    ),
];

static BY_NAME: LazyLock<HashMap<String, Source>> = LazyLock::new(|| {
    SOURCE_MAPPINGS
        .iter()
        .map(|(source, _, name)| ((*name).to_string(), *source))
        .collect()
});

static SOURCE_ANCHOR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^<a href="([^"]+)" rel="nofollow">([^<]+)</a>$"#).unwrap());

#[cfg(test)]
mod tests {
    use super::{Source, SourceAnchor};

    #[test]
    fn round_trip_known_sources() {
        for source in Source::values() {
            let anchor = source.anchor();
            let anchor_json = serde_json::json!(anchor);
            let deserialized: SourceAnchor =
                serde_json::from_str(&anchor_json.to_string()).unwrap();

            assert_eq!(deserialized.source(), Some(source));

            let reserialized = serde_json::json!(deserialized);

            assert_eq!(reserialized, anchor_json);
        }
    }

    #[test]
    fn deserialize_unknown_sources() {
        let expected = SourceAnchor::Other {
            url: "https://example.com".into(),
            name: "Example Client".into(),
        };
        let anchor = expected.to_string();

        let anchor_json = serde_json::json!(anchor).to_string();
        let deserialized: SourceAnchor = serde_json::from_str(&anchor_json).unwrap();

        assert_eq!(deserialized, expected);
    }
}

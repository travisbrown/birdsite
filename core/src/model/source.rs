use regex::Regex;
use serde::{
    de::{Deserialize, Deserializer, Unexpected, Visitor},
    ser::Serialize,
};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown source name")]
    UnknownName(String),
}

/// Enumerated sources for tweets.
///
/// This list is derived from a collection of real tweets, and is ordered by decreasing popularity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum KnownSource {
    TwitterForIPhone,
    TwitterWebApp,
    TwitterForAndroid,
    TwitterForIPad,
    TweetDeckWebApp,
    AdvertiserInterface,
    SocialFlow,
    TwitterMediaStudio,
    SproutSocial,
    Buffer,
    DlvrIt,
    HootSuiteInc,
    TweetDeck,
    TwitterForAdvertisers,
    TwitterAds,
    TheWhiteHouse,
    TwitterWebClient,
    Wildmoka,
    TrueanthemPro2,
    Nonli,
    Healthb0t,
    Sprinklr,
    Illuminatibot,
    Ifttt,
    Empty,
}

/// Sources for tweets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source<'a> {
    Known(KnownSource),
    Other { name: Cow<'a, str> },
}

/// Source anchors for tweets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceAnchor {
    Known(KnownSource),
    // Note that these fields cannot be borrowed because of escaping in the source JSON.
    Other { url: String, name: String },
}

const URLS_AND_NAMES: [(&str, &str); 24] = [
    ("http://twitter.com/download/iphone", "Twitter for iPhone"),
    ("https://mobile.twitter.com", "Twitter Web App"),
    ("http://twitter.com/download/android", "Twitter for Android"),
    ("http://twitter.com/#!/download/ipad", "Twitter for iPad"),
    ("https://twitter.com", "TweetDeck Web App"),
    (
        "https://help.twitter.com/en/using-twitter/how-to-tweet#source-labels",
        "advertiser-interface",
    ),
    ("http://www.socialflow.com", "SocialFlow"),
    ("https://studio.twitter.com", "Twitter Media Studio"),
    ("https://sproutsocial.com", "Sprout Social"),
    ("https://buffer.com", "Buffer"),
    ("https://dlvrit.com/", "dlvr.it"),
    ("https://www.hootsuite.com", "Hootsuite Inc."),
    ("https://about.twitter.com/products/tweetdeck", "TweetDeck"),
    ("https://twitter.com", "Twitter for Advertisers"),
    ("https://ads.twitter.com", "Twitter Ads"),
    ("https://www.sprinklr.com", "The White House"),
    ("http://twitter.com", "Twitter Web Client"),
    ("http://wildmoka.com", "Wildmoka"),
    ("https://trueanthem.com", "trueanthem_pro2"),
    ("https://www.nonli.com", "Nonli"),
    ("https://www.google.com/", "healthb0t"),
    ("https://www.sprinklr.com", "Sprinklr"),
    ("https://google.com", "illuminatibot"),
    ("https://ifttt.com", "Ifttt"),
];

const KNOWN_SOURCES: [KnownSource; 24] = [
    KnownSource::TwitterForIPhone,
    KnownSource::TwitterWebApp,
    KnownSource::TwitterForAndroid,
    KnownSource::TwitterForIPad,
    KnownSource::TweetDeckWebApp,
    KnownSource::AdvertiserInterface,
    KnownSource::SocialFlow,
    KnownSource::TwitterMediaStudio,
    KnownSource::SproutSocial,
    KnownSource::Buffer,
    KnownSource::DlvrIt,
    KnownSource::HootSuiteInc,
    KnownSource::TweetDeck,
    KnownSource::TwitterForAdvertisers,
    KnownSource::TwitterAds,
    KnownSource::TheWhiteHouse,
    KnownSource::TwitterWebClient,
    KnownSource::Wildmoka,
    KnownSource::TrueanthemPro2,
    KnownSource::Nonli,
    KnownSource::Healthb0t,
    KnownSource::Sprinklr,
    KnownSource::Illuminatibot,
    KnownSource::Ifttt,
];

impl KnownSource {
    pub fn name(&self) -> &str {
        match self {
            Self::TwitterForIPhone => URLS_AND_NAMES[0].1,
            Self::TwitterWebApp => URLS_AND_NAMES[1].1,
            Self::TwitterForAndroid => URLS_AND_NAMES[2].1,
            Self::TwitterForIPad => URLS_AND_NAMES[3].1,
            Self::TweetDeckWebApp => URLS_AND_NAMES[4].1,
            Self::AdvertiserInterface => URLS_AND_NAMES[5].1,
            Self::SocialFlow => URLS_AND_NAMES[6].1,
            Self::TwitterMediaStudio => URLS_AND_NAMES[7].1,
            Self::SproutSocial => URLS_AND_NAMES[8].1,
            Self::Buffer => URLS_AND_NAMES[9].1,
            Self::DlvrIt => URLS_AND_NAMES[10].1,
            Self::HootSuiteInc => URLS_AND_NAMES[11].1,
            Self::TweetDeck => URLS_AND_NAMES[12].1,
            Self::TwitterForAdvertisers => URLS_AND_NAMES[13].1,
            Self::TwitterAds => URLS_AND_NAMES[14].1,
            Self::TheWhiteHouse => URLS_AND_NAMES[15].1,
            Self::TwitterWebClient => URLS_AND_NAMES[16].1,
            Self::Wildmoka => URLS_AND_NAMES[17].1,
            Self::TrueanthemPro2 => URLS_AND_NAMES[18].1,
            Self::Nonli => URLS_AND_NAMES[19].1,
            Self::Healthb0t => URLS_AND_NAMES[20].1,
            Self::Sprinklr => URLS_AND_NAMES[21].1,
            Self::Illuminatibot => URLS_AND_NAMES[22].1,
            Self::Ifttt => URLS_AND_NAMES[23].1,
            Self::Empty => "",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        static NAME_MAP: LazyLock<HashMap<String, KnownSource>> = LazyLock::new(|| {
            URLS_AND_NAMES
                .iter()
                .zip(KNOWN_SOURCES)
                .map(|((_, name), value)| (name.to_string(), value))
                .collect()
        });

        NAME_MAP.get(s).copied()
    }
}

impl FromStr for KnownSource {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| Self::Err::UnknownName(s.to_string()))
    }
}

impl Display for KnownSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl<'de> Deserialize<'de> for KnownSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct KnownSourceVisitor;

        impl Visitor<'_> for KnownSourceVisitor {
            type Value = KnownSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct KnownSource")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                KnownSource::from_str(v).ok_or_else(|| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"source name")
                })
            }
        }

        deserializer.deserialize_str(KnownSourceVisitor)
    }
}

impl Serialize for KnownSource {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'a> Source<'a> {
    pub fn into_owned(self) -> Source<'static> {
        match self {
            Source::Known(source) => Source::Known(source),
            Source::Other { name } => Source::Other {
                name: name.to_string().into(),
            },
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Known(source) => source.name(),
            Self::Other { name } => name,
        }
    }

    pub fn parse_str(s: &'a str) -> Self {
        KnownSource::from_str(s)
            .map(Self::Known)
            .unwrap_or_else(|| Self::Other { name: s.into() })
    }
}

impl<'a> Display for Source<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(source) => source.fmt(f),
            Self::Other { name } => name.fmt(f),
        }
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for Source<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SourceVisitor<'a> {
            _a: std::marker::PhantomData<&'a ()>,
        }

        impl<'a, 'de> Visitor<'de> for SourceVisitor<'a> {
            type Value = Source<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Source")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(KnownSource::from_str(v)
                    .map(Self::Value::Known)
                    .unwrap_or_else(|| Self::Value::Other {
                        name: v.to_string().into(),
                    }))
            }
        }

        deserializer.deserialize_str(SourceVisitor {
            _a: std::marker::PhantomData,
        })
    }
}

impl<'a> Serialize for Source<'a> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl SourceAnchor {
    pub fn name(&self) -> &str {
        match self {
            Self::Known(source) => source.name(),
            Self::Other { name, .. } => name,
        }
    }

    pub fn url(&self) -> &str {
        static URL_MAP: LazyLock<BTreeMap<KnownSource, String>> = LazyLock::new(|| {
            URLS_AND_NAMES
                .iter()
                .zip(KNOWN_SOURCES)
                .map(|((url, _), value)| (value, url.to_string()))
                .collect()
        });

        match self {
            Self::Known(source) => {
                // Safe because we control the enumeration of known sources.
                URL_MAP.get(source).expect("Invalid source URL map")
            }
            Self::Other { url, .. } => url,
        }
    }

    pub fn anchor(&self) -> String {
        match self {
            Self::Known(KnownSource::Empty) => String::new(),
            other => format!(
                "<a href=\"{}\" rel=\"nofollow\">{}</a>",
                other.url(),
                other.name()
            ),
        }
    }

    fn parse_str(s: &str) -> Option<Self> {
        static SOURCE_ANCHOR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"^<a href="([^"]+)" rel="nofollow">([^<]+)</a>$"#).unwrap()
        });

        if s.is_empty() {
            Some(Self::Known(KnownSource::Empty))
        } else {
            let (url, name) = SOURCE_ANCHOR_PATTERN
                .captures(s)
                .and_then(|captures| captures.get(1).zip(captures.get(2)))?;

            Some(
                match URLS_AND_NAMES.iter().position(|(known_url, known_name)| {
                    url.as_str() == *known_url && name.as_str() == *known_name
                }) {
                    Some(index) => Self::Known(KNOWN_SOURCES[index]),
                    None => Self::Other {
                        url: url.as_str().to_string(),
                        name: name.as_str().to_string(),
                    },
                },
            )
        }
    }
}

impl<'de> Deserialize<'de> for SourceAnchor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SourceAnchorVisitor;

        impl Visitor<'_> for SourceAnchorVisitor {
            type Value = SourceAnchor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct SourceAnchor")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                SourceAnchor::parse_str(v).ok_or_else(|| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"source anchor")
                })
            }
        }

        deserializer.deserialize_str(SourceAnchorVisitor)
    }
}

impl Serialize for SourceAnchor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.anchor().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_known_sources() {
        for known_source in KNOWN_SOURCES {
            let source = SourceAnchor::Known(known_source);
            let anchor = source.anchor();
            let anchor_json = serde_json::json!(anchor);
            let deserialized: SourceAnchor =
                serde_json::from_str(&anchor_json.to_string()).unwrap();

            assert_eq!(deserialized, source);

            let reserialized = serde_json::json!(deserialized);

            assert_eq!(reserialized, anchor_json);
        }
    }

    #[test]
    fn deserialize_unknown_sources() {
        let source = SourceAnchor::Other {
            url: "https://example.com".into(),
            name: "Example Client".into(),
        };
        let anchor = source.anchor();

        let anchor_json = serde_json::json!(anchor).to_string();
        let deserialized: SourceAnchor = serde_json::from_str(&anchor_json).unwrap();

        assert_eq!(deserialized, source);
    }
}

use regex::Regex;
use serde::{
    de::{Deserialize, Deserializer, Unexpected, Visitor},
    Serialize,
};
use std::sync::LazyLock;

static SOURCE_ANCHOR_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^<a href="([^"]+)" rel="nofollow">([^<]+)</a>$"#).unwrap());

/// Sources for tweets
///
/// This list is derived from a collection of real tweets, and is ordered by decreasing popularity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source {
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
    // Note that these fields cannot be borrowed because of escaping in the source JSON.
    Other { url: String, name: String },
}

const URLS_AND_NAMES: &[(&str, &str)] = &[
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

const SOURCES: &[Source] = &[
    Source::TwitterForIPhone,
    Source::TwitterWebApp,
    Source::TwitterForAndroid,
    Source::TwitterForIPad,
    Source::TweetDeckWebApp,
    Source::AdvertiserInterface,
    Source::SocialFlow,
    Source::TwitterMediaStudio,
    Source::SproutSocial,
    Source::Buffer,
    Source::DlvrIt,
    Source::HootSuiteInc,
    Source::TweetDeck,
    Source::TwitterForAdvertisers,
    Source::TwitterAds,
    Source::TheWhiteHouse,
    Source::TwitterWebClient,
    Source::Wildmoka,
    Source::TrueanthemPro2,
    Source::Nonli,
    Source::Healthb0t,
    Source::Sprinklr,
    Source::Illuminatibot,
    Source::Ifttt,
];

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SourceVisitor;

        impl Visitor<'_> for SourceVisitor {
            type Value = Source;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Source")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Source::parse_anchor(v).ok_or_else(|| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"source anchor")
                })
            }
        }

        deserializer.deserialize_str(SourceVisitor)
    }
}

impl Serialize for Source {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.anchor().serialize(serializer)
    }
}

impl Source {
    pub fn url(&self) -> &str {
        match self {
            Source::TwitterForIPhone => URLS_AND_NAMES[0].0,
            Source::TwitterWebApp => URLS_AND_NAMES[1].0,
            Source::TwitterForAndroid => URLS_AND_NAMES[2].0,
            Source::TwitterForIPad => URLS_AND_NAMES[3].0,
            Source::TweetDeckWebApp => URLS_AND_NAMES[4].0,
            Source::AdvertiserInterface => URLS_AND_NAMES[5].0,
            Source::SocialFlow => URLS_AND_NAMES[6].0,
            Source::TwitterMediaStudio => URLS_AND_NAMES[7].0,
            Source::SproutSocial => URLS_AND_NAMES[8].0,
            Source::Buffer => URLS_AND_NAMES[9].0,
            Source::DlvrIt => URLS_AND_NAMES[10].0,
            Source::HootSuiteInc => URLS_AND_NAMES[11].0,
            Source::TweetDeck => URLS_AND_NAMES[12].0,
            Source::TwitterForAdvertisers => URLS_AND_NAMES[13].0,
            Source::TwitterAds => URLS_AND_NAMES[14].0,
            Source::TheWhiteHouse => URLS_AND_NAMES[15].0,
            Source::TwitterWebClient => URLS_AND_NAMES[16].0,
            Source::Wildmoka => URLS_AND_NAMES[17].0,
            Source::TrueanthemPro2 => URLS_AND_NAMES[18].0,
            Source::Nonli => URLS_AND_NAMES[19].0,
            Source::Healthb0t => URLS_AND_NAMES[20].0,
            Source::Sprinklr => URLS_AND_NAMES[21].0,
            Source::Illuminatibot => URLS_AND_NAMES[22].0,
            Source::Ifttt => URLS_AND_NAMES[23].0,
            Source::Other { url, .. } => url,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Source::TwitterForIPhone => URLS_AND_NAMES[0].1,
            Source::TwitterWebApp => URLS_AND_NAMES[1].1,
            Source::TwitterForAndroid => URLS_AND_NAMES[2].1,
            Source::TwitterForIPad => URLS_AND_NAMES[3].1,
            Source::TweetDeckWebApp => URLS_AND_NAMES[4].1,
            Source::AdvertiserInterface => URLS_AND_NAMES[5].1,
            Source::SocialFlow => URLS_AND_NAMES[6].1,
            Source::TwitterMediaStudio => URLS_AND_NAMES[7].1,
            Source::SproutSocial => URLS_AND_NAMES[8].1,
            Source::Buffer => URLS_AND_NAMES[9].1,
            Source::DlvrIt => URLS_AND_NAMES[10].1,
            Source::HootSuiteInc => URLS_AND_NAMES[11].1,
            Source::TweetDeck => URLS_AND_NAMES[12].1,
            Source::TwitterForAdvertisers => URLS_AND_NAMES[13].1,
            Source::TwitterAds => URLS_AND_NAMES[14].1,
            Source::TheWhiteHouse => URLS_AND_NAMES[15].1,
            Source::TwitterWebClient => URLS_AND_NAMES[16].1,
            Source::Wildmoka => URLS_AND_NAMES[17].1,
            Source::TrueanthemPro2 => URLS_AND_NAMES[18].1,
            Source::Nonli => URLS_AND_NAMES[19].1,
            Source::Healthb0t => URLS_AND_NAMES[20].1,
            Source::Sprinklr => URLS_AND_NAMES[21].1,
            Source::Illuminatibot => URLS_AND_NAMES[22].1,
            Source::Ifttt => URLS_AND_NAMES[23].1,
            Source::Other { name, .. } => name,
        }
    }

    pub fn anchor(&self) -> String {
        format!(
            "<a href=\"{}\" rel=\"nofollow\">{}</a>",
            self.url(),
            self.name()
        )
    }

    fn parse_anchor(s: &str) -> Option<Self> {
        let (url, name) = SOURCE_ANCHOR_PATTERN
            .captures(s)
            .and_then(|captures| captures.get(1).zip(captures.get(2)))?;

        Some(
            match URLS_AND_NAMES.iter().position(|(known_url, known_name)| {
                url.as_str() == *known_url && name.as_str() == *known_name
            }) {
                Some(index) => SOURCES[index].clone(),
                None => Self::Other {
                    url: url.as_str().to_string(),
                    name: name.as_str().to_string(),
                },
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_known_sources() {
        for source in SOURCES {
            let anchor = source.anchor();
            let anchor_json = serde_json::json!(anchor);
            let deserialized: Source = serde_json::from_str(&anchor_json.to_string()).unwrap();

            assert_eq!(&deserialized, source);

            let reserialized = serde_json::json!(deserialized);

            assert_eq!(reserialized, anchor_json);
        }
    }

    #[test]
    fn deserialize_unknown_sources() {
        let source = Source::Other {
            url: "https://example.com".into(),
            name: "Example Client".into(),
        };
        let anchor = source.anchor();

        let anchor_json = serde_json::json!(anchor).to_string();
        let deserialized: Source = serde_json::from_str(&anchor_json).unwrap();

        assert_eq!(deserialized, source);
    }
}

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserialize, Deserializer, Unexpected, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt::Display;
use std::str::FromStr;

const TWITTER_DATE_TIME_FMT: &str = "%a %b %d %H:%M:%S %z %Y";

/// Twitter's representation of a timestamp as a human-readable string.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TextTimestamp(pub DateTime<Utc>);

impl Display for TextTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.format(TWITTER_DATE_TIME_FMT).fmt(f)
    }
}

impl FromStr for TextTimestamp {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::parse_from_str(s, TWITTER_DATE_TIME_FMT).map(|value| Self(value.into()))
    }
}

impl<'de> Deserialize<'de> for TextTimestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct TextTimestampVisitor;

        impl Visitor<'_> for TextTimestampVisitor {
            type Value = TextTimestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct TextTimestamp")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<Self::Value>().map_err(|_| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"text timestamp")
                })
            }
        }

        deserializer.deserialize_str(TextTimestampVisitor)
    }
}

impl Serialize for TextTimestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::timestamp::TextTimestamp;
    use chrono::{TimeZone, Utc};
    use std::io::Cursor;

    const SAMPLE_TEXT_TIMESTAMP: &str = "Thu Jun 25 16:18:41 +0000 2009";
    const SAMPLE_EPOCH_S: i64 = 1245946721;

    #[test]
    fn parse_text_timestamp() {
        let expected = TextTimestamp(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(SAMPLE_TEXT_TIMESTAMP.parse(), Ok(expected));
    }

    #[test]
    fn display_text_timestamp() {
        let value = TextTimestamp(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(&value.to_string(), SAMPLE_TEXT_TIMESTAMP);
    }

    #[test]
    fn deserialize_text_timestamp() {
        let expected = TextTimestamp(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(
            serde_json::from_str::<TextTimestamp>(&format!("\"{}\"", SAMPLE_TEXT_TIMESTAMP))
                .unwrap(),
            expected
        );

        assert_eq!(
            serde_json::from_reader::<_, TextTimestamp>(Cursor::new(&format!(
                "\"{}\"",
                SAMPLE_TEXT_TIMESTAMP
            )))
            .unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_text_timestamp() {
        let value = TextTimestamp(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(
            serde_json::json!(value),
            serde_json::json!(SAMPLE_TEXT_TIMESTAMP)
        );
    }
}

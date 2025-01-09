use chrono::{DateTime, Utc};
use serde::{
    de::{Deserialize, Deserializer, Unexpected, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt::Display;
use std::str::FromStr;

const TWITTER_DATE_TIME_FMT: &str = "%a %b %d %H:%M:%S %z %Y";

/// Timestamp representing when an account or post was created.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CreatedAt(pub DateTime<Utc>);

impl Display for CreatedAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.format(TWITTER_DATE_TIME_FMT).fmt(f)
    }
}

impl FromStr for CreatedAt {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::parse_from_str(s, TWITTER_DATE_TIME_FMT).map(|value| Self(value.into()))
    }
}

impl<'de> Deserialize<'de> for CreatedAt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CreatedAtVisitor;

        impl Visitor<'_> for CreatedAtVisitor {
            type Value = CreatedAt;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CreatedAt")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<Self::Value>().map_err(|_| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"created_at date")
                })
            }
        }

        deserializer.deserialize_str(CreatedAtVisitor)
    }
}

impl Serialize for CreatedAt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::CreatedAt;
    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use std::io::Cursor;

    const SAMPLE_CREATED_AT: &str = "Thu Jun 25 16:18:41 +0000 2009";
    const SAMPLE_EPOCH_S: i64 = 1245946721;

    #[test]
    fn parse_created_at() {
        let expected = CreatedAt(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(SAMPLE_CREATED_AT.parse(), Ok(expected));
    }

    #[test]
    fn display_created_at() {
        let value = CreatedAt(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(&value.to_string(), SAMPLE_CREATED_AT);
    }

    #[test]
    fn deserialize_created_at() {
        let expected = CreatedAt(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(
            serde_json::from_str::<CreatedAt>(&format!("\"{}\"", SAMPLE_CREATED_AT)).unwrap(),
            expected
        );

        assert_eq!(
            serde_json::from_reader::<_, CreatedAt>(Cursor::new(&format!(
                "\"{}\"",
                SAMPLE_CREATED_AT
            )))
            .unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_created_at() {
        let value = CreatedAt(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(json!(value), json!(SAMPLE_CREATED_AT));
    }
}

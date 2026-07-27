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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Twitter's representation of a timestamp in the ISO 8601 form used by the v2 API.
///
/// The wire format always writes millisecond precision, including the trailing zeros that
/// `chrono`'s own RFC 3339 output omits, so preserving it requires an explicit representation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MillisecondTimestamp(pub DateTime<Utc>);

impl Display for MillisecondTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
            .fmt(f)
    }
}

impl FromStr for MillisecondTimestamp {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::parse_from_rfc3339(s).map(|value| Self(value.into()))
    }
}

impl<'de> Deserialize<'de> for MillisecondTimestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MillisecondTimestampVisitor;

        impl Visitor<'_> for MillisecondTimestampVisitor {
            type Value = MillisecondTimestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct MillisecondTimestamp")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<Self::Value>().map_err(|_| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"ISO 8601 timestamp")
                })
            }
        }

        deserializer.deserialize_str(MillisecondTimestampVisitor)
    }
}

impl Serialize for MillisecondTimestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::timestamp::{MillisecondTimestamp, TextTimestamp};
    use chrono::{TimeZone, Utc};
    use std::io::Cursor;

    const SAMPLE_TEXT_TIMESTAMP: &str = "Thu Jun 25 16:18:41 +0000 2009";
    const SAMPLE_EPOCH_S: i64 = 1_245_946_721;

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
            serde_json::from_str::<TextTimestamp>(&format!("\"{SAMPLE_TEXT_TIMESTAMP}\"")).unwrap(),
            expected
        );

        assert_eq!(
            serde_json::from_reader::<_, TextTimestamp>(Cursor::new(&format!(
                "\"{SAMPLE_TEXT_TIMESTAMP}\""
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

    /// Property: every second-precision timestamp round-trips through its JSON representation
    /// (which exercises both `Display` and `FromStr`, since the Serde implementations delegate to
    /// them).
    #[test_strategy::proptest]
    fn round_trip_arbitrary_text_timestamp_json(timestamp: TextTimestamp) {
        let json = serde_json::to_string(&timestamp).unwrap();
        let parsed: TextTimestamp = serde_json::from_str(&json).unwrap();

        proptest::prop_assert_eq!(timestamp, parsed);
    }

    #[test]
    fn display_millisecond_timestamp_keeps_trailing_zeros() {
        let value = MillisecondTimestamp(Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap());

        assert_eq!(&value.to_string(), "2009-06-25T16:18:41.000Z");
    }

    /// Property: every millisecond-precision timestamp round-trips through its JSON
    /// representation, which exercises both `Display` and `FromStr`.
    #[test_strategy::proptest]
    fn round_trip_arbitrary_millisecond_timestamp_json(timestamp: MillisecondTimestamp) {
        let json = serde_json::to_string(&timestamp).unwrap();
        let parsed: MillisecondTimestamp = serde_json::from_str(&json).unwrap();

        proptest::prop_assert_eq!(timestamp, parsed);
    }

    impl proptest::arbitrary::Arbitrary for MillisecondTimestamp {
        type Parameters = ();
        type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<i64>, fn(i64) -> Self>;

        fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
            use proptest::strategy::Strategy;

            let make: fn(i64) -> Self = |milliseconds| {
                Self(
                    Utc.timestamp_millis_opt(milliseconds)
                        .single()
                        .expect("milliseconds are within the representable range"),
                )
            };

            // RFC 3339 writes a four-digit year, so cover 1970-01-01 through 9999-12-31.
            (0..=253_402_300_799_999_i64).prop_map(make)
        }
    }

    impl proptest::arbitrary::Arbitrary for TextTimestamp {
        type Parameters = ();
        type Strategy = proptest::strategy::Map<std::ops::RangeInclusive<i64>, fn(i64) -> Self>;

        fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
            use proptest::strategy::Strategy;

            // The annotation coerces the closure to a plain function pointer so that the concrete
            // strategy type above can name it (closures otherwise have unnameable types).
            let make: fn(i64) -> Self = |seconds| {
                Self(
                    Utc.timestamp_opt(seconds, 0)
                        .single()
                        .expect("seconds are within the representable range"),
                )
            };

            // The format's `%a %b %d` fields only exist at second precision, and `%Y` writes a
            // four-digit year, so cover 1970-01-01 through 9999-12-31 at second precision.
            (0..=253_402_300_799_i64).prop_map(make)
        }
    }
}

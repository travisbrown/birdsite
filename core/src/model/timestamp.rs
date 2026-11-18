use std::fmt::Display;
use std::str::FromStr;

use jiff::Timestamp;
use serde::de::{Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

const TWITTER_DATE_TIME_FMT: &str = "%a %b %d %H:%M:%S %z %Y";

/// Twitter's representation of a timestamp as a human-readable string.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TextTimestamp(pub Timestamp);

impl Display for TextTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.strftime(TWITTER_DATE_TIME_FMT).fmt(f)
    }
}

impl FromStr for TextTimestamp {
    type Err = jiff::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Timestamp::strptime(TWITTER_DATE_TIME_FMT, s).map(Self)
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use jiff::Timestamp;

    use crate::model::timestamp::TextTimestamp;

    const SAMPLE_TEXT_TIMESTAMP: &str = "Thu Jun 25 16:18:41 +0000 2009";
    const SAMPLE_EPOCH_S: i64 = 1_245_946_721;

    #[test]
    fn parse_text_timestamp() {
        let expected = TextTimestamp(Timestamp::from_second(SAMPLE_EPOCH_S).unwrap());

        assert_eq!(
            SAMPLE_TEXT_TIMESTAMP.parse::<TextTimestamp>().unwrap(),
            expected
        );
    }

    #[test]
    fn display_text_timestamp() {
        let value = TextTimestamp(Timestamp::from_second(SAMPLE_EPOCH_S).unwrap());

        assert_eq!(&value.to_string(), SAMPLE_TEXT_TIMESTAMP);
    }

    #[test]
    fn deserialize_text_timestamp() {
        let expected = TextTimestamp(Timestamp::from_second(SAMPLE_EPOCH_S).unwrap());

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
        let value = TextTimestamp(Timestamp::from_second(SAMPLE_EPOCH_S).unwrap());

        assert_eq!(
            serde_json::json!(value),
            serde_json::json!(SAMPLE_TEXT_TIMESTAMP)
        );
    }

    #[test]
    fn text_timestamp_normalizes_offsets_to_utc() {
        let timestamp: TextTimestamp = "Thu Jun 25 18:18:41 +0200 2009".parse().unwrap();
        assert_eq!(timestamp.to_string(), SAMPLE_TEXT_TIMESTAMP);
    }

    #[test]
    fn text_timestamp_preserves_dates_before_the_epoch() {
        let text = "Wed Dec 31 23:59:59 +0000 1969";
        let timestamp: TextTimestamp = text.parse().unwrap();
        assert_eq!(timestamp.0, Timestamp::from_second(-1).unwrap());
        assert_eq!(timestamp.to_string(), text);
    }

    #[test]
    fn text_timestamp_rejects_invalid_dates() {
        for text in [
            "not a timestamp",
            "Thu Feb 30 16:18:41 +0000 2009",
            "Thu Jun 25 25:18:41 +0000 2009",
        ] {
            assert!(text.parse::<TextTimestamp>().is_err(), "{text}");
        }
    }

    /// Property: every second-precision timestamp round-trips through its JSON representation
    /// (which exercises both `Display` and `FromStr`, since the Serde implementations delegate to
    /// them).
    #[proptest::property_test]
    fn round_trip_arbitrary_text_timestamp_json(timestamp: TextTimestamp) {
        let json = serde_json::to_string(&timestamp).unwrap();
        let parsed: TextTimestamp = serde_json::from_str(&json).unwrap();

        proptest::prop_assert_eq!(timestamp, parsed);
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
                    Timestamp::from_second(seconds)
                        .expect("seconds are within the representable range"),
                )
            };

            // The format's `%a %b %d` fields only exist at second precision, and `%Y` writes a
            // four-digit year, so cover 1970-01-01 through Jiff's maximum at second precision.
            (0..=Timestamp::MAX.as_second()).prop_map(make)
        }
    }
}

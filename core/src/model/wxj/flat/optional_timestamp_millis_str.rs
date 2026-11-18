use std::borrow::Cow;

use jiff::Timestamp;
use serde::de::{Deserialize, Deserializer, Error, Unexpected};
use serde::ser::Serializer;

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Timestamp>, D::Error> {
    Option::<Cow<'_, str>>::deserialize(deserializer)?
        .map(|value| {
            value
                .parse::<i64>()
                .ok()
                .and_then(|milliseconds| Timestamp::from_millisecond(milliseconds).ok())
                .ok_or_else(|| {
                    D::Error::invalid_value(Unexpected::Str(&value), &"epoch millisecond string")
                })
        })
        .transpose()
}

// Serde passes optional fields by reference.
#[allow(clippy::ref_option)]
pub fn serialize<S: Serializer>(
    value: &Option<Timestamp>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => serializer.collect_str(&value.as_nanosecond().div_euclid(1_000_000)),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use jiff::Timestamp;

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Data {
        #[serde(default, with = "super")]
        timestamp_ms: Option<Timestamp>,
    }

    #[test]
    fn millisecond_strings_preserve_their_wire_format() {
        for milliseconds in [-1001, -1, 0, 1, 1_245_946_721_123] {
            let json = format!(r#"{{"timestamp_ms":"{milliseconds}"}}"#);
            let data: Data = serde_json::from_str(&json).unwrap();
            assert_eq!(
                data.timestamp_ms,
                Some(Timestamp::from_millisecond(milliseconds).unwrap())
            );
            assert_eq!(serde_json::to_string(&data).unwrap(), json);
        }
    }

    #[test]
    fn missing_and_null_timestamps_are_absent() {
        for json in ["{}", r#"{"timestamp_ms":null}"#] {
            let data: Data = serde_json::from_str(json).unwrap();
            assert_eq!(data.timestamp_ms, None);
            assert_eq!(
                serde_json::to_string(&data).unwrap(),
                r#"{"timestamp_ms":null}"#
            );
        }
    }

    #[test]
    fn invalid_or_unquoted_timestamps_are_rejected() {
        for value in [r#""invalid""#, r#""9223372036854775807""#, "123", "true"] {
            let json = format!(r#"{{"timestamp_ms":{value}}}"#);
            assert!(serde_json::from_str::<Data>(&json).is_err(), "{json}");
        }
    }

    #[test]
    fn serialization_drops_fractional_milliseconds_before_the_epoch() {
        let data = Data {
            timestamp_ms: Some(Timestamp::from_nanosecond(-1).unwrap()),
        };
        assert_eq!(
            serde_json::to_string(&data).unwrap(),
            r#"{"timestamp_ms":"-1"}"#
        );
    }
}

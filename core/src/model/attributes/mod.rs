pub mod ratio_i64;
pub mod ratio_u64;

pub mod text_timestamp {
    use crate::model::timestamp::TextTimestamp;
    use chrono::{DateTime, Utc};
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        TextTimestamp::deserialize(deserializer).map(|text_timestamp| text_timestamp.0)
    }

    pub fn serialize<S: Serializer>(
        value: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        TextTimestamp::serialize(&TextTimestamp(*value), serializer)
    }
}

pub mod timestamp_msec {
    use chrono::{DateTime, Utc};
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        u64::deserialize(deserializer).and_then(|timestamp_msec| {
            timestamp_msec
                .try_into()
                .ok()
                .and_then(DateTime::from_timestamp_millis)
                .ok_or_else(|| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(timestamp_msec),
                        &"epoch millisecond",
                    )
                })
        })
    }

    pub fn serialize<S: Serializer>(
        value: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        i64::serialize(&value.timestamp_millis(), serializer)
    }
}

/// Decode a range from a pair of values.
pub mod range {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, SerializeTuple, Serializer},
    };
    use std::ops::Range;

    pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Range<T>, D::Error> {
        let (start, end) = Deserialize::deserialize(deserializer)?;

        Ok(start..end)
    }

    pub fn serialize<T: Serialize, S: Serializer>(
        value: &Range<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&value.start)?;
        tuple.serialize_element(&value.end)?;
        tuple.end()
    }
}

/// Decode a range from a pair of values.
pub mod range_opt {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, SerializeTuple, Serializer},
    };
    use std::ops::Range;

    pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Range<T>>, D::Error> {
        let range: Option<(T, T)> = Deserialize::deserialize(deserializer)?;

        Ok(range.map(|(start, end)| start..end))
    }

    pub fn serialize<T: Serialize, S: Serializer>(
        value: &Option<Range<T>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => {
                let mut tuple = serializer.serialize_tuple(2)?;
                tuple.serialize_element(&value.start)?;
                tuple.serialize_element(&value.end)?;
                tuple.end()
            }
            None => serializer.serialize_none(),
        }
    }
}

pub mod integer_str {
    use serde::{
        de::{Deserializer, Unexpected, Visitor},
        ser::Serializer,
    };
    use std::{marker::PhantomData, str::FromStr};

    const EXPECTED: &str = "integer string";

    pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        struct IntegerStrVisitor<T> {
            _target: PhantomData<T>,
        }

        impl<'de, T: FromStr> Visitor<'de> for IntegerStrVisitor<T> {
            type Value = T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(EXPECTED)
            }

            fn visit_borrowed_str<E: serde::de::Error>(
                self,
                v: &'de str,
            ) -> Result<Self::Value, E> {
                v.parse::<Self::Value>()
                    .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &EXPECTED))
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<Self::Value>()
                    .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &EXPECTED))
            }
        }

        deserializer.deserialize_str(IntegerStrVisitor::<T> {
            _target: PhantomData,
        })
    }

    pub fn serialize<T: std::fmt::Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
}

pub mod integer_str_opt {
    use serde::{
        de::{Deserializer, Visitor},
        ser::Serializer,
    };
    use std::{marker::PhantomData, str::FromStr};

    const EXPECTED: &str = "optional integer string";

    pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error> {
        struct IntegerStrOptVisitor<T> {
            _target: PhantomData<T>,
        }

        impl<'de, T: FromStr> Visitor<'de> for IntegerStrOptVisitor<T> {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(EXPECTED)
            }

            fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                super::integer_str::deserialize(deserializer).map(Some)
            }
        }

        deserializer.deserialize_option(IntegerStrOptVisitor::<T> {
            _target: PhantomData,
        })
    }

    pub fn serialize<T: std::fmt::Display, S: Serializer>(
        value: &Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.serialize_str(&value.to_string()),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};

    const SAMPLE_TEXT_TIMESTAMP: &str = "Thu Jun 25 16:18:41 +0000 2009";
    const SAMPLE_EPOCH_S: i64 = 1245946721;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct TimestampedData {
        #[serde(with = "super::text_timestamp")]
        value: DateTime<Utc>,
    }

    #[test]
    fn deserialize_text_timestamp() {
        let json = format!(r#"{{"value":"{}"}}"#, SAMPLE_TEXT_TIMESTAMP);
        let expected = TimestampedData {
            value: Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap(),
        };

        assert_eq!(
            serde_json::from_str::<TimestampedData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_text_timestamp() {
        let value = TimestampedData {
            value: Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap(),
        };
        let expected = format!(r#"{{"value":"{}"}}"#, SAMPLE_TEXT_TIMESTAMP);

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrData {
        #[serde(with = "super::integer_str")]
        value: u64,
    }

    #[test]
    fn deserialize_integer_str() {
        let json = format!(r#"{{"value":"{}"}}"#, 123);
        let expected = IntegerStrData { value: 123 };

        assert_eq!(
            serde_json::from_str::<IntegerStrData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_integer_str() {
        let value = IntegerStrData { value: 123 };
        let expected = format!(r#"{{"value":"{}"}}"#, 123);

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrOptData {
        #[serde(
            with = "super::integer_str_opt",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        value: Option<u64>,
    }

    #[test]
    fn deserialize_some_integer_str_opt() {
        let json = format!(r#"{{"value":"{}"}}"#, 123);
        let expected = IntegerStrOptData { value: Some(123) };

        assert_eq!(
            serde_json::from_str::<IntegerStrOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_some_integer_str_opt() {
        let value = IntegerStrOptData { value: Some(123) };
        let expected = format!(r#"{{"value":"{}"}}"#, 123);

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[test]
    fn deserialize_missing_integer_str_opt() {
        let json = "{}";
        let expected = IntegerStrOptData { value: None };

        assert_eq!(
            serde_json::from_str::<IntegerStrOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_null_integer_str_opt() {
        let json = r#"{"value":null}"#;
        let expected = IntegerStrOptData { value: None };

        assert_eq!(
            serde_json::from_str::<IntegerStrOptData>(&json).unwrap(),
            expected
        );
    }
    #[test]
    fn serialize_none_integer_str_opt() {
        let value = IntegerStrOptData { value: None };
        let expected = "{}";

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }
}

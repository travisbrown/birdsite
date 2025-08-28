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

pub mod timestamp_msec_opt {
    use chrono::{DateTime, Utc};
    use serde::{
        de::Deserialize,
        ser::{Serialize, Serializer},
    };
    pub fn deserialize<'de, D: serde::de::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<DateTime<Utc>>, D::Error> {
        let timestamp_msec = Option::<u64>::deserialize(deserializer)?;
        let timestamp = timestamp_msec
            .map(|timestamp_msec| {
                timestamp_msec
                    .try_into()
                    .ok()
                    .and_then(DateTime::from_timestamp_millis)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Unsigned(timestamp_msec),
                            &"optional epoch millisecond",
                        )
                    })
            })
            .map_or(Ok(None), |v| v.map(Some))?;

        Ok(timestamp)
    }

    pub fn serialize<S: Serializer>(
        value: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Option::<i64>::serialize(&value.map(|value| value.timestamp_millis()), serializer)
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

/// Deserialize an array of integer strings into a collection of integers (and the reverse).
pub mod integer_str_array {
    use serde::{
        de::{Deserializer, Visitor},
        ser::Serializer,
    };
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::str::FromStr;

    const EXPECTED: &str = "integer string array";

    pub fn deserialize<'de, E: FromStr, T: FromIterator<E>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        struct IntegerStrArrayVisitor<E, T> {
            _element: PhantomData<E>,
            _target: PhantomData<T>,
        }

        impl<'de, E: FromStr, T: FromIterator<E>> Visitor<'de> for IntegerStrArrayVisitor<E, T> {
            type Value = T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(EXPECTED)
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut error = std::cell::OnceCell::new();

                let wrapper: super::IntegerStrArraySeqAccessWrapper<'de, '_, A, E> =
                    super::IntegerStrArraySeqAccessWrapper {
                        underlying: seq,
                        error: &mut error,
                        _element: PhantomData,
                    };

                let result = T::from_iter(wrapper);

                match error.take() {
                    Some(error) => Err(error),
                    None => Ok(result),
                }
            }
        }

        deserializer.deserialize_seq(IntegerStrArrayVisitor::<E, T> {
            _element: PhantomData,
            _target: PhantomData,
        })
    }

    pub fn serialize<'a, E: std::fmt::Display, T: 'a, S: Serializer>(
        values: &'a T,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        &'a T: IntoIterator<Item = E>,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        for value in values {
            seq.serialize_element(&value.to_string())?;
        }

        seq.end()
    }
}

pub mod integer_str_array_opt {
    use serde::{
        de::{Deserializer, Visitor},
        ser::Serializer,
    };
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::str::FromStr;

    const EXPECTED: &str = "optional integer string array";

    pub fn deserialize<'de, E: FromStr, T: FromIterator<E>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error> {
        struct IntegerStrArrayOptVisitor<E, T> {
            _element: PhantomData<E>,
            _target: PhantomData<T>,
        }

        impl<'de, E: FromStr, T: FromIterator<E>> Visitor<'de> for IntegerStrArrayOptVisitor<E, T> {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(EXPECTED)
            }

            fn visit_none<EE: serde::de::Error>(self) -> Result<Self::Value, EE> {
                Ok(None)
            }

            fn visit_some<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                super::integer_str_array::deserialize(deserializer).map(Some)
            }
        }

        deserializer.deserialize_option(IntegerStrArrayOptVisitor::<E, T> {
            _element: PhantomData,
            _target: PhantomData,
        })
    }

    pub fn serialize<'a, E: std::fmt::Display, T: 'a, S: Serializer>(
        values: &'a Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        &'a T: IntoIterator<Item = E>,
    {
        match values {
            Some(values) => super::integer_str_array::serialize(values, serializer),
            None => serializer.serialize_none(),
        }
    }
}

const INTEGER_STR_ARRAY_ELEMENT_EXPECTED: &str = "integer string";

struct IntegerStrArraySeqAccessWrapper<'de, 'a, A: serde::de::SeqAccess<'de>, E> {
    underlying: A,
    error: &'a mut std::cell::OnceCell<A::Error>,
    _element: std::marker::PhantomData<E>,
}

impl<'de, 'a, A: serde::de::SeqAccess<'de>, E: std::str::FromStr> IntoIterator
    for IntegerStrArraySeqAccessWrapper<'de, 'a, A, E>
{
    type Item = E;
    type IntoIter = IntegerStrArraySeqAccessIterator<'de, 'a, A, E>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { source: self }
    }
}

struct IntegerStrArraySeqAccessIterator<'de, 'a, A: serde::de::SeqAccess<'de>, E> {
    source: IntegerStrArraySeqAccessWrapper<'de, 'a, A, E>,
}

impl<'de, 'a, A: serde::de::SeqAccess<'de>, E: std::str::FromStr> Iterator
    for IntegerStrArraySeqAccessIterator<'de, 'a, A, E>
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.error.get().is_some() {
            None
        } else {
            match self.source.underlying.next_element::<&str>() {
                Ok(Some(value)) => match value.parse() {
                    Ok(value) => Some(value),
                    Err(_) => {
                        // We've just checked whether the cell is initialized.
                        self.source
                            .error
                            .set(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(value),
                                &INTEGER_STR_ARRAY_ELEMENT_EXPECTED,
                            ))
                            .unwrap();
                        None
                    }
                },
                Ok(None) => None,
                Err(error) => {
                    // We've just checked whether the cell is initialized.
                    self.source.error.set(error).unwrap();
                    None
                }
            }
        }
    }
}

/// An optional unsigned integer representation where `-1` indicates absence.
pub mod usize_opt {
    use serde::{
        de::{Deserializer, Unexpected, Visitor},
        ser::Serializer,
    };

    const EXPECTED: &str = "optional unsigned integer";

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<usize>, D::Error> {
        struct UsizeOptVisitor;

        impl<'de> Visitor<'de> for UsizeOptVisitor {
            type Value = Option<usize>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(EXPECTED)
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                v.try_into()
                    .map_err(|_| E::invalid_value(Unexpected::Unsigned(v), &EXPECTED))
                    .map(Some)
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
                if v == -1 {
                    Ok(None)
                } else {
                    Err(E::invalid_value(Unexpected::Signed(v), &EXPECTED))
                }
            }
        }

        deserializer.deserialize_any(UsizeOptVisitor)
    }

    pub fn serialize<S: Serializer>(
        value: &Option<usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.serialize_u64(*value as u64),
            None => serializer.serialize_i64(-1),
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

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrArrayData {
        #[serde(with = "super::integer_str_array")]
        values: Vec<u64>,
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct IntegerStrArrayOptData {
        #[serde(
            with = "super::integer_str_array_opt",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        values: Option<Vec<u64>>,
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

    #[test]
    fn deserialize_integer_str_array() {
        let json = r#"{"values":["123", "456"]}"#;
        let expected = IntegerStrArrayData {
            values: vec![123, 456],
        };

        assert_eq!(
            serde_json::from_str::<IntegerStrArrayData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_integer_str_array() {
        let value = IntegerStrArrayData {
            values: vec![123, 456],
        };
        let expected = r#"{"values":["123","456"]}"#;

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[test]
    fn deserialize_invalid_integer_str_array() {
        let invalid_type_json = r#"{"values":["123", 987, "456"]}"#;
        let invalid_value_json = r#"{"values":["123", "abc", "456"]}"#;

        let invalid_type_result = serde_json::from_str::<IntegerStrArrayData>(&invalid_type_json);
        let invalid_value_result = serde_json::from_str::<IntegerStrArrayData>(&invalid_value_json);

        assert!(invalid_type_result.is_err());
        assert!(invalid_value_result.is_err());
    }

    #[test]
    fn deserialize_integer_str_array_opt() {
        let json = r#"{"values":["123", "456"]}"#;
        let expected = IntegerStrArrayOptData {
            values: Some(vec![123, 456]),
        };

        assert_eq!(
            serde_json::from_str::<IntegerStrArrayOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_integer_str_array_opt() {
        let value = IntegerStrArrayOptData {
            values: Some(vec![123, 456]),
        };
        let expected = r#"{"values":["123","456"]}"#;

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct UsizeOptData {
        #[serde(with = "super::usize_opt")]
        value_a: Option<usize>,
        #[serde(with = "super::usize_opt")]
        value_b: Option<usize>,
    }

    #[test]
    fn deserialize_u64_opt() {
        let json = r#"{"value_a":123,"value_b":-1}"#;
        let expected = UsizeOptData {
            value_a: Some(123),
            value_b: None,
        };

        assert_eq!(
            serde_json::from_str::<UsizeOptData>(&json).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_u64_opt() {
        let value = UsizeOptData {
            value_a: Some(123),
            value_b: None,
        };
        let expected = r#"{"value_a":123,"value_b":-1}"#;

        assert_eq!(serde_json::json!(value).to_string(), expected);
    }
}

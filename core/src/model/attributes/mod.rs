/// Serde attribute for numeric X (Twitter) IDs that may be encoded as either a JSON number or a
/// decimal string.
pub mod id {
    use serde::{
        de::{Deserializer, Unexpected, Visitor},
        ser::Serializer,
    };

    struct IdVisitor;

    impl Visitor<'_> for IdVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("u64 or u64 string")
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse::<u64>().map_err(|_| {
                serde::de::Error::invalid_value(Unexpected::Str(v), &"u64 or u64 string")
            })
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        deserializer.deserialize_any(IdVisitor)
    }

    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(*value)
    }
}

/// Optional variant of [`id`].
pub mod optional_id {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    struct Id(u64);

    impl<'de> Deserialize<'de> for Id {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            super::id::deserialize(deserializer).map(Self)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        Option::<Id>::deserialize(deserializer).map(|id| id.map(|id| id.0))
    }

    pub fn serialize<S: Serializer>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error> {
        Option::<u64>::serialize(value, serializer)
    }
}

/// Serde attribute for numeric X (Twitter) IDs that are guaranteed to be encoded as decimal
/// strings (`id_str`, `rest_id`, and friends).
///
/// Unlike [`id`], this attribute rejects JSON numbers on deserialization, so a field's choice of
/// attribute documents the wire shape: [`id`] means either encoding may appear, while this one
/// means the value is always a string.
pub mod id_str {
    use serde::{
        de::{Deserializer, Unexpected, Visitor},
        ser::Serializer,
    };

    struct IdStrVisitor;

    impl Visitor<'_> for IdStrVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("u64 string")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse::<u64>()
                .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &"u64 string"))
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        deserializer.deserialize_str(IdStrVisitor)
    }

    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }
}

/// Optional variant of [`id_str`].
pub mod optional_id_str {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    struct Id(u64);

    impl<'de> Deserialize<'de> for Id {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            super::id_str::deserialize(deserializer).map(Self)
        }
    }

    impl Serialize for Id {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            super::id_str::serialize(&self.0, serializer)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        Option::<Id>::deserialize(deserializer).map(|id| id.map(|id| id.0))
    }

    pub fn serialize<S: Serializer>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error> {
        value.map(Id).serialize(serializer)
    }
}

/// Serde attribute for sequences of numeric X (Twitter) IDs whose elements may be encoded as
/// either JSON numbers or decimal strings.
///
/// Deserialization always produces an owned vector (a JSON array has no zero-copy `[u64]`
/// representation); the `Cow` allows serialization to borrow an existing slice.
pub mod ids {
    use serde::{
        de::{Deserializer, Visitor},
        ser::{SerializeSeq, Serializer},
    };
    use std::borrow::Cow;

    struct IdsVisitor;

    impl<'de> Visitor<'de> for IdsVisitor {
        type Value = Cow<'static, [u64]>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("sequence of u64s or u64 strings")
        }

        fn visit_seq<A: serde::de::SeqAccess<'de>>(
            self,
            mut seq: A,
        ) -> Result<Self::Value, A::Error> {
            let mut values = seq.size_hint().map_or_else(Vec::new, Vec::with_capacity);

            while let Some(next) = seq.next_element_seed(IdSeed)? {
                values.push(next);
            }

            Ok(Cow::Owned(values))
        }
    }

    struct IdSeed;

    impl<'de> serde::de::DeserializeSeed<'de> for IdSeed {
        type Value = u64;

        fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<u64, D::Error> {
            super::id::deserialize(deserializer)
        }
    }

    pub fn deserialize<'de, 'a, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Cow<'a, [u64]>, D::Error> {
        deserializer.deserialize_seq(IdsVisitor)
    }

    pub fn serialize<S: Serializer>(value: &[u64], serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for id in value {
            seq.serialize_element(id)?;
        }

        seq.end()
    }
}

/// Optional variant of [`ids`].
pub mod optional_ids {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };
    use std::borrow::Cow;

    struct Ids(Vec<u64>);

    impl<'de> Deserialize<'de> for Ids {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            super::ids::deserialize(deserializer).map(|ids| Self(ids.into_owned()))
        }
    }

    struct SerializeIds<'a>(&'a [u64]);

    impl Serialize for SerializeIds<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            super::ids::serialize(self.0, serializer)
        }
    }

    pub fn deserialize<'de, 'a, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Cow<'a, [u64]>>, D::Error> {
        Option::<Ids>::deserialize(deserializer).map(|ids| ids.map(|ids| Cow::Owned(ids.0)))
    }

    pub fn serialize<S: Serializer>(
        value: &Option<Cow<'_, [u64]>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value
            .as_ref()
            .map(|ids| SerializeIds(ids.as_ref()))
            .serialize(serializer)
    }
}

/// Serde attribute for optional sequences of numeric X (Twitter) IDs whose elements are
/// guaranteed to be encoded as decimal strings.
///
/// Like [`id_str`], this attribute rejects JSON numbers on deserialization, documenting that the
/// elements are always strings; use [`optional_ids`] where either encoding may appear.
pub mod optional_ids_str {
    use serde::{
        de::{Deserialize, Deserializer, Visitor},
        ser::{Serialize, SerializeSeq, Serializer},
    };
    use std::borrow::Cow;

    struct IdsVisitor;

    impl<'de> Visitor<'de> for IdsVisitor {
        type Value = Vec<u64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("sequence of u64 strings")
        }

        fn visit_seq<A: serde::de::SeqAccess<'de>>(
            self,
            mut seq: A,
        ) -> Result<Self::Value, A::Error> {
            let mut values = seq.size_hint().map_or_else(Vec::new, Vec::with_capacity);

            while let Some(next) = seq.next_element_seed(IdSeed)? {
                values.push(next);
            }

            Ok(values)
        }
    }

    struct IdSeed;

    impl<'de> serde::de::DeserializeSeed<'de> for IdSeed {
        type Value = u64;

        fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<u64, D::Error> {
            super::id_str::deserialize(deserializer)
        }
    }

    struct Ids(Vec<u64>);

    impl<'de> Deserialize<'de> for Ids {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_seq(IdsVisitor).map(Self)
        }
    }

    struct IdStr(u64);

    impl Serialize for IdStr {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            super::id_str::serialize(&self.0, serializer)
        }
    }

    struct SerializeIds<'a>(&'a [u64]);

    impl Serialize for SerializeIds<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for id in self.0 {
                seq.serialize_element(&IdStr(*id))?;
            }

            seq.end()
        }
    }

    pub fn deserialize<'de, 'a, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Cow<'a, [u64]>>, D::Error> {
        Option::<Ids>::deserialize(deserializer).map(|ids| ids.map(|ids| Cow::Owned(ids.0)))
    }

    pub fn serialize<S: Serializer>(
        value: &Option<Cow<'_, [u64]>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value
            .as_ref()
            .map(|ids| SerializeIds(ids.as_ref()))
            .serialize(serializer)
    }
}

/// Serde attribute for counts that are guaranteed to be encoded as decimal strings.
///
/// Like [`id_str`], this attribute rejects JSON numbers on deserialization, documenting that the
/// value is always a string on the wire.
pub mod count_str {
    use serde::{
        de::{Deserializer, Unexpected, Visitor},
        ser::Serializer,
    };

    struct CountStrVisitor;

    impl Visitor<'_> for CountStrVisitor {
        type Value = usize;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("usize string")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse::<usize>()
                .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &"usize string"))
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
        deserializer.deserialize_str(CountStrVisitor)
    }

    pub fn serialize<S: Serializer>(value: &usize, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }
}

/// Optional variant of [`count_str`].
pub mod optional_count_str {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    struct Count(usize);

    impl<'de> Deserialize<'de> for Count {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            super::count_str::deserialize(deserializer).map(Self)
        }
    }

    impl Serialize for Count {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            super::count_str::serialize(&self.0, serializer)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<usize>, D::Error> {
        Option::<Count>::deserialize(deserializer).map(|count| count.map(|count| count.0))
    }

    pub fn serialize<S: Serializer>(
        value: &Option<usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.map(Count).serialize(serializer)
    }
}

/// Serde attribute for optional counts where `-1` on the wire indicates absence.
///
/// Deserialization maps `-1` to `None` and any unsigned value to `Some`; any other negative
/// value is rejected. Serialization writes `None` back as `-1`.
pub mod optional_count_with_sentinel {
    use serde::{
        de::{Deserializer, Unexpected, Visitor},
        ser::Serializer,
    };

    const EXPECTED: &str = "optional unsigned integer";

    struct CountVisitor;

    impl Visitor<'_> for CountVisitor {
        type Value = Option<usize>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<usize>, D::Error> {
        deserializer.deserialize_any(CountVisitor)
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

/// Reads and writes an ISO 8601 timestamp at the millisecond precision the v2 API always uses.
pub mod millisecond_timestamp {
    use crate::model::timestamp::MillisecondTimestamp;
    use chrono::{DateTime, Utc};
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        MillisecondTimestamp::deserialize(deserializer).map(|timestamp| timestamp.0)
    }

    pub fn serialize<S: Serializer>(
        value: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        MillisecondTimestamp::serialize(&MillisecondTimestamp(*value), serializer)
    }
}

/// Optional variant of [`text_timestamp`].
pub mod optional_text_timestamp {
    use crate::model::timestamp::TextTimestamp;
    use chrono::{DateTime, Utc};
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<DateTime<Utc>>, D::Error> {
        Option::<TextTimestamp>::deserialize(deserializer)
            .map(|text_timestamp| text_timestamp.map(|text_timestamp| text_timestamp.0))
    }

    pub fn serialize<S: Serializer>(
        value: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Option::<TextTimestamp>::serialize(&value.map(TextTimestamp), serializer)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};
    use std::borrow::Cow;

    const SAMPLE_TEXT_TIMESTAMP: &str = "Thu Jun 25 16:18:41 +0000 2009";
    const SAMPLE_EPOCH_S: i64 = 1_245_946_721;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Data<'a> {
        #[serde(with = "super::id")]
        id: u64,
        #[serde(with = "super::id_str")]
        id_str: u64,
        #[serde(with = "super::optional_id_str")]
        rest_id: Option<u64>,
        #[serde(with = "super::ids", borrow)]
        ids: Cow<'a, [u64]>,
        #[serde(with = "super::optional_ids_str")]
        ids_str: Option<Cow<'a, [u64]>>,
        #[serde(with = "super::optional_text_timestamp")]
        timestamp: Option<DateTime<Utc>>,
        #[serde(with = "super::optional_count_with_sentinel")]
        count: Option<usize>,
        #[serde(with = "super::count_str")]
        count_str: usize,
        #[serde(with = "super::optional_count_str")]
        media_count: Option<usize>,
    }

    fn sample_data() -> Data<'static> {
        Data {
            id: 12_345_678,
            id_str: 12_345_678,
            rest_id: Some(23_456_789),
            ids: Cow::Borrowed(&[12_345_678, 23_456_789]),
            ids_str: Some(Cow::Borrowed(&[12_345_678, 23_456_789])),
            timestamp: Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single(),
            count: Some(42),
            count_str: 7,
            media_count: Some(9),
        }
    }

    #[test]
    fn deserialize_attributes() {
        let json = format!(
            r#"{{"id":"12345678","id_str":"12345678","rest_id":"23456789","ids":[12345678,"23456789"],"ids_str":["12345678","23456789"],"timestamp":"{SAMPLE_TEXT_TIMESTAMP}","count":42,"count_str":"7","media_count":"9"}}"#
        );

        assert_eq!(
            serde_json::from_str::<Data<'_>>(&json).unwrap(),
            sample_data()
        );

        assert_eq!(
            serde_json::from_str::<Data<'_>>(
                r#"{"id":12345678,"id_str":"12345678","rest_id":null,"ids":[],"ids_str":null,"timestamp":null,"count":-1,"count_str":"7","media_count":null}"#
            )
            .unwrap(),
            Data {
                id: 12_345_678,
                id_str: 12_345_678,
                rest_id: None,
                ids: Cow::Borrowed(&[]),
                ids_str: None,
                timestamp: None,
                count: None,
                count_str: 7,
                media_count: None,
            }
        );
    }

    #[test]
    fn deserialize_id_str_rejects_numbers() {
        assert!(
            serde_json::from_str::<Data<'_>>(
                r#"{"id":12345678,"id_str":12345678,"rest_id":null,"ids":[],"ids_str":null,"timestamp":null,"count":-1,"count_str":"7","media_count":null}"#
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<Data<'_>>(
                r#"{"id":12345678,"id_str":"12345678","rest_id":23456789,"ids":[],"ids_str":null,"timestamp":null,"count":-1,"count_str":"7","media_count":null}"#
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<Data<'_>>(
                r#"{"id":12345678,"id_str":"12345678","rest_id":null,"ids":[],"ids_str":[23456789],"timestamp":null,"count":-1}"#
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<Data<'_>>(
                r#"{"id":12345678,"id_str":"12345678","rest_id":null,"ids":[],"ids_str":null,"timestamp":null,"count":-1,"count_str":7,"media_count":null}"#
            )
            .is_err()
        );

        assert!(
            serde_json::from_str::<Data<'_>>(
                r#"{"id":12345678,"id_str":"12345678","rest_id":null,"ids":[],"ids_str":null,"timestamp":null,"count":-1,"count_str":"7","media_count":9}"#
            )
            .is_err()
        );
    }

    #[test]
    fn serialize_attributes() {
        let expected = format!(
            r#"{{"id":12345678,"id_str":"12345678","rest_id":"23456789","ids":[12345678,23456789],"ids_str":["12345678","23456789"],"timestamp":"{SAMPLE_TEXT_TIMESTAMP}","count":42,"count_str":"7","media_count":"9"}}"#
        );

        assert_eq!(serde_json::json!(sample_data()).to_string(), expected);
    }
}

pub mod color;
pub mod country;
pub mod entity;
pub mod lang;
pub mod probability;
pub mod snowflake;
pub mod time_zone;
pub mod timestamp;
pub mod user;

/// Decode a range from a pair of values.
pub mod indices {
    use std::ops::Range;

    pub fn deserialize<'de, T: serde::de::Deserialize<'de>, D: serde::de::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Range<T>, D::Error> {
        let (start, end) = serde::de::Deserialize::deserialize(deserializer)?;

        Ok(start..end)
    }

    pub fn serialize<T: serde::ser::Serialize, S: serde::ser::Serializer>(
        value: &Range<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&value.start)?;
        tuple.serialize_element(&value.end)?;
        tuple.end()
    }
}

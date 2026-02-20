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

pub mod optional_text_timestamp {
    use crate::model::timestamp::TextTimestamp;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

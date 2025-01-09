pub mod created_at {
    use crate::timestamp::CreatedAt;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        CreatedAt::deserialize(deserializer).map(|created_at| created_at.0)
    }

    pub fn serialize<S: Serializer>(
        value: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        CreatedAt::serialize(&CreatedAt(*value), serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::created_at;
    use chrono::{DateTime, TimeZone, Utc};
    use serde_json::json;

    const SAMPLE_CREATED_AT: &str = "Thu Jun 25 16:18:41 +0000 2009";
    const SAMPLE_EPOCH_S: i64 = 1245946721;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Data {
        #[serde(with = "created_at")]
        value: DateTime<Utc>,
    }

    #[test]
    fn deserialize_created_at() {
        let json = format!(r#"{{"value":"{}"}}"#, SAMPLE_CREATED_AT);
        let expected = Data {
            value: Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap(),
        };

        assert_eq!(serde_json::from_str::<Data>(&json).unwrap(), expected);
    }

    #[test]
    fn serialize_created_at() {
        let value = Data {
            value: Utc.timestamp_opt(SAMPLE_EPOCH_S, 0).single().unwrap(),
        };
        let expected = format!(r#"{{"value":"{}"}}"#, SAMPLE_CREATED_AT);

        assert_eq!(json!(value).to_string(), expected);
    }
}

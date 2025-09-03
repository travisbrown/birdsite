use super::{Endpoint, TransactionId};
use chrono::DateTime;
use std::borrow::Cow;
use std::collections::HashMap;

/// A simple store for client transaction IDs.
///
/// The store tracks ID values and generation times.
pub struct Store {
    values: HashMap<Endpoint<'static>, TransactionId>,
}

impl Store {
    pub fn lookup<'a>(&self, endpoint: &Endpoint<'a>) -> Option<&TransactionId> {
        self.values.get(&Self::static_endpoint(endpoint))
    }

    pub fn add<'a>(
        &mut self,
        endpoint: &Endpoint<'a>,
        transaction_id: TransactionId,
    ) -> Option<TransactionId> {
        self.values
            .insert(Self::static_endpoint(endpoint), transaction_id)
    }

    pub fn invalidate<'a>(&mut self, endpoint: &Endpoint<'a>) -> Option<TransactionId> {
        self.values.remove(&Self::static_endpoint(endpoint))
    }

    fn static_endpoint<'a>(endpoint: &Endpoint<'a>) -> Endpoint<'static> {
        Endpoint {
            name: endpoint.name.to_string().into(),
            version: endpoint.version.to_string().into(),
        }
    }

    pub fn read_csv<R: std::io::Read>(reader: R) -> Result<Self, csv::Error> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader);

        let values = reader
            .deserialize::<Row>()
            .map(|row| {
                let Row {
                    name,
                    version,
                    timestamp_s,
                    value,
                } = row?;

                Ok((
                    Endpoint::new(name, version),
                    TransactionId {
                        value,
                        animation_key: None,
                        timestamp: DateTime::from_timestamp(timestamp_s, 0).ok_or_else(|| {
                            csv::Error::from(std::io::Error::other("epoch timestamp"))
                        })?,
                    },
                ))
            })
            .collect::<Result<_, csv::Error>>()?;

        Ok(Self { values })
    }

    pub fn write_csv<W: std::io::Write>(&self, writer: W) -> Result<(), csv::Error> {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(writer);

        let mut rows = self
            .values
            .iter()
            .map(|(endpoint, transaction_id)| Row {
                name: endpoint.name.to_string(),
                version: endpoint.version.to_string(),
                timestamp_s: transaction_id.timestamp.timestamp(),
                value: transaction_id.value.to_string(),
            })
            .collect::<Vec<_>>();

        rows.sort();

        for row in rows {
            writer.serialize(row)?;
        }

        Ok(writer.flush()?)
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
struct Row {
    name: String,
    version: String,
    timestamp_s: i64,
    value: String,
}

impl<'de> serde::de::Deserialize<'de> for Store {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        let values: Vec<(String, String, i64, String)> =
            serde::de::Deserialize::deserialize(deserializer)?;

        Ok(Self {
            values: values
                .into_iter()
                .map(|(name, version, timestamp_s, value)| {
                    Ok((
                        Endpoint::new(Cow::from(name), Cow::from(version)),
                        TransactionId {
                            value,
                            animation_key: None,
                            timestamp: DateTime::from_timestamp(timestamp_s, 0).ok_or_else(
                                || {
                                    D::Error::invalid_value(
                                        serde::de::Unexpected::Signed(timestamp_s),
                                        &"epoch second",
                                    )
                                },
                            )?,
                        },
                    ))
                })
                .collect::<Result<_, D::Error>>()?,
        })
    }
}

impl serde::ser::Serialize for Store {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let mut values = self
            .values
            .iter()
            .map(|(endpoint, transaction_id)| {
                (
                    endpoint.name.to_string(),
                    endpoint.version.to_string(),
                    transaction_id.timestamp.timestamp(),
                    transaction_id.value.clone(),
                )
            })
            .collect::<Vec<_>>();

        values.sort();

        let mut seq = serializer.serialize_seq(Some(self.values.len()))?;

        for (name, version, timestamp, value) in values {
            seq.serialize_element(&(name, version, timestamp, value))?;
        }

        seq.end()
    }
}

#[cfg(test)]
mod test {
    use super::Store;
    use base64::prelude::*;

    #[test]
    fn round_trip() {
        let value_0 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(b"test0");
        let value_1 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(b"test1");

        let timestamp_0 = chrono::Utc::now();
        let timestamp_1 = chrono::Utc::now();

        let content = format!(
            "UserByRestId,8r5oa_2vD0WkhIAOkY4TTA,{},{}\nUserTweetsAndReplies,zrnbkQ-daS4clwTMtp6C1w,{},{}\n",
            timestamp_0.timestamp(),
            value_0,
            timestamp_1.timestamp(),
            value_1
        );

        let parsed_store = Store::read_csv(content.as_bytes()).unwrap();

        let mut output = Vec::new();

        parsed_store.write_csv(&mut output).unwrap();

        assert_eq!(content.as_bytes(), output);
    }
}

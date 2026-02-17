use crate::request::name::RequestName;
use chrono::{DateTime, Utc};
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct Request<'a, V> {
    pub name: RequestName,
    pub version: Option<Cow<'a, str>>,
    pub timestamp: DateTime<Utc>,
    pub variables: Option<V>,
}

pub trait Variables<'a> {
    fn parse_with_name<'de: 'a, A: serde::de::MapAccess<'de>>(
        name: RequestName,
        map: &mut A,
    ) -> Option<Result<Self, A::Error>>
    where
        Self: Sized;
}

impl<'a, 'de: 'a, V: Variables<'a> + 'a> serde::de::Deserialize<'de> for Request<'a, V> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RequestVisitor<'a, V> {
            _phantom: PhantomData<&'a V>,
        }

        impl<'a, 'de: 'a, V: Variables<'a> + 'a> serde::de::Visitor<'de> for RequestVisitor<'a, V> {
            type Value = Request<'a, V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct Request")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                RequestField::Name.map_key(&mut map)?;

                let name = map.next_value::<RequestName>()?;

                let version = match map.next_key::<RequestField>()? {
                    Some(RequestField::Version) => {
                        let version = map.next_value::<&str>()?;

                        RequestField::TimestampMs.map_key(&mut map)?;

                        Ok(Some(version))
                    }
                    Some(RequestField::TimestampMs) => Ok(None),
                    _ => Err(serde::de::Error::missing_field(
                        RequestField::TimestampMs.name(),
                    )),
                }?;

                let timestamp_str = map.next_value::<&str>()?;
                let timestamp = timestamp_str
                    .parse::<i64>()
                    .ok()
                    .and_then(DateTime::from_timestamp_millis)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(timestamp_str),
                            &"millisecond timestamp string",
                        )
                    })?;

                RequestField::Variables.map_key(&mut map)?;

                let variables = V::parse_with_name(name, &mut map)
                    .map_or_else(|| Ok(None), |result| result.map(Some))?;

                Ok(Self::Value {
                    name,
                    version: version.map(std::convert::Into::into),
                    timestamp,
                    variables,
                })
            }
        }

        deserializer.deserialize_map(RequestVisitor {
            _phantom: PhantomData,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
enum RequestField {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "version")]
    Version,
    #[serde(rename = "timestamp_ms")]
    TimestampMs,
    #[serde(rename = "variables")]
    Variables,
}

impl RequestField {
    const fn name(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Version => "version",
            Self::TimestampMs => "timestamp_ms",
            Self::Variables => "variables",
        }
    }

    fn map_key<'de, A: serde::de::MapAccess<'de>>(self, map: &mut A) -> Result<Self, A::Error> {
        map.next_key::<Self>().and_then(|field| {
            field
                .filter(|field| *field == self)
                .ok_or_else(|| serde::de::Error::missing_field(self.name()))
        })
    }
}

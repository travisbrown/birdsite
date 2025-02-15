use chrono::{DateTime, Utc};
use serde::{de::Unexpected, ser::SerializeSeq};
use std::borrow::Cow;

pub mod cashtag;
pub mod color;
pub mod country;
pub mod entities;
pub mod lang;
pub mod media;
pub mod probability;
pub mod source;
pub mod time_zone;
pub mod timestamp;
pub mod wbm;

const U64_STRING_NAME: &str = "u64 string";

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    pub url: Cow<'a, str>,
    pub expanded: Cow<'a, str>,
    pub display: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Place<'a> {
    #[serde(flatten, borrow)]
    pub metadata: PlaceMetadata<'a>,
    pub url: Cow<'a, str>,
    pub bounding_box: Option<BoundingBox>,
    pub attributes: PlaceAttributes,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceMetadata<'a> {
    pub id: &'a str,
    pub place_type: PlaceType,
    pub name: Cow<'a, str>,
    pub country_code: country::PossibleCountry,
    pub country: Cow<'a, str>,
    pub full_name: Cow<'a, str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PlaceType {
    #[serde(rename = "city")]
    City,
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "neighborhood")]
    Neighborhood,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "poi")]
    PointOfInterest,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundingBox {
    #[serde(rename = "type")]
    pub bounding_box_type: BoundingBoxType,
    // TODO: Fix this.
    pub coordinates: Vec<serde_json::Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BoundingBoxType {
    Polygon,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceAttributes {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EditControls {
    pub edits_remaining: usize,
    pub is_edit_eligible: bool,
    pub editable_until: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetPublicMetrics {
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
    pub quote_count: usize,
    pub bookmark_count: Option<usize>,
    pub impression_count: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserPublicMetrics {
    pub followers_count: usize,
    pub following_count: usize,
    pub tweet_count: usize,
    pub listed_count: usize,
    pub like_count: Option<usize>,
    pub media_count: Option<usize>,
}

/// A possible ID that is represented as a string, but may be "-1" to represent an absent value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PossibleId(pub Option<u64>);

impl<'de> serde::Deserialize<'de> for PossibleId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value: &str = serde::Deserialize::deserialize(deserializer)?;

        if value == "-1" {
            Ok(Self(None))
        } else {
            match value.parse::<u64>() {
                Ok(id) => Ok(Self(Some(id))),
                Err(_) => Err(serde::de::Error::invalid_value(
                    Unexpected::Str(value),
                    &"u64 string or \"-1\"",
                )),
            }
        }
    }
}

impl serde::ser::Serialize for PossibleId {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Some(id) => serializer.serialize_str(&id.to_string()),
            None => serializer.serialize_str("-1"),
        }
    }
}

#[derive(serde::Deserialize)]
struct PossibleCountInternal(Option<i64>);

/// An integer that is generally a count, but may be -1 to represent an absent value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PossibleCount(pub Option<usize>);

impl<'de> serde::Deserialize<'de> for PossibleCount {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match serde::Deserialize::deserialize(deserializer)? {
            PossibleCountInternal(None | Some(-1)) => Ok(Self(None)),
            PossibleCountInternal(Some(value)) => usize::try_from(value)
                .map_err(|_| {
                    serde::de::Error::invalid_value(Unexpected::Signed(value), &"usize or -1")
                })
                .map(|value| Self(Some(value))),
        }
    }
}

impl serde::ser::Serialize for PossibleCount {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Some(value) => value.serialize(serializer),
            None => serializer.serialize_i64(-1),
        }
    }
}

pub mod id_str {
    use serde::{de::Unexpected, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        let value: &str = Deserialize::deserialize(deserializer)?;
        value.parse::<u64>().map_err(|_| {
            serde::de::Error::invalid_value(Unexpected::Str(value), &super::U64_STRING_NAME)
        })
    }

    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
}

pub mod id_str_optional {
    use serde::{de::Unexpected, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        let value: Option<&str> = Deserialize::deserialize(deserializer)?;

        match value {
            Some(value) => value
                .parse::<u64>()
                .map_err(|_| {
                    serde::de::Error::invalid_value(Unexpected::Str(value), &super::U64_STRING_NAME)
                })
                .map(Some),
            None => Ok(None),
        }
    }

    pub fn serialize<S: Serializer>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.serialize_some(&value.to_string()),
            None => serializer.serialize_none(),
        }
    }
}

struct Ids<'a>(Cow<'a, [u64]>);

impl<'de: 'a, 'a> serde::Deserialize<'de> for Ids<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IdsVisitor<'a> {
            _phantom: std::marker::PhantomData<&'a ()>,
        }

        impl<'de: 'a, 'a> serde::de::Visitor<'de> for IdsVisitor<'a> {
            type Value = Ids<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Ids")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = match seq.size_hint() {
                    Some(len) => Vec::with_capacity(len),
                    None => vec![],
                };

                while let Some(next) = seq.next_element::<&str>()? {
                    let next_id = next.parse::<u64>().map_err(|_| {
                        serde::de::Error::invalid_value(Unexpected::Str(next), &U64_STRING_NAME)
                    })?;
                    values.push(next_id);
                }

                Ok(Ids(Cow::Owned(values)))
            }
        }

        deserializer.deserialize_seq(IdsVisitor {
            _phantom: std::marker::PhantomData,
        })
    }
}

impl serde::ser::Serialize for Ids<'_> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = self.0.as_ref().len();
        let mut seq = serializer.serialize_seq(Some(len))?;
        for id in self.0.iter() {
            seq.serialize_element(id)?;
        }

        seq.end()
    }
}

pub mod id_str_array {
    use super::Ids;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u64>, D::Error> {
        Ok(Ids::deserialize(deserializer)?.0.into_owned())
    }

    pub fn serialize<S: Serializer>(value: &Vec<u64>, serializer: S) -> Result<S::Ok, S::Error> {
        Ids(value.as_slice().into()).serialize(serializer)
    }
}

pub mod id_str_array_optional {
    use super::Ids;
    use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Vec<u64>>, D::Error> {
        let value: Option<Ids> = Deserialize::deserialize(deserializer)?;

        match value {
            Some(value) => Ok(Some(value.0.into_owned())),
            None => Ok(None),
        }
    }

    pub fn serialize<S: Serializer>(
        value: &Option<Vec<u64>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(values) => {
                let mut seq = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    seq.serialize_element(&value.to_string())?;
                }
                seq.end()
            }
            None => serializer.serialize_none(),
        }
    }
}

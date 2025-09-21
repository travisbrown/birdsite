use crate::model::url::{Url, UrlType};
use std::borrow::Cow;
use std::ops::Range;

/// A URL representation that uses snake case (used in the untyped entity representation).
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyUrl<'a> {
    pub url: Cow<'a, str>,
    pub url_type: UrlType,
}

impl<'a> From<&LegacyUrl<'a>> for Url<'a> {
    fn from(value: &LegacyUrl<'a>) -> Self {
        Self {
            url: value.url.clone(),
            url_type: value.url_type,
            urt_endpoint_options: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entity<'a> {
    pub indices: Range<usize>,
    pub reference: LegacyUrl<'a>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for Entity<'a> {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entity = internal::Entity::deserialize(deserializer)?;

        Ok(Self {
            indices: entity.from_index..entity.to_index,
            reference: entity.reference,
        })
    }
}

impl serde::ser::Serialize for Entity<'_> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        internal::Entity::serialize(
            &internal::Entity {
                from_index: self.indices.start,
                to_index: self.indices.end,
                reference: self.reference.clone(),
            },
            serializer,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedEntity<'a> {
    pub indices: Range<usize>,
    pub reference: TypedEntityReference<'a>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for TypedEntity<'a> {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entity = internal::TypedEntity::deserialize(deserializer)?;

        Ok(Self {
            indices: entity.from_index..entity.to_index,
            reference: entity.reference,
        })
    }
}

impl serde::ser::Serialize for TypedEntity<'_> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        internal::TypedEntity::serialize(
            &internal::TypedEntity {
                from_index: self.indices.start,
                to_index: self.indices.end,
                reference: self.reference.clone(),
            },
            serializer,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum TypedEntityReference<'a> {
    TimelineUrl {
        #[serde(flatten)]
        url: Url<'a>,
    },
    TimelineRichTextHashtag {
        text: Cow<'a, str>,
    },
    TimelineRichTextCashtag {
        text: Cow<'a, str>,
    },
}

mod internal {
    #[derive(serde::Deserialize, serde::Serialize)]
    pub(super) struct Entity<'a> {
        pub from_index: usize,
        pub to_index: usize,
        #[serde(rename = "ref")]
        pub reference: super::LegacyUrl<'a>,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub(super) struct TypedEntity<'a> {
        #[serde(rename = "fromIndex")]
        pub from_index: usize,
        #[serde(rename = "toIndex")]
        pub to_index: usize,
        #[serde(rename = "ref")]
        pub reference: super::TypedEntityReference<'a>,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_typed_entity() {
        let doc = r#"{"fromIndex":44,"toIndex":67,"ref":{"type":"TimelineUrl","url":"https://t.co/GAQgLgyG02","urlType":"ExternalUrl"}}"#;
        let expected = super::TypedEntity {
            indices: 44..67,
            reference: super::TypedEntityReference::TimelineUrl {
                url: super::Url {
                    url_type: crate::model::url::UrlType::ExternalUrl,
                    url: "https://t.co/GAQgLgyG02".into(),
                    urt_endpoint_options: None,
                },
            },
        };

        assert_eq!(
            serde_json::from_str::<super::TypedEntity<'_>>(doc).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_typed_entity() {
        let entity = super::TypedEntity {
            indices: 44..67,
            reference: super::TypedEntityReference::TimelineUrl {
                url: super::Url {
                    url_type: crate::model::url::UrlType::ExternalUrl,
                    url: "https://t.co/GAQgLgyG02".into(),
                    urt_endpoint_options: None,
                },
            },
        };
        let doc = serde_json::json!(entity).to_string();

        assert_eq!(
            serde_json::from_str::<super::TypedEntity<'_>>(&doc).unwrap(),
            entity
        );
    }
}

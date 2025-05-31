use std::borrow::Cow;
use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Url<'a> {
    #[serde(rename = "urlType")]
    pub url_type: UrlType,
    pub url: Cow<'a, str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UrlType {
    ExternalUrl,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entity<'a> {
    indices: Range<usize>,
    value: EntityValue<'a>,
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for Entity<'a> {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entity = internal::Entity::deserialize(deserializer)?;

        Ok(Self {
            indices: entity.from_index..entity.to_index,
            value: entity.reference,
        })
    }
}

impl<'a> serde::ser::Serialize for Entity<'a> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        internal::Entity::serialize(
            &internal::Entity {
                from_index: self.indices.start,
                to_index: self.indices.end,
                reference: self.value.clone(),
            },
            serializer,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum EntityValue<'a> {
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
    pub struct Entity<'a> {
        #[serde(rename = "fromIndex")]
        pub from_index: usize,
        #[serde(rename = "toIndex")]
        pub to_index: usize,
        #[serde(rename = "ref")]
        pub reference: super::EntityValue<'a>,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_entity() {
        let doc = r#"{"fromIndex":44,"toIndex":67,"ref":{"type":"TimelineUrl","url":"https://t.co/GAQgLgyG02","urlType":"ExternalUrl"}}"#;
        let expected = super::Entity {
            indices: 44..67,
            value: super::EntityValue::TimelineUrl {
                url: super::Url {
                    url_type: super::UrlType::ExternalUrl,
                    url: "https://t.co/GAQgLgyG02".into(),
                },
            },
        };

        assert_eq!(
            serde_json::from_str::<super::Entity>(doc).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_entity() {
        let entity = super::Entity {
            indices: 44..67,
            value: super::EntityValue::TimelineUrl {
                url: super::Url {
                    url_type: super::UrlType::ExternalUrl,
                    url: "https://t.co/GAQgLgyG02".into(),
                },
            },
        };
        let doc = serde_json::json!(entity).to_string();

        assert_eq!(serde_json::from_str::<super::Entity>(&doc).unwrap(), entity);
    }
}

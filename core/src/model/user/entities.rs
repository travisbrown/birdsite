use bounded_static_derive_more::ToStatic;
use std::borrow::Cow;
use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    pub expanded_url: Option<Cow<'a, str>>,
    pub url: Cow<'a, str>,
    pub display_url: Option<Cow<'a, str>>,
    #[serde(with = "crate::model::attributes::range")]
    pub indices: Range<usize>,
}

impl<'a> Url<'a> {
    pub fn url(&self) -> Cow<'a, str> {
        self.expanded_url.clone().unwrap_or(self.url.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entities<'a> {
    pub description_urls: Cow<'a, [Url<'a>]>,
    pub url: Option<Url<'a>>,
}

impl<'a> bounded_static::IntoBoundedStatic for Entities<'a> {
    type Static = Entities<'static>;

    fn into_static(self) -> Self::Static {
        Entities {
            description_urls: self
                .description_urls
                .iter()
                .map(|description_url| description_url.clone().into_static())
                .collect(),
            url: self.url.map(|url| url.into_static()),
        }
    }
}

impl<'a> bounded_static::ToBoundedStatic for Entities<'a> {
    type Static = Entities<'static>;

    fn to_static(&self) -> Self::Static {
        Entities {
            description_urls: self
                .description_urls
                .iter()
                .map(|description_url| description_url.to_static())
                .collect(),
            url: self.url.as_ref().map(|url| url.to_static()),
        }
    }
}

impl<'a, 'de: 'a> serde::de::Deserialize<'de> for Entities<'a> {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = internal::Entities::deserialize(deserializer)?;

        Ok(Entities {
            description_urls: internal.description.urls,
            url: internal.url.map(|url| url.urls.0),
        })
    }
}

impl<'a> serde::ser::Serialize for Entities<'a> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        internal::Entities::serialize(
            &internal::Entities {
                description: internal::DescriptionUrls {
                    urls: self.description_urls.clone(),
                },
                url: self.url.as_ref().map(|url| internal::UrlUrls {
                    urls: (url.clone(),),
                }),
            },
            serializer,
        )
    }
}

mod internal {
    use std::borrow::Cow;

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Entities<'a> {
        #[serde(borrow)]
        pub description: DescriptionUrls<'a>,
        #[serde(borrow)]
        pub url: Option<UrlUrls<'a>>,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct DescriptionUrls<'a> {
        #[serde(borrow)]
        pub urls: Cow<'a, [super::Url<'a>]>,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct UrlUrls<'a> {
        #[serde(borrow)]
        pub urls: (super::Url<'a>,),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_full() {
        let doc = r#"{"description":{"urls":[{"display_url":"youtube.com/user/EnglishAt…","expanded_url":"http://www.youtube.com/user/EnglishAttitude","url":"http://t.co/7PueaqzO8K","indices":[133,155]}]},"url":{"urls":[{"display_url":"englishattitude.wordpress.com","expanded_url":"http://englishattitude.wordpress.com/","url":"http://t.co/y6Df74QgMj","indices":[0,22]}]}}"#;

        let parsed: super::Entities = serde_json::from_str(doc).unwrap();

        assert_eq!(parsed.description_urls.len(), 1);
        assert_eq!(
            parsed.description_urls[0].url(),
            "http://www.youtube.com/user/EnglishAttitude"
        );
        assert!(parsed.url.is_some());
    }

    #[test]
    fn deserialize_empty() {
        let doc = r#"{"description":{"urls":[]}}"#;

        let parsed: super::Entities = serde_json::from_str(doc).unwrap();

        assert!(parsed.description_urls.is_empty());
        assert!(parsed.url.is_none());
    }
}

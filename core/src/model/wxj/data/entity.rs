use bounded_static_derive_more::ToStatic;
use serde_field_attributes::ratio_u64;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntities<'a> {
    #[serde(borrow)]
    pub description: Option<DescriptionEntities<'a>>,
    pub url: Option<Urls<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptionEntities<'a> {
    #[serde(borrow)]
    pub urls: Option<Vec<UrlDetails<'a>>>,
    pub mentions: Option<Vec<UserMention<'a>>>,
    pub hashtags: Option<Vec<Hashtag<'a>>>,
    pub cashtags: Option<Vec<Cashtag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Urls<'a> {
    #[serde(borrow)]
    pub urls: Vec<UrlDetails<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UrlDetails<'a> {
    pub url: Cow<'a, str>,
    pub display_url: Option<Cow<'a, str>>,
    pub expanded_url: Option<Cow<'a, str>>,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetEntities<'a> {
    #[serde(borrow)]
    pub annotations: Option<Vec<Annotation<'a>>>,
    pub mentions: Option<Vec<TweetMention<'a>>>,
    pub urls: Option<Vec<Url<'a>>>,
    pub hashtags: Option<Vec<Hashtag<'a>>>,
    pub cashtags: Option<Vec<Cashtag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Annotation<'a> {
    #[serde(rename = "type")]
    pub annotation_type: AnnotationType,
    // There is currently one known case where this is -1 (1814274947960709489).
    pub start: isize,
    // There is currently one known case where this is -2 (1833402059414245783).
    pub end: isize,
    #[serde(with = "ratio_u64")]
    pub probability: num_rational::Ratio<u64>,
    pub normalized_text: Cow<'a, str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AnnotationType {
    Organization,
    Person,
    Place,
    Product,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserMention<'a> {
    pub start: usize,
    pub end: usize,
    pub username: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetMention<'a> {
    pub start: usize,
    pub end: usize,
    pub username: Cow<'a, str>,
    #[serde(with = "possible_u64")]
    pub id: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    pub start: usize,
    pub end: usize,
    pub title: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub url: Cow<'a, str>,
    pub expanded_url: Option<Cow<'a, str>>,
    pub display_url: Option<Cow<'a, str>>,
    pub media_key: Option<Cow<'a, str>>,
    // TODO: Use a proper status code representation here.
    pub status: Option<usize>,
    pub unwound_url: Option<Cow<'a, str>>,
    pub images: Option<Vec<UrlImage<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UrlImage<'a> {
    pub url: Cow<'a, str>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hashtag<'a> {
    pub start: usize,
    pub end: usize,
    pub tag: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cashtag {
    pub start: usize,
    pub end: usize,
    pub tag: crate::model::cashtag::Cashtag,
}

pub mod possible_u64 {
    use serde::de::Deserializer;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error> {
        let content: &str = serde::de::Deserialize::deserialize(deserializer)?;

        if content == "-1" {
            Ok(None)
        } else {
            content
                .parse::<u64>()
                .map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(content),
                        &"u64 or -1",
                    )
                })
                .map(Some)
        }
    }

    pub fn serialize<S: serde::ser::Serializer>(
        value: &Option<u64>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.serialize_str(&value.to_string()),
            None => serializer.serialize_str("-1"),
        }
    }
}

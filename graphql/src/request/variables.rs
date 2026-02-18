use crate::request::name::RequestName;
use serde_field_attributes::{integer_or_integer_str, integer_or_integer_str_array};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AboutAccountQuery<'a> {
    #[serde(rename = "screenName")]
    pub screen_name: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BirdwatchFetchOneNote {
    #[serde(with = "integer_or_integer_str")]
    pub note_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetResultsByRestIds {
    #[serde(rename = "tweetIds", with = "integer_or_integer_str_array")]
    pub tweet_ids: Vec<u64>,
    #[serde(rename = "includePromotedContent")]
    pub include_promoted_content: bool,
    #[serde(rename = "withCommunity")]
    pub with_community: bool,
    #[serde(rename = "withVoice")]
    pub with_voice: bool,
    #[serde(rename = "withBirdwatchNotes")]
    pub with_birdwatch_notes: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Variables {
    AboutAccountQuery(AboutAccountQuery<'static>),
    BirdwatchFetchOneNote(BirdwatchFetchOneNote),
    TweetResultsByRestIds(TweetResultsByRestIds),
}

impl<'a> crate::archive::request::Variables<'a> for Variables {
    fn parse_with_name<'de: 'a, A: serde::de::MapAccess<'de>>(
        name: RequestName,
        map: &mut A,
    ) -> Option<Result<Self, A::Error>>
    where
        Self: Sized,
    {
        match name {
            RequestName::AboutAccountQuery => {
                Some(map.next_value().map(Variables::AboutAccountQuery))
            }
            RequestName::BirdwatchFetchOneNote => {
                Some(map.next_value().map(Variables::BirdwatchFetchOneNote))
            }
            RequestName::TweetResultsByRestIds => {
                Some(map.next_value().map(Variables::TweetResultsByRestIds))
            }
            _ => map
                .next_value::<serde::de::IgnoredAny>()
                .map_or_else(|error| Some(Err(error)), |_| None),
        }
    }
}

impl bounded_static::IntoBoundedStatic for Variables {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

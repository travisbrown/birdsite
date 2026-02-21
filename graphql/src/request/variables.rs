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

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Empty {}

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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MembersSliceTimelineQuery<'a> {
    #[serde(rename = "communityId", with = "integer_or_integer_str")]
    pub community_id: u64,
    pub cursor: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserByRestId {
    #[serde(rename = "userId", with = "integer_or_integer_str")]
    pub user_id: u64,
    #[serde(rename = "withSafetyModeUserFields")]
    pub with_safety_mode_user_fields: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserByScreenName<'a> {
    pub screen_name: Cow<'a, str>,
    #[serde(rename = "withSafetyModeUserFields")]
    pub with_safety_mode_user_fields: Option<bool>,
    #[serde(rename = "withGrokTranslatedBio")]
    pub with_grok_translated_bio: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UsersByRestIds {
    #[serde(rename = "userIds", with = "integer_or_integer_str_array")]
    pub user_ids: Vec<u64>,
    #[serde(rename = "withSafetyModeUserFields")]
    pub with_safety_mode_user_fields: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Variables {
    AboutAccountQuery(AboutAccountQuery<'static>),
    BirdwatchFetchOneNote(BirdwatchFetchOneNote),
    BirdwatchFetchPublicData(Empty),
    MembersSliceTimelineQuery(MembersSliceTimelineQuery<'static>),
    TweetResultsByRestIds(TweetResultsByRestIds),
    UserByRestId(UserByRestId),
    UserByScreenName(UserByScreenName<'static>),
    UsersByRestIds(UsersByRestIds),
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
            RequestName::BirdwatchFetchPublicData => {
                Some(map.next_value().map(Variables::BirdwatchFetchPublicData))
            }
            RequestName::MembersSliceTimelineQuery => {
                Some(map.next_value().map(Variables::MembersSliceTimelineQuery))
            }
            RequestName::TweetResultsByRestIds => {
                Some(map.next_value().map(Variables::TweetResultsByRestIds))
            }
            RequestName::UserByRestId => Some(map.next_value().map(Variables::UserByRestId)),
            RequestName::UserByScreenName => {
                Some(map.next_value().map(Variables::UserByScreenName))
            }
            RequestName::UsersByRestIds => Some(map.next_value().map(Variables::UsersByRestIds)),
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

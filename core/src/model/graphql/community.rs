use crate::model::graphql::ResultWrapper;
use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde_field_attributes::{integer_str, optional_integer_str};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "__typename", deny_unknown_fields)]
pub enum CommunityResult<'a, U> {
    Community {
        #[serde(flatten)]
        community: Community<'a, U>,
    },
    CommunityUnavailable {},
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Community<'a, U> {
    #[serde(rename = "id_str", with = "integer_str")]
    pub id: u64,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    pub default_theme: Option<Theme>,
    pub custom_theme: Option<Theme>,
    pub question: Option<Cow<'a, str>>,
    pub search_tags: Option<Vec<Cow<'a, str>>>,
    pub is_nsfw: Option<bool>,
    pub actions: Actions,
    pub primary_community_topic: Option<Topic<'a>>,
    pub admin_results: ResultWrapper<U>,
    pub creator_results: ResultWrapper<U>,
    pub invites_result: InvitesResult,
    pub join_policy: JoinPolicy,
    pub invites_policy: InvitesPolicy,
    pub is_pinned: bool,
    pub members_facepile_results: Vec<ResultWrapper<U>>,
    pub moderator_count: usize,
    pub member_count: usize,
    pub role: Role,
    pub rules: Vec<Rule<'a>>,
    pub custom_banner_media: Option<BannerMedia<'a>>,
    pub default_banner_media: BannerMedia<'a>,
    pub viewer_relationship: ViewerRelationship,
    pub join_requests_result: JoinRequestsResult,
    #[serde(rename = "id")]
    _internal_id: Option<Cow<'a, str>>,
    #[serde(rename = "rest_id", with = "optional_integer_str", default)]
    _rest_id: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Theme {
    Default,
    Blue,
    Green,
    Magenta,
    Orange,
    Plum,
    Purple,
    Red,
    Teal,
    Yellow,
}

/// These are interface elements we don't care about.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Actions {}

/// These are interface elements we don't care about.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InvitesResult {}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Topic<'a> {
    #[serde(with = "integer_str")]
    pub topic_id: u64,
    pub topic_name: Cow<'a, str>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum JoinPolicy {
    Open,
    RestrictedJoinRequestsDisabled,
    RestrictedJoinRequestsRequireModeratorApproval,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum InvitesPolicy {
    MemberInvitesAllowed,
    ModeratorInvitesAllowed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Role {
    NonMember,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rule<'a> {
    #[serde(with = "integer_str")]
    pub rest_id: u64,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BannerMedia<'a> {
    pub media_info: MediaInfo<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaInfo<'a> {
    pub color_info: ColorInfo,
    pub original_img_url: Cow<'a, str>,
    pub original_img_width: usize,
    pub original_img_height: usize,
    pub salient_rect: Option<crate::model::graphql::shapes::Rectangle>,
}

/// These are interface elements we don't care about.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ColorInfo {}

/// These are interface elements we don't care about.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ViewerRelationship {}

/// These are interface elements we don't care about.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct JoinRequestsResult {}

#[cfg(test)]
mod tests {
    use crate::model::graphql::user::repr::User;

    const COMMUNITIES_2024_08_01: &str =
        include_str!("../../../../examples/graphql/communities-2024-08-01.ndjson");
    const COMMUNITIES_2025_06_01: &str =
        include_str!("../../../../examples/graphql/communities-2025-06-01.ndjson");

    #[test]
    fn deserialize_examples_2024_08_01() {
        for (i, line) in COMMUNITIES_2024_08_01.split('\n').enumerate() {
            if let Err(error) = serde_json::from_str::<super::CommunityResult<'_, User<'_>>>(line) {
                panic!("Error at line {}: {:?}", i + 1, error);
            }
        }
    }

    #[test]
    fn deserialize_examples_2025_06_01() {
        for (i, line) in COMMUNITIES_2025_06_01.split('\n').enumerate() {
            if let Err(error) = serde_json::from_str::<super::CommunityResult<'_, User<'_>>>(line) {
                panic!("Error at line {}: {:?}", i + 1, error);
            }
        }
    }
}

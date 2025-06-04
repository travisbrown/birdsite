use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "__typename", deny_unknown_fields)]
pub enum CommunityResult<'a> {
    Community {
        #[serde(flatten)]
        community: Community<'a>,
    },
    CommunityUnavailable {},
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Community<'a> {
    #[serde(rename = "id_str", with = "crate::model::attributes::integer_str")]
    pub id: u64,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub invites_policy: InvitesPolicy,
    pub join_policy: JoinPolicy,
    pub default_theme: Option<Theme>,
    pub custom_theme: Option<Theme>,
    #[serde(rename = "id")]
    _internal_id: Cow<'a, str>,
    #[serde(rename = "rest_id", with = "crate::model::attributes::integer_str")]
    _rest_id: u64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum InvitesPolicy {
    MemberInvitesAllowed,
    ModeratorInvitesAllowed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum JoinPolicy {
    Open,
    RestrictedJoinRequestsDisabled,
    RestrictedJoinRequestsRequireModeratorApproval,
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

use crate::model::graphql::affiliation::AffiliationResult;
use serde::de::Deserialize;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct CommunityUser<'a> {
    pub id: u64,
    pub community_role: CommunityRole,
    pub screen_name: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
    pub affiliation_label_type: Option<AffiliationLabelType>,
    pub identity_affiliation: AffiliationResult<'a>,
    pub profile_image_url: Cow<'a, str>,
    pub protected: bool,
    pub is_blue_verified: bool,
    pub verified: bool,
    pub super_follow_eligible: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CommunityRole {
    Admin,
    Moderator,
    Member,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AffiliationLabelType {
    #[serde(rename = "AutomatedLabel")]
    Automated,
    #[serde(rename = "BusinessLabel")]
    Business,
}

impl<'de: 'a, 'a> Deserialize<'de> for CommunityUser<'a> {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let user = internal::User::deserialize(deserializer)?;

        Ok(CommunityUser {
            id: user.rest_id,
            community_role: user.community_role,
            screen_name: user.legacy.screen_name,
            name: user.legacy.name,
            affiliation_label_type: user
                .affiliates_highlighted_label
                .and_then(|label| label.label)
                .map(|label| label.user_label_type),
            identity_affiliation: user.identity_profile_labels_highlighted_label,
            protected: user.legacy.protected,
            is_blue_verified: user.is_blue_verified,
            verified: user.legacy.verified,
            profile_image_url: user.legacy.profile_image_url_https,
            super_follow_eligible: user.super_follow_eligible,
        })
    }
}

mod internal {
    use crate::model::user::properties::VerifiedType;
    use serde_field_attributes::integer_str;

    use super::CommunityRole;
    use std::borrow::Cow;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct User<'a> {
        #[serde(rename = "__typename")]
        _typename: &'a str,
        #[serde(rename = "id")]
        _id: &'a str,
        #[serde(with = "integer_str")]
        pub rest_id: u64,
        pub community_role: CommunityRole,
        #[serde(borrow)]
        pub legacy: Legacy<'a>,
        pub super_follow_eligible: bool,
        pub affiliates_highlighted_label: Option<AffiliatesHighlightedLabel>,
        pub identity_profile_labels_highlighted_label:
            crate::model::graphql::affiliation::AffiliationResult<'a>,
        pub is_blue_verified: bool,
        #[serde(rename = "super_following")]
        _super_following: Option<bool>,
        #[serde(rename = "super_followed_by")]
        _super_followed_by: Option<bool>,
        #[serde(rename = "smart_blocking")]
        _smart_blocking: Option<bool>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Legacy<'a> {
        #[serde(rename = "id_str", with = "integer_str")]
        pub _id: u64,
        pub screen_name: Cow<'a, str>,
        pub name: Option<Cow<'a, str>>,
        pub profile_image_url_https: Cow<'a, str>,
        pub protected: bool,
        pub verified: bool,
        pub verified_type: Option<VerifiedType>,
        #[serde(rename = "follow_request_sent")]
        _follow_request_sent: Option<bool>,
        #[serde(rename = "following")]
        _following: Option<bool>,
        #[serde(rename = "followed_by")]
        _followed_by: Option<bool>,
        #[serde(rename = "blocking")]
        _blocking: Option<bool>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct AffiliatesHighlightedLabel {
        pub label: Option<AffiliatesHighlightedLabelLabel>,
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct AffiliatesHighlightedLabelLabel {
        pub user_label_type: super::AffiliationLabelType,
    }
}

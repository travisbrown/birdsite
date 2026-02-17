use crate::model::user::properties::VerifiedType;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, bounded_static_derive_more::ToStatic)]
pub struct User<'a> {
    pub id: u64,
    pub screen_name: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
    pub profile_image_url: Option<Cow<'a, str>>,
    pub protected: Option<bool>,
    pub is_blue_verified: Option<bool>,
    pub verified: bool,
    pub verified_type: Option<VerifiedType>,
    pub super_follow_eligible: Option<bool>,
}

impl<'a> From<super::User<'a>> for User<'a> {
    fn from(value: super::User<'a>) -> Self {
        Self {
            id: value.id,
            screen_name: value.screen_name,
            name: value.name,
            profile_image_url: value.profile_image_url,
            protected: value.protected,
            is_blue_verified: Some(value.is_blue_verified),
            verified: value.verified,
            verified_type: value.verified_type,
            super_follow_eligible: value.super_follow_eligible,
        }
    }
}

impl<'a> From<super::community::User<'a>> for User<'a> {
    fn from(value: super::community::User<'a>) -> Self {
        Self {
            id: value.id,
            screen_name: value.screen_name,
            name: value.name,
            profile_image_url: Some(value.profile_image_url),
            protected: Some(value.protected),
            is_blue_verified: value.is_blue_verified,
            verified: value.verified,
            verified_type: value.verified_type,
            super_follow_eligible: value.super_follow_eligible,
        }
    }
}

use crate::model::country::Country;
use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

pub mod entities;
pub mod properties;

#[derive(Clone, Debug, Eq, ToStatic, PartialEq)]
pub struct User<'a> {
    pub id: u64,
    pub created_at: Option<DateTime<Utc>>,
    pub screen_name: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub location: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub description_urls: Option<Vec<Cow<'a, str>>>,
    pub protected: Option<bool>,
    pub verified: Option<bool>,
    pub blue_verified: Option<bool>,
    pub verified_type: Option<properties::VerifiedType>,
    pub followers_count: Option<usize>,
    pub friends_count: Option<usize>,
    pub statuses_count: Option<usize>,
    pub media_count: Option<usize>,
    pub listed_count: Option<usize>,
    pub subscribers_count: Option<usize>,
    pub creator_subscriptions_count: Option<usize>,
    pub pinned_tweet_id: Option<Option<u64>>,
    pub default_profile: Option<bool>,
    pub default_profile_image: Option<bool>,
    pub profile_image_url: Option<Cow<'a, str>>,
    pub profile_image_shape: Option<properties::ProfileImageShape>,
    pub highlights_info: Option<properties::HighlightsInfo>,
    pub profile_interstitial_type: Option<properties::ProfileInterstitialType>,
    pub parody_commentary_fan_label: Option<properties::ParodyCommentaryFanLabel>,
    pub withheld_in_countries: Option<Vec<Country>>,
}

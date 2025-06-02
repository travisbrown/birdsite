use crate::model::country::Country;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

pub mod entities;
pub mod properties;

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl<'a> User<'a> {
    pub fn into_owned(self) -> User<'static> {
        User {
            id: self.id,
            created_at: self.created_at,
            screen_name: self
                .screen_name
                .map(|screen_name| screen_name.into_owned().into()),
            name: self.name.map(|name| name.into_owned().into()),
            description: self
                .description
                .map(|description| description.into_owned().into()),
            location: self.location.map(|location| location.into_owned().into()),
            url: self.url.map(|url| url.into_owned().into()),
            description_urls: self.description_urls.map(|description_urls| {
                description_urls
                    .into_iter()
                    .map(|description_url| description_url.into_owned().into())
                    .collect()
            }),
            protected: self.protected,
            verified: self.verified,
            blue_verified: self.blue_verified,
            verified_type: self.verified_type,
            followers_count: self.followers_count,
            friends_count: self.friends_count,
            statuses_count: self.statuses_count,
            media_count: self.media_count,
            listed_count: self.listed_count,
            subscribers_count: self.subscribers_count,
            creator_subscriptions_count: self.creator_subscriptions_count,
            pinned_tweet_id: self.pinned_tweet_id,
            default_profile: self.default_profile,
            default_profile_image: self.default_profile_image,
            profile_image_url: self
                .profile_image_url
                .map(|profile_image_url| profile_image_url.into_owned().into()),
            profile_image_shape: self.profile_image_shape,
            highlights_info: self.highlights_info,
            profile_interstitial_type: self.profile_interstitial_type,
            parody_commentary_fan_label: self.parody_commentary_fan_label,
            withheld_in_countries: self.withheld_in_countries.clone(),
        }
    }
}

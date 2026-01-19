//! This data format appears for tweets in the Wayback Machine until at least 2022 (current last seen is 22 November 2022).
use crate::model::{
    attributes::text_timestamp,
    color::Color,
    country::Country,
    lang::Lang,
    place::{Place, TypedCoordinates},
    source::SourceAnchor,
    time_zone::TimeZone,
};
use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use serde_field_attributes::{integer_str, optional_integer_str, optional_range, optional_usize};
use std::borrow::Cow;
use std::ops::Range;

pub mod entity;
pub mod media;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetSnapshot<'a> {
    #[serde(with = "text_timestamp")]
    pub created_at: DateTime<Utc>,
    pub id: u64,
    #[serde(with = "integer_str")]
    id_str: u64,
    pub text: Cow<'a, str>,
    pub source: SourceAnchor,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<u64>,
    #[serde(with = "optional_integer_str")]
    in_reply_to_status_id_str: Option<u64>,
    pub in_reply_to_user_id: Option<u64>,
    #[serde(with = "optional_integer_str")]
    pub in_reply_to_user_id_str: Option<u64>,
    pub user: User<'a>,
    pub in_reply_to_screen_name: Option<Cow<'a, str>>,
    pub geo: Option<TypedCoordinates>,
    pub coordinates: Option<TypedCoordinates>,
    pub place: Option<Place<'a>>,
    pub contributors: Option<Vec<u64>>,
    pub quoted_status_id: Option<u64>,
    #[serde(with = "optional_integer_str")]
    #[serde(default)]
    pub quoted_status_id_str: Option<u64>,
    // We have to write out the type here and below because of an apparent bug in the `ToStatic` macro.
    #[allow(clippy::use_self)]
    pub quoted_status: Option<Box<TweetSnapshot<'a>>>,
    pub quoted_status_permalink: Option<Url<'a>>,
    #[allow(clippy::use_self)]
    pub retweeted_status: Option<Box<TweetSnapshot<'a>>>,
    pub is_quote_status: bool,
    // Missing for one known case (881014163392401408).
    pub quote_count: Option<usize>,
    // Missing for one known case (881014163392401408).
    pub reply_count: Option<usize>,
    pub retweet_count: usize,
    pub favorite_count: usize,
    pub entities: entity::TweetEntities<'a>,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub filter_level: FilterLevel,
    pub lang: Lang,
    pub timestamp_ms: Option<String>,
    #[serde(with = "optional_range", default)]
    pub display_text_range: Option<Range<usize>>,
    #[serde(borrow)]
    pub extended_tweet: Option<ExtendedTweet<'a>>,
    pub extended_entities: Option<entity::ExtendedTweetExtendedEntities<'a>>,
    pub withheld_in_countries: Option<Vec<Country>>,
    pub scopes: Option<Scopes>,
}

impl<'a> TweetSnapshot<'a> {
    pub fn users(&self) -> Vec<User<'a>> {
        let mut users = Vec::with_capacity(1);

        self.add_users(&mut users);

        users
    }

    fn add_users(&self, acc: &mut Vec<User<'a>>) {
        acc.push(self.user.clone());

        if let Some(quoted_status) = &self.quoted_status {
            quoted_status.add_users(acc);
        }

        if let Some(retweeted_status) = &self.retweeted_status {
            retweeted_status.add_users(acc);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    pub url: Cow<'a, str>,
    pub expanded: Cow<'a, str>,
    pub display: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scopes {
    pub followers: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweet<'a> {
    #[serde(borrow)]
    pub full_text: Cow<'a, str>,
    pub display_text_range: Range<usize>,
    pub entities: entity::TweetEntities<'a>,
    pub extended_entities: Option<entity::ExtendedTweetExtendedEntities<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    pub id: u64,
    #[serde(with = "integer_str")]
    id_str: u64,
    pub name: Cow<'a, str>,
    pub screen_name: Cow<'a, str>,
    pub location: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub translator_type: Option<TranslatorType>,
    pub protected: bool,
    pub verified: bool,
    #[serde(with = "optional_usize")]
    pub followers_count: Option<usize>,
    #[serde(with = "optional_usize")]
    pub friends_count: Option<usize>,
    pub listed_count: Option<usize>,
    #[serde(with = "optional_usize")]
    pub favourites_count: Option<usize>,
    #[serde(with = "optional_usize")]
    pub statuses_count: Option<usize>,
    #[serde(with = "text_timestamp")]
    pub created_at: DateTime<Utc>,
    pub utc_offset: Option<isize>,
    pub time_zone: Option<TimeZone>,
    pub geo_enabled: bool,
    pub lang: Option<Lang>,
    pub contributors_enabled: bool,
    pub is_translator: bool,
    pub profile_background_color: Color,
    profile_background_image_url: Cow<'a, str>,
    pub profile_background_image_url_https: Cow<'a, str>,
    pub profile_background_tile: bool,
    pub profile_link_color: Color,
    pub profile_sidebar_border_color: Color,
    pub profile_sidebar_fill_color: Color,
    pub profile_text_color: Color,
    pub profile_use_background_image: bool,
    profile_image_url: Cow<'a, str>,
    pub profile_image_url_https: Cow<'a, str>,
    pub profile_banner_url: Option<Cow<'a, str>>,
    pub default_profile: bool,
    pub default_profile_image: bool,
    // If the following three fields are present, they are always `null`.
    following: Option<()>,
    follow_request_sent: Option<()>,
    notifications: Option<()>,
    pub withheld_in_countries: Option<Vec<Country>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FilterLevel {
    #[serde(rename = "low")]
    Low,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TranslatorType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "badged")]
    Badged,
    #[serde(rename = "moderator")]
    Moderator,
}

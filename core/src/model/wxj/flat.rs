//! This data format appears for tweets in the Wayback Machine until at least 2020 (TODO: find more precise dates).
use crate::model::{
    attributes::text_timestamp, color::Color, country::Country, lang::Lang, time_zone::TimeZone,
};
use chrono::{DateTime, Utc};
use serde_field_attributes::{integer_str, optional_integer_str, optional_usize};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//#[serde(deny_unknown_fields)]
pub struct TweetSnapshot<'a> {
    #[serde(with = "text_timestamp")]
    pub created_at: DateTime<Utc>,
    pub id: u64,
    #[serde(with = "integer_str")]
    id_str: u64,
    pub text: Cow<'a, str>,
    //pub source: Source,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<u64>,
    #[serde(with = "optional_integer_str")]
    in_reply_to_status_id_str: Option<u64>,
    pub in_reply_to_user_id: Option<u64>,
    #[serde(with = "optional_integer_str")]
    in_reply_to_user_id_str: Option<u64>,
    pub user: User<'a>,
    pub in_reply_to_screen_name: Option<Cow<'a, str>>,
    //geo: Option<Geo>,
    //coordinates: Option<Geo>,
    //pub place: Option<Place<'a>>,
    contributors: Option<Vec<u64>>,
    pub quoted_status_id: Option<u64>,
    #[serde(with = "optional_integer_str")]
    #[serde(default)]
    quoted_status_id_str: Option<u64>,
    pub quoted_status: Option<Box<TweetSnapshot<'a>>>,
    //pub quoted_status_permalink: Option<Url<'a>>,
    pub retweeted_status: Option<Box<TweetSnapshot<'a>>>,
    pub is_quote_status: bool,
    // Missing for one known case (881014163392401408).
    pub quote_count: Option<usize>,
    // Missing for one known case (881014163392401408).
    pub reply_count: Option<usize>,
    pub retweet_count: usize,
    pub favorite_count: usize,
    //pub entities: serde_json::Value,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub filter_level: FilterLevel,
    pub lang: Lang,
    pub timestamp_ms: Option<String>,
    pub display_text_range: Option<(usize, usize)>,
    #[serde(borrow)]
    pub extended_tweet: Option<ExtendedTweet<'a>>,
    //pub extended_entities: Option<ExtendedTweetExtendedEntities<'a>>,
    pub withheld_in_countries: Option<Vec<Country>>,
    pub scopes: Option<Scopes>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scopes {
    pub followers: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
//#[serde(deny_unknown_fields)]
pub struct ExtendedTweet<'a> {
    #[serde(borrow)]
    pub full_text: Cow<'a, str>,
    pub display_text_range: (usize, usize),
    //pub entities: ExtendedTweetEntities<'a>,
    //pub extended_entities: Option<ExtendedTweetExtendedEntities<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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
    utc_offset: Option<isize>,
    time_zone: Option<TimeZone>,
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

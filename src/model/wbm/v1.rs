//! This data format appears for tweets in the Wayback Machine until at least 2020 (TODO: find more precise dates).

use crate::model::{
    color::Color, id_str, id_str_optional, source::Source, timestamp::text_timestamp, Lang, Url,
};
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    #[serde(with = "text_timestamp")]
    pub created_at: DateTime<Utc>,
    pub id: u64,
    #[serde(with = "id_str")]
    id_str: u64,
    pub text: Cow<'a, str>,
    pub source: Source,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<u64>,
    #[serde(with = "id_str_optional")]
    in_reply_to_status_id_str: Option<u64>,
    pub in_reply_to_user_id: Option<u64>,
    #[serde(with = "id_str_optional")]
    in_reply_to_user_id_str: Option<u64>,
    pub user: User<'a>,
    pub in_reply_to_screen_name: Option<Cow<'a, str>>,
    geo: Option<()>,
    coordinates: Option<()>,
    place: Option<()>,
    contributors: Option<()>,
    pub quoted_status_id: Option<u64>,
    #[serde(with = "id_str_optional")]
    #[serde(default)]
    quoted_status_id_str: Option<u64>,
    pub quoted_status: Option<Box<Tweet<'a>>>,
    pub quoted_status_permalink: Option<Url<'a>>,
    pub is_quote_status: bool,
    pub quote_count: usize,
    pub reply_count: usize,
    pub retweet_count: usize,
    pub favorite_count: usize,
    pub entities: serde_json::Value,
    pub favorited: bool,
    pub retweeted: bool,
    pub filter_level: FilterLevel,
    pub lang: Lang,
    pub timestamp_ms: Option<String>,
    pub display_text_range: Option<(usize, usize)>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    pub id: u64,
    #[serde(with = "id_str")]
    id_str: u64,
    pub name: Cow<'a, str>,
    pub screen_name: Cow<'a, str>,
    pub location: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub translator_type: TranslatorType,
    pub protected: bool,
    pub verified: bool,
    pub followers_count: usize,
    pub friends_count: usize,
    pub listed_count: usize,
    pub favourites_count: usize,
    pub statuses_count: usize,
    #[serde(with = "text_timestamp")]
    pub created_at: DateTime<Utc>,
    utc_offset: Option<()>,
    time_zone: Option<()>,
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
}

#[cfg(test)]
mod tests {
    use super::Tweet;

    const TWEET_EXAMPLE_01: &str =
        include_str!("../../../examples/wbm-v1-1326237393893158914.json");
    const TWEET_EXAMPLE_02: &str = include_str!("../../../examples/wbm-v1-986837527037337601.json");

    #[test]
    fn parse_tweet_data_example_01() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_01).unwrap();

        assert_eq!(tweet.id, 1326237393893158914);
    }

    #[test]
    fn parse_tweet_data_example_02() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_02).unwrap();

        assert_eq!(tweet.id, 986837527037337601);
    }
}

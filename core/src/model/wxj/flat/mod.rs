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
use serde_field_attributes::{optional_range, optional_timestamp_millis_str, range};
use std::borrow::Cow;
use std::ops::Range;

pub mod entity;

// The bools mirror the wire format.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetSnapshot<'a> {
    #[serde(with = "text_timestamp")]
    pub created_at: DateTime<Utc>,
    pub id: u64,
    #[serde(with = "crate::model::attributes::id_str")]
    id_str: u64,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
    pub source: SourceAnchor,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<u64>,
    #[serde(with = "crate::model::attributes::optional_id_str")]
    in_reply_to_status_id_str: Option<u64>,
    pub in_reply_to_user_id: Option<u64>,
    #[serde(with = "crate::model::attributes::optional_id_str")]
    in_reply_to_user_id_str: Option<u64>,
    pub user: User<'a>,
    #[serde(borrow)]
    pub in_reply_to_screen_name: Option<Cow<'a, str>>,
    pub geo: Option<TypedCoordinates>,
    pub coordinates: Option<TypedCoordinates>,
    pub place: Option<Place<'a>>,
    pub contributors: Option<Vec<u64>>,
    pub quoted_status_id: Option<u64>,
    #[serde(with = "crate::model::attributes::optional_id_str")]
    #[serde(default)]
    quoted_status_id_str: Option<u64>,
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
    // Present only on top-level streamed tweets; nested quoted/retweeted statuses omit it.
    #[serde(with = "optional_timestamp_millis_str", default)]
    pub timestamp_ms: Option<DateTime<Utc>>,
    #[serde(with = "optional_range", default)]
    pub display_text_range: Option<Range<usize>>,
    #[serde(borrow)]
    pub extended_tweet: Option<ExtendedTweet<'a>>,
    pub extended_entities: Option<entity::ExtendedTweetExtendedEntities<'a>>,
    pub withheld_in_countries: Option<Vec<Country>>,
    pub withheld_copyright: Option<bool>,
    pub scopes: Option<Scopes>,
}

impl<'a> TweetSnapshot<'a> {
    #[must_use]
    pub fn users(&self) -> Vec<&User<'a>> {
        let mut users = Vec::with_capacity(1);

        self.add_users(&mut users);

        users
    }

    fn add_users<'s>(&'s self, acc: &mut Vec<&'s User<'a>>) {
        acc.push(&self.user);

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
    #[serde(borrow)]
    pub url: Cow<'a, str>,
    #[serde(borrow)]
    pub expanded: Cow<'a, str>,
    #[serde(borrow)]
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
    #[serde(with = "range")]
    pub display_text_range: Range<usize>,
    pub entities: entity::TweetEntities<'a>,
    pub extended_entities: Option<entity::ExtendedTweetExtendedEntities<'a>>,
}

// The bools mirror the wire format.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    pub id: u64,
    #[serde(with = "crate::model::attributes::id_str")]
    id_str: u64,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub screen_name: Cow<'a, str>,
    #[serde(borrow)]
    pub location: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub url: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub description: Option<Cow<'a, str>>,
    pub translator_type: Option<TranslatorType>,
    pub protected: bool,
    pub verified: bool,
    #[serde(with = "crate::model::attributes::optional_count_with_sentinel")]
    pub followers_count: Option<usize>,
    #[serde(with = "crate::model::attributes::optional_count_with_sentinel")]
    pub friends_count: Option<usize>,
    pub listed_count: Option<usize>,
    #[serde(with = "crate::model::attributes::optional_count_with_sentinel")]
    pub favourites_count: Option<usize>,
    #[serde(with = "crate::model::attributes::optional_count_with_sentinel")]
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
    #[serde(borrow)]
    profile_background_image_url: Cow<'a, str>,
    #[serde(borrow)]
    pub profile_background_image_url_https: Cow<'a, str>,
    pub profile_background_tile: bool,
    pub profile_link_color: Color,
    pub profile_sidebar_border_color: Color,
    pub profile_sidebar_fill_color: Color,
    pub profile_text_color: Color,
    pub profile_use_background_image: bool,
    #[serde(borrow)]
    profile_image_url: Cow<'a, str>,
    #[serde(borrow)]
    pub profile_image_url_https: Cow<'a, str>,
    #[serde(borrow)]
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

#[cfg(test)]
mod tests {
    use super::{ExtendedTweet, TweetSnapshot};

    /// Recursively drops object entries whose value is `null`.
    ///
    /// Absent optional fields are serialized as explicit nulls rather than omitted, so this
    /// normalization is what lets a re-serialized snapshot be compared against the input JSON.
    fn strip_nulls(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(fields) => {
                fields.retain(|_, field| !field.is_null());

                for field in fields.values_mut() {
                    strip_nulls(field);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    strip_nulls(item);
                }
            }
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }

    #[test]
    fn serializes_extended_tweet_display_text_range_as_array() {
        // No corpus line exercises `extended_tweet`, and this field previously lacked the `range`
        // attribute, so it was emitted as a `start`/`end` map that still reparsed as an equal
        // value.
        let json = concat!(
            r#"{"full_text":"hello world","display_text_range":[0,11],"entities":"#,
            r#"{"hashtags":[],"urls":[],"user_mentions":[],"symbols":[],"media":null}}"#
        );

        let extended = serde_json::from_str::<ExtendedTweet<'_>>(json).unwrap();

        assert_eq!(extended.display_text_range, 0..11);
        assert!(
            serde_json::to_string(&extended)
                .unwrap()
                .contains(r#""display_text_range":[0,11]"#)
        );
    }

    #[test]
    fn deserialize_and_round_trip_flat_examples() {
        let lines = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/wxj/flat.ndjson"
        ))
        .split('\n')
        .filter(|line| !line.is_empty());

        for (i, line) in lines.enumerate() {
            let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(line)
                .unwrap_or_else(|error| panic!("Line {}: invalid flat snapshot: {error}", i + 1));

            // The millisecond timestamp is present on the top-level streamed tweet but omitted on
            // nested statuses, exercising both the parsed and defaulted paths.
            assert!(
                snapshot.timestamp_ms.is_some(),
                "Line {}: missing timestamp_ms",
                i + 1
            );
            for nested in snapshot
                .retweeted_status
                .iter()
                .chain(&snapshot.quoted_status)
            {
                assert!(
                    nested.timestamp_ms.is_none(),
                    "Line {}: nested timestamp_ms",
                    i + 1
                );
            }

            // Re-serializing and re-parsing must yield an identical value, which confirms the
            // `DateTime`-to-millisecond-string conversion is symmetric.
            let serialized = serde_json::to_string(&snapshot).expect("serializable snapshot");
            let reparsed = serde_json::from_str::<TweetSnapshot<'_>>(&serialized)
                .expect("re-parseable snapshot");
            assert_eq!(snapshot, reparsed, "Line {}: round-trip mismatch", i + 1);

            // Value equality alone cannot see a changed JSON shape, since a two-element array
            // emitted as a `start`/`end` map reparses to an equal value, so also compare the
            // emitted JSON against the input.
            let mut original =
                serde_json::from_str::<serde_json::Value>(line).expect("valid input JSON");
            let mut emitted =
                serde_json::from_str::<serde_json::Value>(&serialized).expect("valid output JSON");
            strip_nulls(&mut original);
            strip_nulls(&mut emitted);
            assert_eq!(original, emitted, "Line {}: wire-format mismatch", i + 1);
        }
    }
}

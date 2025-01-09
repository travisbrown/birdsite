//! This data format appears for tweets in the Wayback Machine until at least 2020 (TODO: find more precise dates).
use crate::model::{
    Place, PossibleCount, Url,
    color::Color,
    country::Country,
    entities::{ExtendedTweetEntities, ExtendedTweetExtendedEntities},
    id_str, id_str_optional,
    lang::Lang,
    source::Source,
    time_zone::TimeZone,
    timestamp::text_timestamp,
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
    geo: Option<Geo>,
    coordinates: Option<Geo>,
    pub place: Option<Place<'a>>,
    contributors: Option<Vec<u64>>,
    pub quoted_status_id: Option<u64>,
    #[serde(with = "id_str_optional")]
    #[serde(default)]
    quoted_status_id_str: Option<u64>,
    pub quoted_status: Option<Box<Tweet<'a>>>,
    pub quoted_status_permalink: Option<Url<'a>>,
    pub retweeted_status: Option<Box<Tweet<'a>>>,
    pub is_quote_status: bool,
    // Missing for one known case (881014163392401408).
    pub quote_count: Option<usize>,
    // Missing for one known case (881014163392401408).
    pub reply_count: Option<usize>,
    pub retweet_count: usize,
    pub favorite_count: usize,
    pub entities: serde_json::Value,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub filter_level: FilterLevel,
    pub lang: Lang,
    pub timestamp_ms: Option<String>,
    pub display_text_range: Option<(usize, usize)>,
    #[serde(borrow)]
    pub extended_tweet: Option<ExtendedTweet<'a>>,
    pub extended_entities: Option<ExtendedTweetExtendedEntities<'a>>,
    pub withheld_in_countries: Option<Vec<Country>>,
    pub scopes: Option<Scopes>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Geo {
    #[serde(rename = "type")]
    pub geo_type: GeoType,
    // TODO: Fix this.
    pub coordinates: Vec<serde_json::Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GeoType {
    Point,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scopes {
    pub followers: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedTweet<'a> {
    pub full_text: Cow<'a, str>,
    pub display_text_range: (usize, usize),
    pub entities: ExtendedTweetEntities<'a>,
    #[serde(borrow)]
    pub extended_entities: Option<ExtendedTweetExtendedEntities<'a>>,
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
    pub translator_type: Option<TranslatorType>,
    pub protected: bool,
    pub verified: bool,
    pub followers_count: PossibleCount,
    pub friends_count: PossibleCount,
    pub listed_count: PossibleCount,
    pub favourites_count: PossibleCount,
    pub statuses_count: PossibleCount,
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

#[cfg(test)]
mod tests {
    use super::Tweet;

    const TWEET_EXAMPLE_01: &str =
        include_str!("../../../../examples/wbm-v1-1326237393893158914.json");
    const TWEET_EXAMPLE_02: &str =
        include_str!("../../../../examples/wbm-v1-986837527037337601.json");
    const TWEET_EXAMPLE_03: &str =
        include_str!("../../../../examples/wbm-v1-1125834307958788097.json");
    const TWEET_EXAMPLE_04: &str =
        include_str!("../../../../examples/wbm-v1-1142795641313271808.json");
    const TWEET_EXAMPLE_05: &str =
        include_str!("../../../../examples/wbm-v1-927401529837015041.json");
    const TWEET_EXAMPLE_06: &str =
        include_str!("../../../../examples/wbm-v1-1426766010791641094.json");
    const TWEET_EXAMPLE_07: &str =
        include_str!("../../../../examples/wbm-v1-1103673505617494018.json");
    const TWEET_EXAMPLE_08: &str =
        include_str!("../../../../examples/wbm-v1-1186383762835361792.json");
    const TWEET_EXAMPLE_09: &str =
        include_str!("../../../../examples/wbm-v1-1573518747654098945.json");
    const TWEET_EXAMPLE_10: &str =
        include_str!("../../../../examples/wbm-v1-1006425694312771584.json");
    const TWEET_EXAMPLE_11: &str =
        include_str!("../../../../examples/wbm-v1-881014163392401408.json");
    const TWEET_EXAMPLE_12: &str =
        include_str!("../../../../examples/wbm-v1-869020559493013505.json");
    const TWEET_EXAMPLE_13: &str =
        include_str!("../../../../examples/wbm-v1-1287425330517626882.json");
    const TWEET_EXAMPLE_14: &str =
        include_str!("../../../../examples/wbm-v1-1455280705504698387.json");
    const TWEET_EXAMPLE_15: &str =
        include_str!("../../../../examples/wbm-v1-1113327568692736006.json");
    const TWEET_EXAMPLE_16: &str =
        include_str!("../../../../examples/wbm-v1-907294938248949760.json");
    const TWEET_EXAMPLE_17: &str =
        include_str!("../../../../examples/wbm-v1-1591427284862377986.json");
    const TWEET_EXAMPLE_18: &str =
        include_str!("../../../../examples/wbm-v1-1141444685342355456.json");
    const TWEET_EXAMPLE_19: &str =
        include_str!("../../../../examples/wbm-v1-1452944423688409088.json");
    const TWEET_EXAMPLE_20: &str =
        include_str!("../../../../examples/wbm-v1-911468346234859521.json");
    const TWEET_EXAMPLE_21: &str =
        include_str!("../../../../examples/wbm-v1-1521459914643755008.json");
    const TWEET_EXAMPLE_22: &str =
        include_str!("../../../../examples/wbm-v1-998645860140441602.json");
    const TWEET_EXAMPLE_23: &str =
        include_str!("../../../../examples/wbm-v1-1329632859162697728.json");
    const TWEET_EXAMPLE_24: &str =
        include_str!("../../../../examples/wbm-v1-1283938882443186176.json");
    const TWEET_EXAMPLE_25: &str =
        include_str!("../../../../examples/wbm-v1-1239286867297816577.json");
    const TWEET_EXAMPLE_26: &str =
        include_str!("../../../../examples/wbm-v1-1175405918722449408.json");
    const TWEET_EXAMPLE_27: &str =
        include_str!("../../../../examples/wbm-v1-1206307973523476480.json");
    const TWEET_EXAMPLE_28: &str =
        include_str!("../../../../examples/wbm-v1-1179259452261851136.json");
    const TWEET_EXAMPLE_29: &str =
        include_str!("../../../../examples/wbm-v1-860177515423436801.json");
    const TWEET_EXAMPLE_30: &str =
        include_str!("../../../../examples/wbm-v1-861630070003290112.json");
    const TWEET_EXAMPLE_31: &str =
        include_str!("../../../../examples/wbm-v1-1338604362164264968.json");
    const TWEET_EXAMPLE_32: &str =
        include_str!("../../../../examples/wbm-v1-865619549365956608.json");
    const TWEET_EXAMPLE_33: &str =
        include_str!("../../../../examples/wbm-v1-1351526379427401732.json");
    const TWEET_EXAMPLE_34: &str =
        include_str!("../../../../examples/wbm-v1-901037368022519808.json");
    const TWEET_EXAMPLE_35: &str =
        include_str!("../../../../examples/wbm-v1-910881101723217920.json");

    #[test]
    fn parse_tweet_v1_example_01() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_01).unwrap();

        assert_eq!(tweet.id, 1326237393893158914);
    }

    #[test]
    fn parse_tweet_v1_example_02() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_02).unwrap();

        assert_eq!(tweet.id, 986837527037337601);
    }

    #[test]
    fn parse_tweet_v1_example_03() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_03).unwrap();

        assert_eq!(tweet.id, 1125834307958788097);
    }

    #[test]
    fn parse_tweet_v1_example_04() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_04).unwrap();

        assert_eq!(tweet.id, 1142795641313271808);
    }

    #[test]
    fn parse_tweet_v1_example_05() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_05).unwrap();

        assert_eq!(tweet.id, 927401529837015041);
    }

    #[test]
    fn parse_tweet_v1_example_06() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_06).unwrap();

        assert_eq!(tweet.id, 1426766010791641094);
    }

    #[test]
    fn parse_tweet_v1_example_07() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_07).unwrap();

        assert_eq!(tweet.id, 1103673505617494018);
    }

    #[test]
    fn parse_tweet_v1_example_08() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_08).unwrap();

        assert_eq!(tweet.id, 1186383762835361792);
    }

    #[test]
    fn parse_tweet_v1_example_09() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_09).unwrap();

        assert_eq!(tweet.id, 1573518747654098945);
    }

    #[test]
    fn parse_tweet_v1_example_10() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_10).unwrap();

        assert_eq!(tweet.id, 1006425694312771584);
    }

    #[test]
    fn parse_tweet_v1_example_11() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_11).unwrap();

        assert_eq!(tweet.id, 881014163392401408);
    }

    #[test]
    fn parse_tweet_v1_example_12() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_12).unwrap();

        assert_eq!(tweet.id, 869020559493013505);
    }

    #[test]
    fn parse_tweet_v1_example_13() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_13).unwrap();

        assert_eq!(tweet.id, 1287425330517626882);
    }

    #[test]
    fn parse_tweet_v1_example_14() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_14).unwrap();

        assert_eq!(tweet.id, 1455280705504698387);
    }

    #[test]
    fn parse_tweet_v1_example_15() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_15).unwrap();

        assert_eq!(tweet.id, 1113327568692736006);
    }

    #[test]
    fn parse_tweet_v1_example_16() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_16).unwrap();

        assert_eq!(tweet.id, 907294938248949760);
    }

    #[test]
    fn parse_tweet_v1_example_17() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_17).unwrap();

        assert_eq!(tweet.id, 1591427284862377986);
    }

    #[test]
    fn parse_tweet_v1_example_18() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_18).unwrap();

        assert_eq!(tweet.id, 1141444685342355456);
    }

    #[test]
    fn parse_tweet_v1_example_19() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_19).unwrap();

        assert_eq!(tweet.id, 1452944423688409088);
    }

    #[test]
    fn parse_tweet_v1_example_20() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_20).unwrap();

        assert_eq!(tweet.id, 911468346234859521);
    }

    #[test]
    fn parse_tweet_v1_example_21() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_21).unwrap();

        assert_eq!(tweet.id, 1521459914643755008);
    }

    #[test]
    fn parse_tweet_v1_example_22() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_22).unwrap();

        assert_eq!(tweet.id, 998645860140441602);
    }

    #[test]
    fn parse_tweet_v1_example_23() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_23).unwrap();

        assert_eq!(tweet.id, 1329632859162697728);
    }

    #[test]
    fn parse_tweet_v1_example_24() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_24).unwrap();

        assert_eq!(tweet.id, 1283938882443186176);
    }

    #[test]
    fn parse_tweet_v1_example_25() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_25).unwrap();

        assert_eq!(tweet.id, 1239286867297816577);
    }

    #[test]
    fn parse_tweet_v1_example_26() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_26).unwrap();

        assert_eq!(tweet.id, 1175405918722449408);
    }

    #[test]
    fn parse_tweet_v1_example_27() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_27).unwrap();

        assert_eq!(tweet.id, 1206307973523476480);
    }

    #[test]
    fn parse_tweet_v1_example_28() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_28).unwrap();

        assert_eq!(tweet.id, 1179259452261851136);
    }

    #[test]
    fn parse_tweet_v1_example_29() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_29).unwrap();

        assert_eq!(tweet.id, 860177515423436801);
    }

    #[test]
    fn parse_tweet_v1_example_30() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_30).unwrap();

        assert_eq!(tweet.id, 861630070003290112);
    }

    #[test]
    fn parse_tweet_v1_example_31() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_31).unwrap();

        assert_eq!(tweet.id, 1338604362164264968);
    }

    #[test]
    fn parse_tweet_v1_example_32() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_32).unwrap();

        assert_eq!(tweet.id, 865619549365956608);
    }

    #[test]
    fn parse_tweet_v1_example_33() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_33).unwrap();

        assert_eq!(tweet.id, 1351526379427401732);
    }

    #[test]
    fn parse_tweet_v1_example_34() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_34).unwrap();

        assert_eq!(tweet.id, 901037368022519808);
    }

    #[test]
    fn parse_tweet_v1_example_35() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_35).unwrap();

        assert_eq!(tweet.id, 910881101723217920);
    }
}

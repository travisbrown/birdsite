//! This data format appears for tweets in the Wayback Machine from at least 2023 into 2025 (TODO: find previous start date).

use crate::model::{
    entities::{TweetEntities, UserEntities},
    id_str, id_str_array, id_str_array_optional, id_str_optional, EditControls, Lang,
    TweetPublicMetrics, UserPublicMetrics,
};
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tweet<'a> {
    pub data: TweetData<'a>,
    #[serde(borrow)]
    pub includes: TweetIncludes<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetData<'a> {
    pub article: Option<Article>,
    pub attachments: Attachments<'a>,
    #[serde(with = "id_str")]
    pub id: u64,
    #[serde(with = "id_str")]
    pub author_id: u64,
    pub context_annotations: Option<Vec<ContextAnnotation<'a>>>,
    #[serde(with = "id_str")]
    pub conversation_id: u64,
    pub created_at: DateTime<Utc>,
    pub edit_controls: EditControls,
    #[serde(with = "id_str_array")]
    pub edit_history_tweet_ids: Vec<u64>,
    pub lang: Lang,
    pub entities: TweetEntities<'a>,
    pub geo: Geo,
    pub note_tweet: Option<NoteTweet<'a>>,
    pub possibly_sensitive: bool,
    pub public_metrics: TweetPublicMetrics,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    pub reply_settings: ReplySettings,
    pub text: Cow<'a, str>,
    #[serde(with = "id_str_optional")]
    #[serde(default)]
    pub in_reply_to_user_id: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextAnnotation<'a> {
    pub domain: ContextDomain<'a>,
    pub entity: ContextEntity<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextDomain<'a> {
    #[serde(with = "id_str")]
    pub id: u64,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextEntity<'a> {
    #[serde(with = "id_str")]
    pub id: u64,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetIncludes<'a> {
    pub media: Option<Vec<Media<'a>>>,
    #[serde(borrow)]
    pub users: Vec<User<'a>>,
    pub tweets: Vec<TweetData<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Media<'a> {
    pub media_key: Cow<'a, str>,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub url: Cow<'a, str>,
    pub public_metrics: MediaPublicMetrics,
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MediaType {
    #[serde(rename = "photo")]
    Photo,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaPublicMetrics {}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoteTweet<'a> {
    pub entities: TweetEntities<'a>,
    pub text: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub reference_type: ReferenceType,
    #[serde(with = "id_str")]
    pub id: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReferenceType {
    #[serde(rename = "retweeted")]
    Retweeted,
    #[serde(rename = "replied_to")]
    RepliedTo,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ReplySettings {
    #[serde(rename = "everyone")]
    Everyone,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    #[serde(with = "id_str")]
    pub id: u64,
    pub username: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub description: Cow<'a, str>,
    pub location: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub profile_image_url: Cow<'a, str>,
    #[serde(with = "id_str_optional")]
    #[serde(default)]
    pub pinned_tweet_id: Option<u64>,
    #[serde(borrow)]
    pub entities: Option<UserEntities<'a>>,
    pub verified: bool,
    pub protected: bool,
    pub public_metrics: UserPublicMetrics,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Article {}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attachments<'a> {
    // TODO: use a structured representation for these keys (format: "3_1881163280061730816").
    pub media_keys: Option<Vec<Cow<'a, str>>>,
    #[serde(with = "id_str_array_optional")]
    #[serde(default)]
    pub media_source_tweet_id: Option<Vec<u64>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Geo {}

#[cfg(test)]
mod tests {
    use super::Tweet;

    const TWEET_EXAMPLE_01: &str =
        include_str!("../../../examples/wbm-data-1879337629495496982.json");
    const TWEET_EXAMPLE_02: &str =
        include_str!("../../../examples/wbm-data-1637945937258754048.json");
    const TWEET_EXAMPLE_03: &str =
        include_str!("../../../examples/wbm-data-1881187152257810817.json");
    const TWEET_EXAMPLE_04: &str =
        include_str!("../../../examples/wbm-data-1857270748944994804.json");
    // TODO: redundant (passes with no changes)
    const TWEET_EXAMPLE_05: &str =
        include_str!("../../../examples/wbm-data-1859432681898983820.json");
    const TWEET_EXAMPLE_06: &str =
        include_str!("../../../examples/wbm-data-1865165548880888019.json");
    const TWEET_EXAMPLE_07: &str =
        include_str!("../../../examples/wbm-data-1875993789892047207.json");

    #[test]
    fn parse_tweet_data_example_01() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_01).unwrap();

        assert_eq!(tweet.data.id, 1879337629495496982);
    }

    #[test]
    fn parse_tweet_data_example_02() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_02).unwrap();

        assert_eq!(tweet.data.id, 1637945937258754048);
    }

    #[test]
    fn parse_tweet_data_example_03() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_03).unwrap();

        assert_eq!(tweet.data.id, 1881187152257810817);
    }

    #[test]
    fn parse_tweet_data_example_04() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_04).unwrap();

        assert_eq!(tweet.data.id, 1857270748944994804);
    }

    #[test]
    fn parse_tweet_data_example_05() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_05).unwrap();

        assert_eq!(tweet.data.id, 1859432681898983820);
    }

    #[test]
    fn parse_tweet_data_example_06() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_06).unwrap();

        assert_eq!(tweet.data.id, 1865165548880888019);
    }

    #[test]
    fn parse_tweet_data_example_07() {
        let tweet: Tweet = serde_json::from_str(TWEET_EXAMPLE_07).unwrap();

        assert_eq!(tweet.data.id, 1875993789892047207);
    }
}

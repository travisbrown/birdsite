#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetPublicMetrics {
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
    pub quote_count: usize,
    pub bookmark_count: Option<usize>,
    pub impression_count: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaPublicMetrics {
    pub view_count: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserPublicMetrics {
    // Generally has a value (one known case where it doesn't).
    pub followers_count: Option<usize>,
    // Generally has a value (one known case where it doesn't).
    pub following_count: Option<usize>,
    // Generally has a value (one known case where it doesn't).
    #[serde(with = "crate::model::attributes::usize_opt")]
    pub tweet_count: Option<usize>,
    // Generally has a value (one known case where it doesn't).
    pub listed_count: Option<usize>,
    pub like_count: Option<usize>,
    pub media_count: Option<usize>,
}

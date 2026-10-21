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
    #[serde(with = "crate::model::attributes::optional_count_with_sentinel")]
    pub tweet_count: Option<usize>,
    // Generally has a value (one known case where it doesn't).
    pub listed_count: Option<usize>,
    #[serde(
        with = "crate::model::attributes::optional_count_with_sentinel",
        default
    )]
    pub like_count: Option<usize>,
    pub media_count: Option<usize>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::UserPublicMetrics;

    #[test]
    fn user_like_counts_accept_unavailable_values_and_round_trip() {
        for (value, expected) in [
            (None, None),
            (Some(json!(null)), None),
            (Some(json!(-1)), None),
            (Some(json!(0)), Some(0)),
            (Some(json!(47)), Some(47)),
        ] {
            let mut input = json!({"tweet_count": -1});
            if let Some(value) = value {
                input["like_count"] = value;
            }
            let metrics: UserPublicMetrics = serde_json::from_value(input).unwrap();
            assert_eq!(metrics.like_count, expected);
            let emitted = serde_json::to_value(metrics).unwrap();
            assert_eq!(
                emitted["like_count"],
                expected.map_or_else(|| json!(-1), |count| json!(count))
            );
            assert_eq!(
                metrics,
                serde_json::from_value::<UserPublicMetrics>(emitted).unwrap()
            );
        }
    }

    #[test]
    fn user_like_counts_reject_other_negative_values_and_wrong_types() {
        for value in [json!(-2), json!(1.5), json!("-1"), json!(false)] {
            let input = json!({"tweet_count": 0, "like_count": value});
            assert!(serde_json::from_value::<UserPublicMetrics>(input).is_err());
        }
    }
}

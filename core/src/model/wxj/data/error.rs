use std::borrow::Cow;

use archivindex_serde::borrowable_str;
use bounded_static_derive_more::ToStatic;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TweetError<'a> {
    #[serde(borrow)]
    pub resource_id: Cow<'a, str>,
    #[serde(borrow)]
    pub parameter: Cow<'a, str>,
    pub resource_type: TweetErrorResourceType,
    pub section: Option<TweetErrorSection>,
    #[serde(borrow)]
    pub title: Cow<'a, str>,
    #[serde(default, with = "borrowable_str::option")]
    pub value: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub detail: Cow<'a, str>,
    #[serde(rename = "type")]
    pub error_type: TweetErrorType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorResourceType {
    #[serde(rename = "place")]
    Place,
    #[serde(rename = "tweet")]
    Tweet,
    #[serde(rename = "user")]
    User,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorSection {
    #[serde(rename = "includes")]
    Includes,
}

/// Problem types accepted under both API hostnames and serialized using `api.twitter.com`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TweetErrorType {
    #[serde(
        rename = "https://api.twitter.com/2/problems/not-authorized-for-resource",
        alias = "https://api.x.com/2/problems/not-authorized-for-resource"
    )]
    NotAuthorizedForResource,
    #[serde(
        rename = "https://api.twitter.com/2/problems/resource-not-found",
        alias = "https://api.x.com/2/problems/resource-not-found"
    )]
    ResourceNotFound,
    #[serde(
        rename = "https://api.twitter.com/2/problems/resource-unavailable",
        alias = "https://api.x.com/2/problems/resource-unavailable"
    )]
    ResourceUnavailable,
}

#[cfg(test)]
mod tests {
    use super::TweetErrorType;

    /// The API hostname changed, but the problem types retain their meaning. Serialization uses
    /// the original hostname for both spellings.
    #[test]
    fn error_types_accept_both_api_hostnames() {
        for (problem, expected) in [
            (
                "not-authorized-for-resource",
                TweetErrorType::NotAuthorizedForResource,
            ),
            ("resource-not-found", TweetErrorType::ResourceNotFound),
            ("resource-unavailable", TweetErrorType::ResourceUnavailable),
        ] {
            for host in ["api.twitter.com", "api.x.com"] {
                let json = format!(r#""https://{host}/2/problems/{problem}""#);
                let parsed = serde_json::from_str::<TweetErrorType>(&json).unwrap();
                assert_eq!(parsed, expected);
                assert_eq!(
                    serde_json::to_string(&parsed).unwrap(),
                    format!(r#""https://api.twitter.com/2/problems/{problem}""#)
                );
            }
        }
        assert!(
            serde_json::from_str::<TweetErrorType>(r#""https://api.x.com/2/problems/unknown""#)
                .is_err()
        );
    }
}

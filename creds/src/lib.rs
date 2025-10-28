#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![forbid(unsafe_code)]
use regex::Regex;
use std::sync::LazyLock;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Creds {
    pub bearer_token: String,
    pub csrf_token: String,
    pub cookie: String,
}

static BEARER_TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\-H 'authorization: Bearer ([\w%]+)'").unwrap());
static CSRF_TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\-H 'x-csrf-token: ([\w]+)'").unwrap());
static COOKIE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\-H 'Cookie: ([^']+)'").unwrap());
static COOKIE_SHORT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\-b '([^']+)'").unwrap());

impl Creds {
    #[must_use]
    pub fn new(bearer_token: &str, csrf_token: &str, cookie: &str) -> Self {
        Self {
            bearer_token: bearer_token.to_string(),
            csrf_token: csrf_token.to_string(),
            cookie: cookie.to_string(),
        }
    }

    pub fn parse_curl_command(command: &str) -> Option<Self> {
        let bearer_token = BEARER_TOKEN_RE
            .captures(command)
            .and_then(|captures| captures.get(1))
            .map(|capture_match| capture_match.as_str())?
            .to_string();

        let csrf_token = CSRF_TOKEN_RE
            .captures(command)
            .and_then(|captures| captures.get(1))
            .map(|capture_match| capture_match.as_str())?
            .to_string();

        let cookie = COOKIE_RE
            .captures(command)
            .and_then(|captures| captures.get(1))
            .map(|capture_match| capture_match.as_str())
            .or_else(|| {
                COOKIE_SHORT_RE
                    .captures(command)
                    .and_then(|captures| captures.get(1))
                    .map(|capture_match| capture_match.as_str())
            })?
            .to_string();

        Some(Self {
            bearer_token,
            csrf_token,
            cookie,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_curl_command() {
        let command = r#"curl 'https://twitter.com/i/api/graphql/VgitpdpNZ-RUIp5D1Z_D-A/UserTweets?variables=%7B%22userId%22%3A%221234%22%2C%22count%22%3A20%2C%22includePromotedContent%22%3Atrue%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Atrue%7D&features=%7B%22responsive_web_graphql_exclude_directive_enabled%22%3Atrue%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_home_pinned_timelines_enabled%22%3Atrue%2C%22creator_subscriptions_tweet_preview_api_enabled%22%3Atrue%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Afalse%2C%22c9s_tweet_anatomy_moderator_badge_enabled%22%3Atrue%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22responsive_web_edit_tweet_api_enabled%22%3Atrue%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Atrue%2C%22view_counts_everywhere_api_enabled%22%3Atrue%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22responsive_web_twitter_article_tweet_consumption_enabled%22%3Afalse%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Atrue%2C%22standardized_nudges_misinfo%22%3Atrue%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Atrue%2C%22longform_notetweets_rich_text_read_enabled%22%3Atrue%2C%22longform_notetweets_inline_media_enabled%22%3Atrue%2C%22responsive_web_media_download_video_enabled%22%3Afalse%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D' --compressed -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/118.0' -H 'Accept: */*' -H 'Accept-Language: en-US,en;q=0.5' -H 'Accept-Encoding: gzip, deflate, br' -H 'Referer: https://twitter.com/test' -H 'content-type: application/json' -H 'X-Client-UUID: aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa' -H 'x-twitter-auth-type: OAuth2Session' -H 'x-csrf-token: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' -H 'x-twitter-client-language: en' -H 'x-twitter-active-user: yes' -H 'X-Client-Transaction-Id: g/3r6gMV7WHWL4oRyH3eHdiL1dqt3emNwcdbnaLiqLdYi/PpCEUvgL7N7t1O88AsCYS/b4PyYoZ3w5zUwYlNuasP9w3Igg' -H 'Sec-Fetch-Dest: empty' -H 'Sec-Fetch-Mode: cors' -H 'Sec-Fetch-Site: same-origin' -H 'authorization: Bearer aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' -H 'Connection: keep-alive' -H 'Cookie: guest_id=v1%3A1234; kdt=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; twid=u%3D1234; auth_token=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; eu_cn=1; des_opt_in=Y; ct0=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; d_prefs=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; guest_id_ads=v1%3A1234; guest_id_marketing=v1%3A1234; personalization_id="aa_aaaaa/aaa+aaaaa+aaaaaa=="; _ga=GA1.2.1234.1234; twtr_pixel_opt_in=Y; external_referer=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa|0|aaaaaaaaaaa%3D; lang=en; _gid=GA1.2.1234.1234' -H 'TE: trailers'"#;

        let creds = Creds::parse_curl_command(command).unwrap();
        let expected = Creds {
            bearer_token: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            csrf_token: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            cookie: r#"guest_id=v1%3A1234; kdt=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; twid=u%3D1234; auth_token=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; eu_cn=1; des_opt_in=Y; ct0=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; d_prefs=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; guest_id_ads=v1%3A1234; guest_id_marketing=v1%3A1234; personalization_id="aa_aaaaa/aaa+aaaaa+aaaaaa=="; _ga=GA1.2.1234.1234; twtr_pixel_opt_in=Y; external_referer=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa|0|aaaaaaaaaaa%3D; lang=en; _gid=GA1.2.1234.1234"#.to_string()
        };

        assert_eq!(creds, expected);
    }
}

use base64::prelude::*;
use regex::Regex;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::sync::LazyLock;

const HOME_URL: &str = "https://x.com/";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36";

static SITE_VERIFICATION_CONTENT_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta[name='twitter-site-verification']").unwrap());
static SITE_VERIFICATION_CONTENT_ATTR: &str = "content";

static ONDEMAND_NAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"['|"]ondemand\.s['|"]:\s*['|"]([\w]*)['|"]"#).unwrap());

static ONDEMAND_INDICES_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\(\w\[(\d{1,2})\],\s*16\)").unwrap());

static FRAME_SVG_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("svg[id^='loading-x-anim']").unwrap());

static SECOND_PATH_SEL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("g > path:nth-of-type(2)").unwrap());

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP client error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Base64 decoding error")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("Response error")]
    Response(StatusCode),
    #[error("Missing site verification content")]
    MissingSiteVerificationContent,
    #[error("Invalid site verification content")]
    InvalidSiteVerificationContent,
    #[error("Missing ondemand file name")]
    MissingOndemand,
    #[error("Invalid indices")]
    InvalidIndices,
    #[error("Invalid frames")]
    InvalidFrames,
    #[error("Short site verification key")]
    ShortSiteVerificationKey { length: usize },
    #[error("Short frame array")]
    ShortFrameArray { length: usize, expected: usize },
    #[error("Short frame")]
    ShortFrame { length: usize },
}

pub struct SiteInfo {
    pub verification_key: Vec<u8>,
    pub indices: Vec<usize>,
    pub frame: Vec<i32>,
}

pub struct Client {
    underlying: reqwest::Client,
    user_agent: String,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            underlying: reqwest::Client::default(),
            user_agent: USER_AGENT.to_string(),
        }
    }
}

impl Client {
    pub async fn get_site_info(&self) -> Result<SiteInfo, Error> {
        let home = self.download_home().await?;
        let ondemand_url = home.ondemand_url()?;
        let verification_key = home.site_verification_key()?;
        let ondemand = self.download_ondemand(&ondemand_url).await?;
        let indices = ondemand.indices()?;
        let frame_index =
            verification_key
                .get(5)
                .ok_or_else(|| Error::ShortSiteVerificationKey {
                    length: verification_key.len(),
                })?
                % 4;
        let frame_array = home.frame_array(frame_index as usize)?;

        // Safe because we've already checked that there are at least two indices.
        let key_index = indices[0];
        let frame_index_from_key =
            (verification_key
                .get(key_index)
                .ok_or_else(|| Error::ShortSiteVerificationKey {
                    length: verification_key.len(),
                })?
                % 16) as usize;

        let frame_array_length = frame_array.len();
        let frame = frame_array
            .into_iter()
            .nth(frame_index_from_key)
            .ok_or_else(|| Error::ShortFrameArray {
                length: frame_array_length,
                expected: frame_index_from_key,
            })?;

        if frame.len() < 11 {
            Err(Error::ShortFrame {
                length: frame.len(),
            })
        } else {
            Ok(SiteInfo {
                verification_key,
                indices,
                frame,
            })
        }
    }

    async fn download_home(&self) -> Result<Home, Error> {
        let response = self
            .underlying
            .get(HOME_URL)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            let html = Html::parse_document(&body);

            Ok(Home { body, html })
        } else {
            Err(Error::Response(response.status()))
        }
    }

    async fn download_ondemand(&self, url: &str) -> Result<Ondemand, Error> {
        let response = self.underlying.get(url).send().await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;

            Ok(Ondemand { content: body })
        } else {
            Err(Error::Response(response.status()))
        }
    }
}

struct Home {
    body: String,
    html: Html,
}

impl Home {
    fn ondemand_url(&self) -> Result<String, Error> {
        let name = ONDEMAND_NAME_RE
            .captures(&self.body)
            .and_then(|captures| captures.get(1))
            .ok_or_else(|| Error::MissingOndemand)?;

        Ok(format!(
            "https://abs.twimg.com/responsive-web/client-web/ondemand.s.{}a.js",
            name.as_str()
        ))
    }

    fn site_verification_key(&self) -> Result<Vec<u8>, Error> {
        let metas = self
            .html
            .select(&SITE_VERIFICATION_CONTENT_SEL)
            .collect::<Vec<_>>();

        if metas.is_empty() {
            Err(Error::MissingSiteVerificationContent)
        } else if metas.len() > 1 {
            Err(Error::InvalidSiteVerificationContent)
        } else {
            let content = metas[0]
                .attr(SITE_VERIFICATION_CONTENT_ATTR)
                .ok_or_else(|| Error::MissingSiteVerificationContent)?;

            Ok(BASE64_STANDARD.decode(content)?)
        }
    }

    fn frame_array(&self, index: usize) -> Result<Vec<Vec<i32>>, Error> {
        let frames = self.html.select(&FRAME_SVG_SEL).collect::<Vec<_>>();
        let frame = frames.get(index).ok_or_else(|| Error::InvalidFrames)?;

        let second_paths = frame.select(&SECOND_PATH_SEL).collect::<Vec<_>>();

        if second_paths.len() == 1 {
            let second_path = second_paths[0];
            let d = second_path.attr("d").ok_or_else(|| Error::InvalidFrames)?;
            let array = d[9..]
                .split('C')
                .map(|part| {
                    part.replace([',', 'h', 's'], " ")
                        .split_whitespace()
                        .map(str::parse::<i32>)
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| Error::InvalidFrames)?;

            Ok(array)
        } else {
            Err(Error::InvalidFrames)
        }
    }
}

struct Ondemand {
    content: String,
}

impl Ondemand {
    fn indices(&self) -> Result<Vec<usize>, Error> {
        let indices = ONDEMAND_INDICES_RE
            .captures_iter(&self.content)
            .map(|capture| {
                capture
                    .get(1)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .ok_or_else(|| Error::InvalidIndices)
            })
            .collect::<Result<Vec<_>, _>>()?;

        if indices.len() < 2 {
            Err(Error::InvalidIndices)
        } else {
            Ok(indices)
        }
    }
}

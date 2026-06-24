use crate::{Endpoint, SiteInfo, TransactionId, generator::Generator};
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

// X serves two different web clients (which one varies day to day), so we support both ways of
// locating the file that holds the key-byte indices.
//
// 1. The webpack `client-web` build references an `ondemand.s.<name>a.js` file directly in the home
//    page, in two formats: an older one seen until around 2026-03-17, and a newer one since
//    2026-03-18.
static ONDEMAND_NAME_V1_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"['"]ondemand\.s['"]:\s*['"]([\w]*)['"]"#).unwrap());

static ONDEMAND_NAME_V2_INDEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#",\s*(\d+)\s*:\s*['"]ondemand\.s['"]"#).unwrap());

// 2. The Vite `x-web` build (first seen 2026-06-23) instead preloads a transaction-id chunk that
//    dynamically imports a `sign.o-<hash>.js` module, whose hashed name isn't referenced anywhere
//    else on the page.
static TRANSACTION_MODULE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://abs\.twimg\.com/[\w./-]*?sentry-filter-[\w-]+\.js").unwrap()
});

static SIGN_MODULE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"sign\.o-[\w-]+\.js").unwrap());

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
    #[error("Missing transaction-id index source")]
    MissingIndexSource,
    #[error("Missing sign module name")]
    MissingSignModule,
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

#[derive(Clone, Debug)]
pub struct Client {
    underlying: reqwest::Client,
    user_agent: String,
    generator: Generator,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(
            reqwest::Client::default(),
            USER_AGENT.to_string(),
            crate::generator::Generator::default(),
        )
    }
}

impl Client {
    const fn new(client: reqwest::Client, user_agent: String, generator: Generator) -> Self {
        Self {
            underlying: client,
            user_agent,
            generator,
        }
    }

    /// Download site information and generate a transaction ID for the given endpoint.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if downloading or parsing the site information fails (see
    /// [`get_site_info`](Self::get_site_info)).
    pub async fn generate(
        &self,
        endpoint: &Endpoint<'_>,
    ) -> Result<TransactionId, crate::client::Error> {
        let site_info = self.get_site_info().await?;

        Ok(self.generator.compute(&site_info, endpoint, None, None))
    }

    /// Download site information and generate a transaction ID for the given endpoint.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if downloading or parsing the site information fails (see
    /// [`get_site_info`](Self::get_site_info)).
    pub async fn generate_for_path(
        &self,
        path: &str,
    ) -> Result<TransactionId, crate::client::Error> {
        let site_info = self.get_site_info().await?;

        Ok(self
            .generator
            .compute_for_path(&site_info, path, None, None))
    }

    /// Download site information and generate transaction IDs for the given endpoints.
    ///
    /// This function only downloads the necessary files once.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if downloading or parsing the site information fails (see
    /// [`get_site_info`](Self::get_site_info)).
    pub async fn generate_batch(
        &self,
        endpoints: &[&Endpoint<'_>],
    ) -> Result<Vec<TransactionId>, crate::client::Error> {
        let site_info = self.get_site_info().await?;

        Ok(endpoints
            .iter()
            .map(|endpoint| self.generator.compute(&site_info, endpoint, None, None))
            .collect())
    }

    /// Download and parse the site information needed to generate transaction IDs.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if a download fails or returns a non-OK status, or if the home page or
    /// sign module does not have the expected shape (missing or malformed verification key,
    /// indices, or animation frames).
    pub async fn get_site_info(&self) -> Result<SiteInfo, Error> {
        // `Home` holds a `scraper::Html`, which is not `Sync`; its scope must
        // end before the next await (an explicit `drop` is not enough for the
        // compiler's capture analysis) so that this future (and every
        // `generate` future built on it) stays `Send` and usable with
        // `tokio::spawn`.
        let (index_source, verification_key, frame_array) = {
            let home = self.download_home().await?;
            let index_source = home.index_source()?;
            let verification_key = home.site_verification_key()?;
            let frame_index =
                verification_key
                    .get(5)
                    .ok_or_else(|| Error::ShortSiteVerificationKey {
                        length: verification_key.len(),
                    })?
                    % 4;
            let frame_array = home.frame_array(frame_index as usize)?;

            (index_source, verification_key, frame_array)
        };

        let indices = self.download_indices(&index_source).await?;

        // Every index reads a byte of the verification key (the first below,
        // the rest in the generator), and both values come from remote pages,
        // so reject any out-of-bounds index up front rather than panicking
        // later in `compute`.
        if indices.iter().any(|index| *index >= verification_key.len()) {
            Err(Error::ShortSiteVerificationKey {
                length: verification_key.len(),
            })
        } else {
            // Safe because we've already checked that there are at least two
            // indices and that every index is in bounds for the key.
            let frame_index_from_key = (verification_key[indices[0]] % 16) as usize;

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

    /// Download the key-byte indices used by the generator.
    ///
    /// For the `client-web` build the indices file URL is already known. For the `x-web` build we
    /// first download the transaction-id chunk to discover the hashed name of the sign module that
    /// holds them.
    async fn download_indices(&self, source: &IndexSource) -> Result<Vec<usize>, Error> {
        let indices_url = match source {
            IndexSource::Direct(url) => url.clone(),
            IndexSource::ViaModule(module_url) => {
                let module = self.download_script(module_url).await?;
                sign_module_url(module_url, &module)?
            }
        };

        let content = self.download_script(&indices_url).await?;

        Ondemand { content }.indices()
    }

    async fn download_script(&self, url: &str) -> Result<String, Error> {
        let response = self
            .underlying
            .get(url)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            Ok(response.text().await?)
        } else {
            Err(Error::Response(response.status()))
        }
    }
}

/// Where the key-byte indices come from, depending on which web client X served.
enum IndexSource {
    /// `client-web`: the URL of the `ondemand.s.<name>a.js` file, built from the home page.
    Direct(String),
    /// `x-web`: the URL of the transaction-id chunk that imports the `sign.o-<hash>.js` module the
    /// indices live in. The module's URL is resolved after downloading this chunk.
    ViaModule(String),
}

struct Home {
    body: String,
    html: Html,
}

impl Home {
    /// Locate the source of the key-byte indices, trying the `client-web` build first and falling
    /// back to `x-web`. Whichever the home page was served as, only one will match.
    fn index_source(&self) -> Result<IndexSource, Error> {
        self.ondemand_url()
            .map(IndexSource::Direct)
            .or_else(|| self.transaction_module_url().map(IndexSource::ViaModule))
            .ok_or(Error::MissingIndexSource)
    }

    // `client-web`: the indices file is named directly in the home page.
    fn ondemand_url(&self) -> Option<String> {
        let name =
            Self::find_ondemand_name_v2(&self.body).or_else(|| Self::find_ondemand_name_v1(&self.body))?;

        Some(format!(
            "https://abs.twimg.com/responsive-web/client-web/ondemand.s.{name}a.js"
        ))
    }

    // Older format, seen until around 2026-03-17.
    fn find_ondemand_name_v1(content: &str) -> Option<&str> {
        ONDEMAND_NAME_V1_RE
            .captures(content)
            .and_then(|captures| captures.get(1))
            .map(|name_match| name_match.as_str())
    }

    // Newer format, first seen 2026-03-18.
    fn find_ondemand_name_v2(content: &str) -> Option<&str> {
        let index = ONDEMAND_NAME_V2_INDEX_RE
            .captures(content)
            .and_then(|captures| captures.get(1))
            .and_then(|index_match| index_match.as_str().parse::<u32>().ok())?;

        let name_re = Regex::new(&format!(r#",\s*{index}\s*:\s*['"]([a-f0-9]+)['"]"#))
            .expect("Invalid regex (should never happen)");

        name_re
            .captures(content)
            .and_then(|captures| captures.get(1))
            .map(|name_match| name_match.as_str())
    }

    // `x-web`: the indices file (`sign.o-<hash>.js`) is imported by a preloaded transaction-id
    // chunk, whose URL we return here. NB: this depends on that chunk's source name
    // (`sentry-filter`), which may change across builds.
    fn transaction_module_url(&self) -> Option<String> {
        TRANSACTION_MODULE_RE
            .find(&self.body)
            .map(|module_match| module_match.as_str().to_string())
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

/// Resolve the absolute URL of the sign module (`sign.o-<hash>.js`) given the transaction chunk
/// it's imported from and that chunk's source. The module is a sibling of the chunk, so we reuse
/// the chunk URL's directory.
fn sign_module_url(module_url: &str, module: &str) -> Result<String, Error> {
    let name = SIGN_MODULE_RE
        .find(module)
        .map(|name_match| name_match.as_str())
        .ok_or(Error::MissingSignModule)?;

    let base = module_url
        .rfind('/')
        .map(|index| &module_url[..=index])
        .ok_or(Error::MissingSignModule)?;

    Ok(format!("{base}{name}"))
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

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time regression check: `scraper::Html` is not `Sync`, so holding
    // it across an await would make every `generate` future `!Send` and
    // unusable with `tokio::spawn` on a multithreaded runtime.
    #[test]
    fn generate_futures_are_send() {
        fn assert_send<T: Send>(_value: &T) {}

        let client = Client::default();
        let endpoint = crate::Endpoint::new("Name", "version");

        assert_send(&client.generate(&endpoint));
        assert_send(&client.generate_for_path("/path"));
        assert_send(&client.generate_batch(&[&endpoint]));
        assert_send(&client.get_site_info());
    }

    // X serves either the `client-web` or `x-web` build depending on the day, so the index source
    // must be located from both home-page shapes. These guard the parsing offline, since the live
    // integration test only ever exercises whichever build is currently served.

    #[test]
    fn client_web_index_source() {
        // The `ondemand.s` chunk is named directly in the home page (newer "v2" format): an index
        // (`59924`) maps to the `ondemand.s` label in one object and to the file's hash in another.
        let body = r#""a",59924:"ondemand.s","b","c",59924:"479fb39","d""#;
        let home = Home {
            body: body.to_string(),
            html: Html::parse_document(body),
        };

        match home.index_source().unwrap() {
            IndexSource::Direct(url) => assert_eq!(
                url,
                "https://abs.twimg.com/responsive-web/client-web/ondemand.s.479fb39a.js"
            ),
            IndexSource::ViaModule(url) => panic!("expected client-web direct URL, got {url}"),
        }
    }

    #[test]
    fn x_web_index_source() {
        // No `ondemand.s`; instead a preloaded transaction-id chunk that imports the sign module.
        let body = r#"<link rel="modulepreload" href="https://abs.twimg.com/x-web/x-web/assets/sentry-filter-B3LxlQus.js"/>"#;
        let home = Home {
            body: body.to_string(),
            html: Html::parse_document(body),
        };

        let module_url = match home.index_source().unwrap() {
            IndexSource::ViaModule(url) => {
                assert_eq!(
                    url,
                    "https://abs.twimg.com/x-web/x-web/assets/sentry-filter-B3LxlQus.js"
                );
                url
            }
            IndexSource::Direct(url) => panic!("expected x-web module URL, got {url}"),
        };

        // The sign module is resolved as a sibling of the chunk, from the chunk's `import(...)`.
        let chunk = r"...import(`./sign.o-DZxHycaM.js`).then(e=>e.default())...";
        assert_eq!(
            sign_module_url(&module_url, chunk).unwrap(),
            "https://abs.twimg.com/x-web/x-web/assets/sign.o-DZxHycaM.js"
        );
    }

    #[test]
    fn indices_parsed_in_document_order() {
        let content = "var x=[M(t[21],16),N(M(t[14],16),M(t[45],16)),P(t[19],16)];";

        let indices = Ondemand {
            content: content.to_string(),
        }
        .indices()
        .unwrap();

        assert_eq!(indices, vec![21, 14, 45, 19]);
    }
}

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

// The transaction-id chunk that the home page preloads. It dynamically imports the sign module
// (below), whose hashed name isn't referenced anywhere else on the page.
static TRANSACTION_MODULE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://abs\.twimg\.com/[\w./-]*?sentry-filter-[\w-]+\.js").unwrap()
});

// The sign module (`sign.o-<hash>.js`), which contains the key-byte indices. It replaced the old
// `ondemand.s.<name>a.js` file when X migrated to `x-web` (June 2026).
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
    #[error("Missing transaction module URL")]
    MissingTransactionModule,
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
        let (module_url, verification_key, frame_array) = {
            let home = self.download_home().await?;
            let module_url = home.transaction_module_url()?;
            let verification_key = home.site_verification_key()?;
            let frame_index =
                verification_key
                    .get(5)
                    .ok_or_else(|| Error::ShortSiteVerificationKey {
                        length: verification_key.len(),
                    })?
                    % 4;
            let frame_array = home.frame_array(frame_index as usize)?;

            (module_url, verification_key, frame_array)
        };

        let indices = self.download_indices(&module_url).await?;

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
    async fn download_indices(&self, module_url: &str) -> Result<Vec<usize>, Error> {
        let module = self.download_script(module_url).await?;
        let sign_url = sign_module_url(module_url, &module)?;
        let sign = self.download_script(&sign_url).await?;

        Ondemand { content: sign }.indices()
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

struct Home {
    body: String,
    html: Html,
}

impl Home {
    // X migrated from the webpack `client-web` build (which referenced an
    // `ondemand.s.<name>a.js` file directly in the home page, in formats seen until
    // 2026-03-17 and from 2026-03-18) to the Vite-based `x-web` build on 2026-06-23.
    // The indices now live in a `sign.o-<hash>.js` module that's only reachable via
    // the preloaded transaction-id chunk.
    fn transaction_module_url(&self) -> Result<String, Error> {
        TRANSACTION_MODULE_RE
            .find(&self.body)
            .map(|module_match| module_match.as_str().to_string())
            .ok_or(Error::MissingTransactionModule)
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
}

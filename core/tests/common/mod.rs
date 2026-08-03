//! Helpers shared by the archived-snapshot round-trip tests.
//!
//! Each test names Wayback Machine captures by archived URL, capture timestamp, and expected
//! SHA-1 digest, and `archivindex-wbm-test-data` downloads them on the first run into a gitignored
//! cache, verifying the digest on every later run. A capture the archive cannot serve is skipped
//! with a logged warning instead of failing the test, and is tried again on the next run.
use std::fmt::{Display, Formatter};

/// A Wayback Machine capture of one tweet's JSON snapshot.
pub struct Capture {
    /// The tweet author's screen name at capture time, the first component of the archived URL.
    pub screen_name: &'static str,
    /// The tweet's ID, the second component of the archived URL.
    pub id: u64,
    /// The capture timestamp (`YYYYMMDDhhmmss`).
    pub timestamp: &'static str,
    /// The SHA-1 digest of the capture's bytes, in the archive's base-32 rendering.
    pub digest: &'static str,
}

impl Capture {
    /// The URL the capture was archived under.
    fn url(&self) -> String {
        format!(
            "https://twitter.com/{}/status/{}",
            self.screen_name, self.id
        )
    }
}

/// Shown in assertion messages, where the URL identifies the capture most directly.
impl Display for Capture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url())
    }
}

/// Opens the shared download-once snapshot cache.
pub fn cache() -> archivindex_wbm_test_data::Cache {
    // Cargo runs a crate's tests with the working directory set to the package root, so this
    // resolves to `core/tests/data/.cache`, which the workspace `.gitignore` excludes.
    archivindex_wbm_test_data::Cache::new("tests/data/.cache").expect("Cannot build HTTP client")
}

/// Returns the capture's text, downloading it on the first call and verifying the saved file's
/// digest on every later one.
///
/// `None` means the snapshot is unavailable: a warning has been logged, the capture should be
/// skipped, and the next run will try the download again.
pub async fn fetch(cache: &archivindex_wbm_test_data::Cache, capture: &Capture) -> Option<String> {
    let timestamp = capture
        .timestamp
        .parse()
        .expect("Invalid capture timestamp");
    let digest = capture.digest.parse().expect("Invalid capture digest");

    let bytes = cache
        .bytes(&capture.url(), timestamp, digest)
        .await
        .expect("Unexpected I/O error")?;

    Some(
        String::from_utf8(bytes.to_vec())
            .unwrap_or_else(|error| panic!("{capture}: snapshot is not UTF-8: {error}")),
    )
}

/// Asserts that the emitted JSON matches the archived JSON after normalization.
///
/// This complements the typed round-trip comparison the tests perform first: value equality alone
/// cannot see a changed JSON shape, since (for example) a two-element array emitted as a
/// `start`/`end` map reparses to an equal value.
pub fn assert_wire_format(original: &str, serialized: &str, capture: &Capture) {
    let mut original =
        serde_json::from_str::<serde_json::Value>(original).expect("valid input JSON");
    let mut emitted =
        serde_json::from_str::<serde_json::Value>(serialized).expect("valid output JSON");
    normalize(&mut original);
    normalize(&mut emitted);
    assert_eq!(original, emitted, "{capture}: wire-format mismatch");
}

/// The data-format fields whose RFC 3339 timestamps lose their zero subsecond part on the way
/// through the model (see [`normalize`]).
const TIMESTAMP_KEYS: [&str; 3] = ["created_at", "editable_until", "end_datetime"];

/// Recursively drops object entries whose value is `null` and rewrites known aliased spellings.
///
/// Three intentional model normalizations would otherwise read as wire-format changes: absent
/// optional fields are serialized as explicit nulls rather than omitted; the media variant
/// `bitrate` key of older snapshots is accepted as a `serde` alias but always re-emitted in the
/// newer `bit_rate` spelling; and the data format's timestamps are stored as plain
/// `chrono::DateTime` values, whose serializer drops the always-zero `.000` subsecond part the API
/// emits. Undoing each on both sides is what lets a re-serialized snapshot be compared against the
/// archived JSON.
fn normalize(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(fields) => {
            fields.retain(|_, field| !field.is_null());

            if let Some(bit_rate) = fields.remove("bitrate") {
                fields.insert("bit_rate".to_owned(), bit_rate);
            }

            for key in TIMESTAMP_KEYS {
                if let Some(serde_json::Value::String(timestamp)) = fields.get_mut(key)
                    && let Some(seconds) = timestamp.strip_suffix(".000Z")
                {
                    *timestamp = format!("{seconds}Z");
                }
            }

            for field in fields.values_mut() {
                normalize(field);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                normalize(item);
            }
        }
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => {}
    }
}

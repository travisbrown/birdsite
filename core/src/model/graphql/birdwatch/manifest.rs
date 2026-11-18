use std::path::{Path, PathBuf};

use jiff::Timestamp;
use jiff::civil::Date;
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid URL: {0}")]
    InvalidUrl(Url),
    #[error("Missing URLs")]
    MissingUrls,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileSet {
    #[serde(with = "jiff::fmt::serde::timestamp::millisecond::required")]
    pub created_at: Timestamp,
    pub urls: Vec<Url>,
}

impl FileSet {
    pub fn downloads<P: AsRef<Path>>(&self, base: P) -> Result<Vec<(url::Url, PathBuf)>, Error> {
        self.urls
            .iter()
            .map(|url| {
                let mut path_segments = url
                    .path_segments()
                    .ok_or_else(|| Error::InvalidUrl(url.clone()))?
                    .collect::<Vec<_>>();

                let filename = path_segments
                    .pop()
                    .filter(|segment| !segment.is_empty())
                    .ok_or_else(|| Error::InvalidUrl(url.clone()))?;

                let dirname = path_segments
                    .pop()
                    .filter(|segment| !segment.is_empty())
                    .ok_or_else(|| Error::InvalidUrl(url.clone()))?;

                Ok((url.clone(), base.as_ref().join(dirname).join(filename)))
            })
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bundle {
    pub notes: FileSet,
    pub ratings: FileSet,
    pub note_status_history: FileSet,
    pub user_enrollment: FileSet,
    pub bat_signals: Option<FileSet>,
}

impl Bundle {
    pub fn downloads<P: AsRef<Path>>(&self, base: P) -> Result<Vec<(url::Url, PathBuf)>, Error> {
        let mut result = Vec::with_capacity(13);
        result.extend(self.notes.downloads(&base)?);
        result.extend(self.ratings.downloads(&base)?);
        result.extend(self.note_status_history.downloads(&base)?);
        result.extend(self.user_enrollment.downloads(&base)?);

        if let Some(bat_signals) = &self.bat_signals {
            result.extend(bat_signals.downloads(&base)?);
        }
        Ok(result)
    }

    pub fn date(&self) -> Result<Date, Error> {
        let url = self
            .notes
            .urls
            .iter()
            .chain(&self.ratings.urls)
            .chain(&self.note_status_history.urls)
            .chain(&self.user_enrollment.urls)
            .next()
            .ok_or(Error::MissingUrls)?;

        let segments = url
            .path_segments()
            .map_or_default(std::iter::Iterator::collect::<Vec<_>>);

        if segments.len() < 4 {
            Err(Error::InvalidUrl(url.clone()))
        } else {
            segments[1]
                .parse::<i16>()
                .ok()
                .zip(segments[2].parse::<i8>().ok())
                .zip(segments[3].parse::<i8>().ok())
                .and_then(|((year, month), day)| Date::new(year, month, day).ok())
                .ok_or_else(|| Error::InvalidUrl(url.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    struct Manifest {
        birdwatch_latest_public_data_file_bundle: super::Bundle,
    }

    #[test]
    fn deserialize_birdwatch_examples() {
        let line = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/graphql/birdwatch-manifest-2025-08-28.json"
        ));

        let result = serde_json::from_str::<Manifest>(line);

        assert!(result.is_ok());
    }

    #[test]
    fn bundle_date_validates_calendar_dates() {
        let mut bundle: super::Bundle = serde_json::from_value(serde_json::json!({
            "notes": {"created_at": 0, "urls": []},
            "ratings": {"created_at": 0, "urls": []},
            "note_status_history": {"created_at": 0, "urls": []},
            "user_enrollment": {"created_at": 0, "urls": []}
        }))
        .unwrap();

        assert!(matches!(bundle.date(), Err(super::Error::MissingUrls)));
        bundle.notes.urls = vec![
            "https://example.com/data/2024/02/29/notes.tsv"
                .parse()
                .unwrap(),
        ];
        assert_eq!(bundle.date().unwrap(), jiff::civil::date(2024, 2, 29));

        for path in ["2023/02/29", "2024/13/01", "10000/01/01", "2024/02"] {
            bundle.notes.urls = vec![
                format!("https://example.com/data/{path}/notes.tsv")
                    .parse()
                    .unwrap(),
            ];
            assert!(matches!(bundle.date(), Err(super::Error::InvalidUrl(_))));
        }
    }
}

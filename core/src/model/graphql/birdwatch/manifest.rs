use chrono::{DateTime, NaiveDate, Utc};
use std::path::{Path, PathBuf};
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid URL error")]
    InvalidUrl(Url),
    #[error("Missing URLs")]
    MissingUrls,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileSet {
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
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
}

impl Bundle {
    pub fn downloads<P: AsRef<Path>>(&self, base: P) -> Result<Vec<(url::Url, PathBuf)>, Error> {
        let mut result = Vec::with_capacity(13);
        result.extend(self.notes.downloads(&base)?);
        result.extend(self.ratings.downloads(&base)?);
        result.extend(self.note_status_history.downloads(&base)?);
        result.extend(self.user_enrollment.downloads(&base)?);
        Ok(result)
    }

    pub fn date(&self) -> Result<NaiveDate, Error> {
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
            .map(|segments| segments.collect::<Vec<_>>())
            .unwrap_or_default();

        if segments.len() < 4 {
            Err(Error::InvalidUrl(url.clone()))
        } else {
            segments[1]
                .parse::<i32>()
                .ok()
                .zip(segments[2].parse::<u32>().ok())
                .zip(segments[3].parse::<u32>().ok())
                .and_then(|((year, month), day)| NaiveDate::from_ymd_opt(year, month, day))
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
        let line =
            include_str!("../../../../../examples/graphql/birdwatch-manifest-2025-08-28.json");

        let result = serde_json::from_str::<Manifest>(line);

        assert!(result.is_ok());
    }
}

use crate::model::{entity::TypedEntity, graphql::text::Text};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::borrow::Cow;

pub mod model;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Note<'a> {
    Available {
        metadata: NoteMetadata<'a>,
        data: Box<NoteData<'a>>,
    },
    Unavailable {
        metadata: NoteMetadata<'a>,
    },
    Empty {
        id: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoteMetadata<'a> {
    pub id: u64,
    pub decided_by: Option<model::Model>,
    pub fully_visible_model: Option<bool>,
    pub tombstone: Option<Text<'a>>,
    pub rating_status: Option<RatingStatus>,
    pub rating_survey: Option<crate::model::url::Url<'a>>,
    pub helpful_tags: Option<Vec<HelpfulTag>>,
    pub not_helpful_tags: Option<Vec<NotHelpfulTag>>,
    pub created_at: Option<DateTime<Utc>>,
    pub can_appeal: Option<bool>,
    pub appeal_status: Option<AppealStatus>,
    pub is_media_note: Option<bool>,
    pub media_note_matches: Option<usize>,
    pub media_note_matches_v2: Option<MediaNoteMatchesV2>,
    pub is_api_author: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoteData<'a> {
    pub classification: Classification,
    pub summary: Summary<'a>,
    pub misleading_tags: Option<Vec<MisleadingTag>>,
    pub not_misleading_tags: Option<Vec<NotMisleadingTag>>,
    pub trustworthy_sources: bool,
    pub tweet_id: Option<u64>,
    pub media_note_category: Option<MediaNoteCategory>,
    pub profile: Option<super::profile::Profile<'a>>,
    pub is_in_account_language: Option<bool>,
    pub show_matched_parent_note: Option<bool>,
}

impl<'de: 'a, 'a> Deserialize<'de> for Note<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let note = internal::Note::deserialize(deserializer)?;

        if note.is_empty() {
            Ok(Self::Empty { id: note.rest_id })
        } else {
            let metadata = NoteMetadata {
                id: note.rest_id,
                decided_by: note.decided_by,
                fully_visible_model: note.fully_visible_model,
                tombstone: note.tombstone,
                rating_status: note.rating_status,
                rating_survey: note.rating_survey,
                helpful_tags: note.helpful_tags,
                not_helpful_tags: note.not_helpful_tags,
                created_at: note.created_at,
                can_appeal: note.can_appeal,
                appeal_status: note.appeal_status,
                is_media_note: note.is_media_note,
                media_note_matches: note.media_note_matches,
                media_note_matches_v2: note.media_note_matches_v2,
                is_api_author: note.is_api_author,
            };

            let data = note.data_v1.and_then(|data_v1| match data_v1 {
                internal::DataV1::Available {
                    classification,
                    summary,
                    misleading_tags,
                    not_misleading_tags,
                    trustworthy_sources,
                } => {
                    let tweet_result = note
                        .tweet_results
                        .and_then(|tweet_results| tweet_results.result);

                    Some(NoteData {
                        classification,
                        summary,
                        misleading_tags,
                        not_misleading_tags,
                        trustworthy_sources,
                        tweet_id: tweet_result.as_ref().and_then(|result| result.rest_id),
                        media_note_category: tweet_result
                            .and_then(|result| result.media_note_category),
                        profile: note.birdwatch_profile,
                        is_in_account_language: note.is_in_account_language,
                        show_matched_parent_note: note.show_matched_parent_note,
                    })
                }
                internal::DataV1::Unavailable {} => None,
            });

            Ok(match data {
                Some(data) => Self::Available {
                    metadata,
                    data: Box::new(data),
                },
                None => Self::Unavailable { metadata },
            })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Summary<'a> {
    pub text: Cow<'a, str>,
    #[serde(borrow)]
    pub entities: Vec<TypedEntity<'a>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Classification {
    NotMisleading,
    MisinformedOrPotentiallyMisleading,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MisleadingTag {
    DisputedClaimAsFact,
    FactualError,
    ManipulatedMedia,
    MisinterpretedSatire,
    MissingImportantContext,
    OutdatedInformation,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum NotMisleadingTag {
    ClearlySatire,
    FactuallyCorrect,
    Other,
    OutdatedNowButNotWhenWritten,
    PersonalOpinion,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RatingStatus {
    NeedsMoreRatings,
    CurrentlyRatedNotHelpful,
    CurrentlyRatedHelpful,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HelpfulTag {
    AddressesClaim,
    Clear,
    Empathetic,
    GoodSources,
    ImportantContext,
    Informative,
    UnbiasedLanguage,
    UniqueContext,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum NotHelpfulTag {
    Incorrect,
    IrrelevantSources,
    MissingKeyPoints,
    NoSources,
    NoteNotNeeded,
    OffTopic,
    OpinionSpeculation,
    Other,
    Rude,
    TwitterViolationAny,
    Unclear,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MediaNoteCategory {
    Unsupported,
    SingleImage,
    SingleVideo,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AppealStatus {
    NotAppealed,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MediaNoteMatchesV2 {
    pub match_count: usize,
    pub shoud_show_matches: bool,
}

mod internal {
    use crate::model::graphql::text::Text;
    use chrono::{DateTime, Utc, serde::ts_milliseconds_option};
    use serde_field_attributes::{integer_str, optional_integer_str};

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct Note<'a> {
        #[serde(with = "integer_str")]
        pub(super) rest_id: u64,
        #[serde(borrow)]
        pub(super) data_v1: Option<DataV1<'a>>,
        pub(super) decided_by: Option<super::model::Model>,
        pub(super) fully_visible_model: Option<bool>,
        pub(super) tombstone: Option<Text<'a>>,
        pub(super) rating_status: Option<super::RatingStatus>,
        pub(super) rating_survey: Option<crate::model::url::Url<'a>>,
        pub(super) helpful_tags: Option<Vec<super::HelpfulTag>>,
        pub(super) not_helpful_tags: Option<Vec<super::NotHelpfulTag>>,
        pub(super) tweet_results: Option<TweetResults>,
        pub(super) birdwatch_profile: Option<super::super::profile::Profile<'a>>,
        #[serde(default, with = "ts_milliseconds_option")]
        pub(super) created_at: Option<DateTime<Utc>>,
        pub(super) can_appeal: Option<bool>,
        pub(super) appeal_status: Option<super::AppealStatus>,
        pub(super) is_media_note: Option<bool>,
        #[serde(default, with = "optional_integer_str")]
        pub(super) media_note_matches: Option<usize>,
        pub(super) media_note_matches_v2: Option<super::MediaNoteMatchesV2>,
        pub(super) is_in_account_language: Option<bool>,
        pub(super) is_api_author: Option<bool>,
        pub(super) show_matched_parent_note: Option<bool>,
    }

    impl Note<'_> {
        pub(super) fn is_empty(&self) -> bool {
            (self.data_v1.is_none() || self.created_at.is_none())
                && (self.can_appeal == Some(false) || self.can_appeal.is_none())
                && (self.appeal_status == Some(super::AppealStatus::NotAppealed)
                    || self.appeal_status.is_none())
                && (self.is_media_note == Some(false) || self.is_media_note.is_none())
                && (self.media_note_matches == Some(0) || self.media_note_matches.is_none())
                && self.is_in_account_language.is_none()
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    #[serde(untagged, deny_unknown_fields)]
    pub(super) enum DataV1<'a> {
        Available {
            classification: super::Classification,
            #[serde(borrow)]
            summary: super::Summary<'a>,
            misleading_tags: Option<Vec<super::MisleadingTag>>,
            not_misleading_tags: Option<Vec<super::NotMisleadingTag>>,
            trustworthy_sources: bool,
        },
        Unavailable {},
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct TweetResults {
        pub(super) result: Option<TweetResultsResult>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub(super) struct TweetResultsResult {
        #[serde(with = "optional_integer_str", default)]
        pub(super) rest_id: Option<u64>,
        pub(super) media_note_category: Option<super::MediaNoteCategory>,
    }
}

#[cfg(test)]
mod tests {
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    struct BirdwatchNote<'a> {
        #[serde(borrow)]
        birdwatch_note_by_rest_id: super::Note<'a>,
    }

    #[test]
    fn deserialize_birdwatch_examples() {
        let lines =
            include_str!("../../../../../../examples/graphql/birdwatch-notes-2025-08-28.ndjson")
                .split("\n")
                .filter(|line| !line.is_empty());

        for (i, line) in lines.enumerate() {
            let result = serde_json::from_str::<BirdwatchNote<'_>>(line);

            if let Err(error) = &result {
                println!(
                    "Line {}: {line:?} is an invalid note object: {error}",
                    i + 1
                );
            }

            assert!(result.is_ok());
        }
    }
}

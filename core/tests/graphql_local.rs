//! Deserializes real archived GraphQL exchanges (kept outside version control) with the typed
//! response models.
//!
//! The files under `tests/data/local/` (gitignored, so the test is a no-op on a fresh checkout)
//! are single lines from the archived exchange corpus, each named
//! `<request-name>-<timestamp_ms>.json` after its content, and each is parsed with the model
//! matching its request name. They were selected to maximize model coverage over the corpus,
//! within the eras the models support: user results from the legacy era (before screen names
//! moved from `legacy` into `core` around 22 May 2025) with every observed unavailability reason
//! and the empty-`data` case; the `CommunityUnavailable` community variant (the corpus's
//! available-community records all carry GraphQL-internal `id` and `__typename` decorations the
//! model does not yet support, so the committed fixtures remain the coverage for available
//! communities); Birdwatch notes covering both classifications and every observed tag, status,
//! and metadata field; and both Birdwatch manifest bundle shapes.
use birdsite::model::graphql::birdwatch::manifest::Bundle;
use birdsite::model::graphql::birdwatch::note::Note;
use birdsite::model::graphql::community::CommunityResult;
use birdsite::model::graphql::user::{User, UserResult};

/// The portion of an exchange line the model tests care about; the `request` and `errors` fields
/// are exercised by the `birdsite-graphql` crate's tests instead.
#[derive(serde::Deserialize)]
struct Exchange<D> {
    data: Option<D>,
}

/// A result wrapper that tolerates the sibling fields (such as `id`) of some eras.
#[derive(serde::Deserialize)]
struct Entry<A> {
    result: Option<A>,
}

/// `UsersByRestIds` responses.
#[derive(serde::Deserialize)]
struct UsersData<'a> {
    #[serde(borrow)]
    users: Vec<Entry<UserResult<'a, User<'a>>>>,
}

/// `UserByRestId` and `UserByScreenName` responses; the `user` field is absent when the account
/// does not exist.
#[derive(serde::Deserialize)]
struct UserData<'a> {
    #[serde(borrow, default)]
    user: Option<Entry<UserResult<'a, User<'a>>>>,
}

/// `CommunitiesFetchOneQuery` responses.
#[derive(serde::Deserialize)]
struct CommunityData<'a> {
    #[serde(rename = "communityResults", borrow)]
    community_results: CommunityResults<'a>,
}

#[derive(serde::Deserialize)]
struct CommunityResults<'a> {
    #[serde(borrow)]
    result: Option<CommunityResult<'a, User<'a>>>,
}

/// `CommunitiesSearchQuery` responses.
#[derive(serde::Deserialize)]
struct CommunitySearchData<'a> {
    #[serde(borrow)]
    communities_search_slice: CommunitySearchSlice<'a>,
}

#[derive(serde::Deserialize)]
struct CommunitySearchSlice<'a> {
    #[serde(borrow)]
    items_results: Vec<CommunityResults<'a>>,
}

/// `BirdwatchFetchOneNote` responses.
#[derive(serde::Deserialize)]
struct NoteData<'a> {
    #[serde(borrow)]
    birdwatch_note_by_rest_id: Note<'a>,
}

/// `BirdwatchFetchPublicData` responses.
#[derive(serde::Deserialize)]
struct ManifestData {
    birdwatch_latest_public_data_file_bundle: Bundle,
}

fn parse<'a, D: serde::de::Deserialize<'a>>(contents: &'a str, name: &str) -> Option<D> {
    serde_json::from_str::<Exchange<D>>(contents)
        .unwrap_or_else(|error| panic!("{name}: invalid example: {error}"))
        .data
}

#[test]
fn deserialize_local_graphql_examples() {
    // Kept out of version control, so the test is a no-op when the directory is absent rather
    // than a hard failure.
    let Ok(entries) = std::fs::read_dir("tests/data/local") else {
        return;
    };

    let mut checked = 0;
    for entry in entries {
        let path = entry.expect("Invalid directory entry").path();
        let contents = std::fs::read_to_string(&path).expect("Invalid example file");
        let label = path.display().to_string();
        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .and_then(|stem| stem.rsplit_once('-'))
            .map(|(name, _)| name.to_owned())
            .expect("Invalid file name");

        match name.as_str() {
            "UsersByRestIds" => {
                let data: Option<UsersData<'_>> = parse(&contents, &label);
                assert!(
                    data.expect("missing data")
                        .users
                        .iter()
                        .any(|entry| entry.result.is_some()),
                    "{label}: no user results"
                );
            }
            "UserByRestId" | "UserByScreenName" => {
                let data: Option<UserData<'_>> = parse(&contents, &label);
                // The `user` field and its result are legitimately absent for missing accounts.
                let _result = data
                    .and_then(|data| data.user)
                    .and_then(|entry| entry.result);
            }
            "CommunitiesFetchOneQuery" => {
                let data: Option<CommunityData<'_>> = parse(&contents, &label);
                let _result = data.and_then(|data| data.community_results.result);
            }
            "CommunitiesSearchQuery" => {
                let data: Option<CommunitySearchData<'_>> = parse(&contents, &label);
                assert!(
                    data.expect("missing data")
                        .communities_search_slice
                        .items_results
                        .iter()
                        .any(|entry| entry.result.is_some()),
                    "{label}: no community results"
                );
            }
            "BirdwatchFetchOneNote" => {
                let data: Option<NoteData<'_>> = parse(&contents, &label);
                let _note = data.expect("missing data").birdwatch_note_by_rest_id;
            }
            "BirdwatchFetchPublicData" => {
                let data: Option<ManifestData> = parse(&contents, &label);
                let _bundle = data
                    .expect("missing data")
                    .birdwatch_latest_public_data_file_bundle;
            }
            other => panic!("{}: unexpected example kind {other}", path.display()),
        }

        checked += 1;
    }

    assert!(checked > 0, "Example directory present but empty");
}

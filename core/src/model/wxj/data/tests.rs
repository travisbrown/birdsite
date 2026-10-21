use std::borrow::Cow;

use serde_json::{Value, json};

use super::{Article, NoteRequestSuggestion, TweetSnapshot, VerifiedType};

const SNAPSHOT: &str = concat!(
    r#"{"data":{"author_id":"1","conversation_id":"2","#,
    r#""created_at":"2026-05-19T13:42:45.000Z","id":"2","lang":"en","#,
    r#""possibly_sensitive":false,"public_metrics":{"retweet_count":0,"#,
    r#""reply_count":0,"like_count":0,"quote_count":0},"#,
    r#""reply_settings":"everyone","text":"hello world"},"#,
    r#""includes":{"users":[]}}"#
);

/// Regression: `Tweet::text` must deserialize as `Cow::Borrowed` for escape-free input,
/// confirming the `#[serde(borrow)]` attribute preserves the crate's zero-copy design.
#[test]
fn tweet_text_borrows_when_unescaped() {
    let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(SNAPSHOT).unwrap();

    assert!(matches!(snapshot.data.text, Cow::Borrowed(_)));
}

#[test]
fn older_snapshots_omit_new_tweet_and_article_fields() {
    let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(SNAPSHOT).unwrap();

    assert_eq!(snapshot.data.display_text_range, None);
    assert_eq!(snapshot.data.paid_partnership, None);
    assert_eq!(snapshot.data.media_metadata, None);
    assert_eq!(snapshot.data.card_uri, None);
    assert_eq!(snapshot.data.community_id, None);
    assert_eq!(snapshot.data.note_request_suggestions, None);
    assert_eq!(snapshot.data.suggested_source_links, None);
    assert_eq!(snapshot.data.suggested_source_links_with_counts, None);

    for json in ["{}", r#"{"title":"Article title"}"#] {
        let article = serde_json::from_str::<Article<'_>>(json).unwrap();

        assert_eq!(article.cover_media, None);
        assert_eq!(article.entities, None);
        assert_eq!(article.media_entities, None);
        assert_eq!(article.plain_text, None);
        assert_eq!(article.preview_text, None);
        assert_eq!(article.title.is_some(), json.contains("title"));
    }
}

fn snapshot_with_new_fields() -> Value {
    let mut snapshot: Value = serde_json::from_str(SNAPSHOT).unwrap();
    let tweet = snapshot["data"].as_object_mut().unwrap();
    tweet.extend(
        json!({
            "card_uri": "card://123",
            "community_id": "18446744073709551615",
            "display_text_range": [1, 11],
            "paid_partnership": false,
            "media_metadata": [
                {"media_key": "3_123", "alt_text": "A photo", "description": "", "title": ""},
                {"media_key": "3_456"}
            ],
            "note_request_suggestions": [
                {
                    "source_link": "https://example.com/source",
                    "suggestion": "Check the source",
                    "suggestion_id": "18446744073709551615"
                },
                {"source_link": null, "suggestion": "An older video", "suggestion_id": "4"}
            ],
            "suggested_source_links": ["https://example.com/source"],
            "suggested_source_links_with_counts": [
                {"count": "3", "url": "https://example.com/source"}
            ],
            "article": {
                "title": "Article title",
                "cover_media": "3_123",
                "plain_text": "",
                "preview_text": "Preview text",
                "media_entities": ["3_456"],
                "entities": {
                    "hashtags": [{"start": 0, "end": 8, "text": "Example"}],
                    "mentions": [{"start": 0, "end": 8, "username": "example"}],
                    "tweets": [{"id": "18446744073709551615"}, {"id": "3"}],
                    "urls": [{"text": "https://example.com/"}]
                }
            }
        })
        .as_object()
        .unwrap()
        .clone(),
    );
    snapshot["includes"]["tweets"] = json!([snapshot["data"]]);
    snapshot["includes"]["users"] = json!([{
        "id": "1",
        "username": "example",
        "name": "Example",
        "created_at": "2020-01-01T00:00:00Z",
        "description": "",
        "profile_image_url": "https://example.com/profile.jpg",
        "verified": true,
        "protected": false,
        "public_metrics": {"tweet_count": 1},
        "affiliation": {
            "badge_url": "https://example.com/badge.jpg",
            "description": "Example organization",
            "url": "https://example.com/",
            "user_id": ["3"]
        },
        "is_identity_verified": false,
        "most_recent_tweet_id": "18446744073709551615",
        "profile_banner_url": "https://example.com/banner.jpg",
        "verified_type": "blue"
    }]);
    snapshot
}

/// Both the main tweet and included tweets must preserve the new fields and their wire shapes.
#[test]
fn new_tweet_and_article_fields_round_trip() {
    let input = snapshot_with_new_fields();
    let json = input.to_string();
    let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&json).unwrap();

    assert_eq!(snapshot.data.display_text_range, Some(1..11));
    assert_eq!(snapshot.data.paid_partnership, Some(false));
    let media = snapshot.data.media_metadata.as_ref().unwrap();
    assert_eq!(media[0].media_key, "3_123");
    assert_eq!(media[0].alt_text.as_deref(), Some("A photo"));
    assert_eq!(media[0].description.as_deref(), Some(""));
    assert_eq!(media[0].title.as_deref(), Some(""));
    assert_eq!(media[1].alt_text, None);
    assert_eq!(media[1].description, None);
    assert_eq!(media[1].title, None);
    assert_eq!(
        snapshot.data.card_uri,
        Some(super::media::Card::Uri(Cow::Borrowed("card://123")))
    );
    assert_eq!(snapshot.data.community_id, Some(u64::MAX));
    let article_entities = snapshot
        .data
        .article
        .as_ref()
        .unwrap()
        .entities
        .as_ref()
        .unwrap();
    let article_tweets = article_entities.tweets.as_ref().unwrap();
    assert_eq!(article_tweets[0].id, u64::MAX);
    assert_eq!(article_tweets[1].id, 3);
    let suggestions = snapshot.data.note_request_suggestions.as_ref().unwrap();
    assert_eq!(suggestions[0].suggestion_id, u64::MAX);
    assert_eq!(
        suggestions[0].suggestion.as_deref(),
        Some("Check the source")
    );
    assert_eq!(suggestions[1].source_link, None);
    assert_eq!(
        snapshot
            .data
            .suggested_source_links_with_counts
            .as_ref()
            .unwrap()[0]
            .count,
        3
    );
    assert_eq!(
        snapshot
            .data
            .article
            .as_ref()
            .unwrap()
            .plain_text
            .as_deref(),
        Some("")
    );
    assert_eq!(snapshot.includes.tweets.as_ref().unwrap()[0], snapshot.data);

    let serialized = serde_json::to_string(&snapshot).unwrap();
    let reparsed = serde_json::from_str::<TweetSnapshot<'_>>(&serialized).unwrap();
    assert_eq!(snapshot, reparsed);
    let emitted: Value = serde_json::from_str(&serialized).unwrap();
    for pointer in ["/data", "/includes/tweets/0"] {
        for field in [
            "display_text_range",
            "paid_partnership",
            "article",
            "card_uri",
            "community_id",
            "note_request_suggestions",
            "suggested_source_links",
            "suggested_source_links_with_counts",
        ] {
            assert_eq!(
                emitted.pointer(pointer).unwrap()[field],
                input["data"][field]
            );
        }
        assert_eq!(
            emitted.pointer(pointer).unwrap()["media_metadata"][0],
            input["data"]["media_metadata"][0]
        );
        assert_eq!(
            emitted.pointer(pointer).unwrap()["media_metadata"][1]["media_key"],
            "3_456"
        );
    }
}

#[test]
fn article_tweets_can_be_absent_or_empty() {
    for (json, expected_count) in [
        (r#"{"entities":{}}"#, None),
        (r#"{"entities":{"tweets":[]}}"#, Some(0)),
    ] {
        let article = serde_json::from_str::<Article<'_>>(json).unwrap();

        assert_eq!(
            article.entities.unwrap().tweets.as_ref().map(Vec::len),
            expected_count
        );
    }
}

#[test]
fn note_request_suggestion_accepts_missing_source_and_borrows_text() {
    let suggestion = serde_json::from_str::<NoteRequestSuggestion<'_>>(
        r#"{"suggestion":"An older video","suggestion_id":"4"}"#,
    )
    .unwrap();

    assert_eq!(suggestion.source_link, None);
    assert_eq!(suggestion.suggestion.as_deref(), Some("An older video"));
    assert!(matches!(suggestion.suggestion, Some(Cow::Borrowed(_))));
    assert_eq!(suggestion.suggestion_id, 4);
}

#[test]
fn new_user_fields_round_trip() {
    for (name, verified_type) in [
        ("none", VerifiedType::None),
        ("blue", VerifiedType::Blue),
        ("business", VerifiedType::Business),
        ("government", VerifiedType::Government),
    ] {
        let mut input = snapshot_with_new_fields();
        input["includes"]["users"][0]["verified_type"] = json!(name);
        let json = input.to_string();
        let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&json).unwrap();
        let user = snapshot.lookup_user(1).unwrap();
        assert_eq!(user.verified_type, Some(verified_type));
        assert_eq!(user.is_identity_verified, Some(false));
        assert_eq!(user.most_recent_tweet_id, Some(u64::MAX));
        assert_eq!(
            user.affiliation.as_ref().unwrap().user_id.as_deref(),
            Some(&[3][..])
        );

        let serialized = serde_json::to_string(&snapshot).unwrap();
        let reparsed = serde_json::from_str::<TweetSnapshot<'_>>(&serialized).unwrap();
        assert_eq!(snapshot, reparsed);
        let emitted: Value = serde_json::from_str(&serialized).unwrap();
        for field in [
            "affiliation",
            "is_identity_verified",
            "most_recent_tweet_id",
            "profile_banner_url",
            "verified_type",
        ] {
            assert_eq!(
                emitted["includes"]["users"][0][field],
                input["includes"]["users"][0][field]
            );
        }
    }
}

/// Older captures lack these fields, while recent users without an affiliation have `{}`.
#[test]
fn user_fields_can_be_absent_and_affiliation_can_be_empty() {
    let mut input = snapshot_with_new_fields();
    let user = input["includes"]["users"][0].as_object_mut().unwrap();
    for field in [
        "affiliation",
        "is_identity_verified",
        "most_recent_tweet_id",
        "profile_banner_url",
        "verified_type",
    ] {
        user.remove(field);
    }
    let json = input.to_string();
    let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&json).unwrap();
    let user = snapshot.lookup_user(1).unwrap();
    assert_eq!(user.affiliation, None);
    assert_eq!(user.is_identity_verified, None);
    assert_eq!(user.most_recent_tweet_id, None);
    assert_eq!(user.profile_banner_url, None);
    assert_eq!(user.verified_type, None);

    input["includes"]["users"][0]["affiliation"] = json!({});
    let json = input.to_string();
    let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&json).unwrap();
    let affiliation = snapshot
        .lookup_user(1)
        .unwrap()
        .affiliation
        .as_ref()
        .unwrap();
    assert_eq!(affiliation.badge_url, None);
    assert_eq!(affiliation.description, None);
    assert_eq!(affiliation.url, None);
    assert_eq!(affiliation.user_id, None);
}

#[test]
fn new_fields_reject_malformed_values() {
    for (pointer, value) in [
        ("/data/card_uri", json!(123)),
        ("/data/article/entities/hashtags/0/text", json!(123)),
        ("/data/article/entities/hashtags/0/start", json!(-1)),
        (
            "/data/article/entities/hashtags/0",
            json!({"start": 0, "end": 8, "tag": "Example"}),
        ),
        ("/data/community_id", json!(123)),
        ("/data/community_id", json!("-1")),
        ("/data/community_id", json!("18446744073709551616")),
        ("/data/article/entities/tweets", json!(["3"])),
        ("/data/article/entities/tweets/0", json!({})),
        ("/data/article/entities/tweets/0/id", json!(3)),
        ("/data/article/entities/tweets/0/id", json!("-1")),
        (
            "/data/article/entities/tweets/0/id",
            json!("18446744073709551616"),
        ),
        ("/data/note_request_suggestions", json!({})),
        ("/data/note_request_suggestions/0/suggestion_id", json!(123)),
        ("/data/note_request_suggestions/0/source_link", json!(123)),
        ("/data/note_request_suggestions/0/suggestion", json!(123)),
        (
            "/data/note_request_suggestions/0",
            json!({"suggestion": "No ID"}),
        ),
        (
            "/data/suggested_source_links",
            json!("https://example.com/"),
        ),
        ("/data/suggested_source_links", json!([123])),
        ("/data/media_metadata/0/description", json!(123)),
        ("/data/media_metadata/0/title", json!(123)),
        ("/data/suggested_source_links_with_counts/0/count", json!(3)),
        (
            "/data/suggested_source_links_with_counts/0/count",
            json!("-1"),
        ),
        (
            "/data/suggested_source_links_with_counts/0/count",
            json!("many"),
        ),
        ("/data/suggested_source_links_with_counts/0/url", json!(3)),
        (
            "/data/suggested_source_links_with_counts/0",
            json!({"count": "3"}),
        ),
        ("/data/display_text_range", json!([0])),
        ("/data/display_text_range", json!([0, 1, 2])),
        ("/data/display_text_range", json!([-1, 2])),
        ("/data/paid_partnership", json!("false")),
        ("/data/media_metadata/0", json!({"alt_text": "No key"})),
        ("/data/media_metadata/0/alt_text", json!(12)),
        ("/data/article/cover_media", json!(123)),
        ("/data/article/media_entities", json!([123])),
        ("/includes/users/0/verified_type", json!("unknown")),
        ("/includes/users/0/is_identity_verified", json!("false")),
        ("/includes/users/0/most_recent_tweet_id", json!(123)),
        ("/includes/users/0/affiliation/user_id", json!("3")),
        ("/includes/users/0/affiliation/user_id", json!([3])),
        (
            "/data/article/entities/urls/0",
            json!({"url": "https://example.com/"}),
        ),
        (
            "/data/article/entities/mentions/0",
            json!({"username": "example"}),
        ),
    ] {
        let mut input = snapshot_with_new_fields();
        *input.pointer_mut(pointer).unwrap() = value;
        assert!(
            serde_json::from_str::<TweetSnapshot<'_>>(&input.to_string()).is_err(),
            "accepted malformed field at {pointer}"
        );
    }
}

#[test]
fn new_objects_reject_unknown_fields() {
    for pointer in [
        "/data/media_metadata/0",
        "/data/note_request_suggestions/0",
        "/data/suggested_source_links_with_counts/0",
        "/data/article",
        "/data/article/entities",
        "/data/article/entities/tweets/0",
        "/data/article/entities/hashtags/0",
        "/data/article/entities/urls/0",
        "/data/article/entities/mentions/0",
        "/includes/users/0/affiliation",
    ] {
        let mut input = snapshot_with_new_fields();
        input.pointer_mut(pointer).unwrap()["unknown"] = json!(true);
        assert!(
            serde_json::from_str::<TweetSnapshot<'_>>(&input.to_string()).is_err(),
            "accepted unknown field at {pointer}"
        );
    }
}

#[test]
fn media_cards_round_trip_and_reject_malformed_objects() {
    for card in [
        json!("https://example.com/card"),
        json!({"media_keys": ["7_123", "13_456"]}),
    ] {
        let mut input = snapshot_with_new_fields();
        for pointer in ["/data/card_uri", "/includes/tweets/0/card_uri"] {
            *input.pointer_mut(pointer).unwrap() = card.clone();
        }
        let text = input.to_string();
        let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&text).unwrap();
        let serialized = serde_json::to_string(&snapshot).unwrap();
        assert_eq!(
            snapshot,
            serde_json::from_str::<TweetSnapshot<'_>>(&serialized).unwrap()
        );
        let emitted: Value = serde_json::from_str(&serialized).unwrap();
        for pointer in ["/data/card_uri", "/includes/tweets/0/card_uri"] {
            assert_eq!(emitted.pointer(pointer), Some(&card));
        }
    }

    for card in [
        json!({}),
        json!({"media_keys": [123]}),
        json!({"media_keys": [], "unknown": true}),
    ] {
        assert!(serde_json::from_str::<super::media::Card<'_>>(&card.to_string()).is_err());
    }
}

#[test]
fn polls_accept_options_without_metadata() {
    for metadata in [
        json!({}),
        json!({
            "id": "123", "voting_status": "closed", "duration_minutes": 60,
            "end_datetime": "2026-10-21T12:00:00Z"
        }),
    ] {
        let mut input = metadata;
        input["options"] = json!([{"label": "Yes", "position": 1, "votes": 47}]);
        let text = input.to_string();
        let poll = serde_json::from_str::<super::Poll<'_>>(&text).unwrap();
        assert_eq!(poll.id.is_some(), input.get("id").is_some());
        assert_eq!(
            poll.voting_status.is_some(),
            input.get("voting_status").is_some()
        );
        assert_eq!(
            poll.duration_minutes.is_some(),
            input.get("duration_minutes").is_some()
        );
        assert_eq!(
            poll.end_datetime.is_some(),
            input.get("end_datetime").is_some()
        );
        assert_eq!(poll.options[0].votes, 47);
        let serialized = serde_json::to_string(&poll).unwrap();
        assert_eq!(
            poll,
            serde_json::from_str::<super::Poll<'_>>(&serialized).unwrap()
        );
        let emitted: Value = serde_json::from_str(&serialized).unwrap();
        for (field, value) in input.as_object().unwrap() {
            assert_eq!(&emitted[field], value);
        }
    }

    for input in [
        json!({}),
        json!({"options": [], "id": 123}),
        json!({"options": [], "voting_status": "invalid"}),
    ] {
        assert!(serde_json::from_str::<super::Poll<'_>>(&input.to_string()).is_err());
    }
}

/// Missing authors are valid snapshot data, but cannot supply required user metadata.
#[test]
fn missing_authors_are_explicit_in_accessors_and_metadata_errors() {
    for pointer in ["/data", "/includes/tweets/0"] {
        let mut input = snapshot_with_new_fields();
        input
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove("author_id");
        input["data"]["referenced_tweets"] = json!([{"type": "retweeted", "id": "2"}]);
        let text = input.to_string();
        let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&text).unwrap();
        assert!(matches!(
            crate::model::wxj::metadata::tweet::TweetMetadata::from_data_tweet_snapshot(&snapshot),
            Err(super::FormatError::MissingAuthor(2))
        ));
        let snapshot = crate::model::wxj::TweetSnapshot::Data(snapshot);
        if pointer == "/data" {
            assert_eq!(snapshot.user_id(), None);
            assert_eq!(snapshot.user_screen_name(), None);
            assert_eq!(snapshot.canonical_url(false), None);
        } else {
            assert_eq!(snapshot.user_id(), Some(1));
            assert_eq!(snapshot.user_screen_name(), Some("example"));
        }
    }
}

#[test]
fn note_request_suggestion_can_omit_text() {
    let suggestion = serde_json::from_str::<NoteRequestSuggestion<'_>>(
        r#"{"source_link":"https://example.com/source","suggestion_id":"4"}"#,
    )
    .unwrap();
    assert_eq!(suggestion.suggestion, None);
    assert_eq!(
        suggestion.source_link.as_deref(),
        Some("https://example.com/source")
    );
    let serialized = serde_json::to_string(&suggestion).unwrap();
    assert_eq!(
        suggestion,
        serde_json::from_str::<NoteRequestSuggestion<'_>>(&serialized).unwrap()
    );
    let escaped = serde_json::from_str::<NoteRequestSuggestion<'_>>(
        r#"{"suggestion":"An older\nvideo","suggestion_id":"4"}"#,
    )
    .unwrap();
    assert_eq!(escaped.suggestion.as_deref(), Some("An older\nvideo"));
}

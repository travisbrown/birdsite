use std::borrow::Cow;

use birdsite::model::KeyValuePair;
use birdsite::model::graphql::timeline::client::event::TransparentGuideDetails;
use birdsite::model::user::entities::Url;
use birdsite::model::user::properties::TipjarSettings;
use birdsite::model::wxj::data::Article;
use birdsite::model::wxj::data::media::Card;
use serde_json::{Value, json};

/// A shared lifetime bound does not replace the borrowing annotation on either direct Cow field.
#[test]
fn both_direct_cow_fields_borrow() {
    let pair: KeyValuePair<'_> = serde_json::from_str(r#"{"key":"key","value":"value"}"#).unwrap();

    assert!(matches!(pair.key, Cow::Borrowed("key")));
    assert!(matches!(pair.value, Cow::Borrowed("value")));
}

/// The direct URL field supplies the lifetime bound for both optional string adapters.
#[test]
fn optional_strings_borrow_and_preserve_their_wire_format() {
    for (expanded, display) in [
        (None, None),
        (Some("null"), Some("null")),
        (Some(r#""expanded""#), Some(r#""display""#)),
        (Some(r#""expanded\nurl""#), Some(r#""display\nurl""#)),
    ] {
        let mut fields = vec![
            r#""url":"short""#.to_owned(),
            r#""indices":[0,5]"#.to_owned(),
        ];
        for (name, value) in [("expanded_url", expanded), ("display_url", display)] {
            if let Some(value) = value {
                fields.push(format!("\"{name}\":{value}"));
            }
        }
        let input = format!("{{{}}}", fields.join(","));
        let url: Url<'_> = serde_json::from_str(&input).unwrap();

        assert!(matches!(url.url, Cow::Borrowed("short")));
        for (value, encoded) in [(&url.expanded_url, expanded), (&url.display_url, display)] {
            match encoded {
                None | Some("null") => assert!(value.is_none()),
                Some(encoded) if encoded.contains('\\') => {
                    assert!(matches!(value, Some(Cow::Owned(_))));
                }
                Some(_) => assert!(matches!(value, Some(Cow::Borrowed(_)))),
            }
        }

        let mut expected: Value = serde_json::from_str(&input).unwrap();
        for name in ["expanded_url", "display_url"] {
            expected
                .as_object_mut()
                .unwrap()
                .entry(name)
                .or_insert(Value::Null);
        }
        assert_eq!(serde_json::to_value(&url).unwrap(), expected);
    }
}

/// Only the first adapter has a borrow annotation, but every present handle should borrow.
#[test]
fn optional_strings_share_one_lifetime_bound() {
    let input = r#"{
        "bandcamp_handle":"bandcamp", "bitcoin_handle":"bitcoin", "cash_app_handle":"cash",
        "ethereum_handle":"ethereum", "gofundme_handle":"gofundme", "patreon_handle":"patreon",
        "pay_pal_handle":"paypal", "venmo_handle":"venmo"
    }"#;
    let settings: TipjarSettings<'_> = serde_json::from_str(input).unwrap();

    for handle in [
        settings.bandcamp_handle,
        settings.bitcoin_handle,
        settings.cash_app_handle,
        settings.ethereum_handle,
        settings.gofundme_handle,
        settings.patreon_handle,
        settings.pay_pal_handle,
        settings.venmo_handle,
    ] {
        assert!(matches!(handle, Some(Cow::Borrowed(_))));
    }
}

/// The URI variant supplies the lifetime bound for the sequence adapter in the other variant.
#[test]
fn strings_borrow_in_an_untagged_enum_sequence() {
    let input = r#"{"media_keys":["plain","escaped\nkey"]}"#;
    let card: Card<'_> = serde_json::from_str(input).unwrap();
    let Card::MediaKeys { media_keys } = &card else {
        panic!("expected media keys");
    };

    assert!(matches!(media_keys[0], Cow::Borrowed("plain")));
    assert!(matches!(&media_keys[1], Cow::Owned(value) if value == "escaped\nkey"));
    assert_eq!(
        serde_json::to_value(&card).unwrap(),
        serde_json::from_str::<Value>(input).unwrap()
    );
}

#[test]
fn optional_sequences_borrow_and_preserve_missing_null_and_empty_values() {
    for input in [
        "{}",
        r#"{"media_entities":null}"#,
        r#"{"media_entities":[]}"#,
        r#"{"media_entities":["plain","escaped\nkey"]}"#,
    ] {
        let article: Article<'_> = serde_json::from_str(input).unwrap();
        let expected: Value = serde_json::from_str(input).unwrap();

        match article.media_entities.as_deref() {
            None => assert!(expected["media_entities"].is_null()),
            Some([]) => assert_eq!(expected["media_entities"], json!([])),
            Some(values) => {
                assert!(matches!(values[0], Cow::Borrowed("plain")));
                assert!(matches!(&values[1], Cow::Owned(value) if value == "escaped\nkey"));
            }
        }
        assert_eq!(
            serde_json::to_value(&article).unwrap()["media_entities"],
            expected["media_entities"]
        );
    }

    for input in [r#"{"media_entities":"plain"}"#, r#"{"media_entities":[1]}"#] {
        assert!(serde_json::from_str::<Article<'_>>(input).is_err());
    }
}

/// Nested models still borrow when their lifetime bound comes from a different field.
#[test]
fn nested_article_fields_share_the_outer_lifetime_bound() {
    let input = r#"{
        "title":"Title",
        "entities":{
            "code":[{"code":"x", "content":"x", "language":"text"}],
            "hashtags":[{"start":0, "end":4, "text":"tag"}],
            "mentions":[{"start":0, "end":5, "username":"user"}],
            "urls":[{"text":"https://example.com"}]
        }
    }"#;
    let article: Article<'_> = serde_json::from_str(input).unwrap();

    assert!(matches!(article.title, Some(Cow::Borrowed("Title"))));
    let entities = article.entities.unwrap();
    let code = &entities.code.as_ref().unwrap()[0];
    for value in [&code.code, &code.content, &code.language] {
        assert!(matches!(value, Cow::Borrowed(_)));
    }
    assert!(matches!(
        entities.hashtags.unwrap()[0].text,
        Cow::Borrowed("tag")
    ));
    assert!(matches!(
        entities.mentions.unwrap()[0].username,
        Cow::Borrowed("user")
    ));
    assert!(matches!(
        entities.urls.unwrap()[0].text,
        Cow::Borrowed("https://example.com")
    ));
}

/// The outer Cow owns the decoded array while individual strings can borrow from the JSON.
#[test]
fn optional_cow_slices_borrow_their_strings() {
    let input = r#"{
        "type":"TimelineTrendUrtMetadata", "impressionId":"1", "impressionToken":"token",
        "position":0, "trendName":"trend", "relatedTerms":["plain","escaped\nterm"]
    }"#;
    let details: TransparentGuideDetails<'_> = serde_json::from_str(input).unwrap();
    let TransparentGuideDetails::TimelineTrendUrtMetadata { related_terms, .. } = &details else {
        panic!("expected trend metadata");
    };
    let terms = related_terms.as_ref().unwrap();

    assert!(matches!(terms, Cow::Owned(_)));
    assert!(matches!(terms[0], Cow::Borrowed("plain")));
    assert!(matches!(&terms[1], Cow::Owned(value) if value == "escaped\nterm"));
    assert_eq!(
        serde_json::to_value(&details).unwrap()["relatedTerms"],
        json!(["plain", "escaped\nterm"])
    );
}

use std::fmt::Debug;

use birdsite::model::place::Coordinates;
use birdsite::model::wxj::data::entity::Annotation;
use birdsite::model::wxj::data::place::BoundingBox;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

fn assert_round_trip<T: DeserializeOwned + Serialize + PartialEq + Debug>(json: &str) {
    let parsed: T = serde_json::from_str(json).unwrap();
    assert_eq!(serde_json::to_string(&parsed).unwrap(), json);

    let from_reader: T = serde_json::from_reader(json.as_bytes()).unwrap();
    assert_eq!(from_reader, parsed);

    let value = serde_json::to_value(&parsed).unwrap();
    assert_eq!(value.to_string(), json);
    assert_eq!(serde_json::from_value::<T>(value).unwrap(), parsed);
}

#[test]
fn coordinates_preserve_decimal_precision() {
    assert_round_trip::<Coordinates>(
        "[-122.123456789012345678901234567890,49.987654321098765432109876543210]",
    );
}

#[test]
fn bounding_boxes_preserve_decimal_precision() {
    assert_round_trip::<BoundingBox>(
        "[-122.123456789012345678901234567890,-49.987654321098765432109876543210,122.123456789012345678901234567890,49.987654321098765432109876543210]",
    );
}

#[test]
fn coordinates_accept_numbers_outside_machine_precision() {
    for json in [
        "[9007199254740993,-9007199254740993]",
        "[18446744073709551616,-9223372036854775809]",
        "[1e-400,1e+400]",
        "[-0.0,0.0000]",
    ] {
        assert_round_trip::<Coordinates>(json);
    }
}

#[test]
fn annotation_probabilities_preserve_decimal_precision() {
    for probability in ["0.123456789012345678901234567890", "1e-400", "0", "1"] {
        let json = format!(
            r#"{{"type":"Place","start":0,"end":4,"probability":{probability},"normalized_text":"place"}}"#
        );
        let annotation: Annotation<'_> = serde_json::from_str(&json).unwrap();
        assert_eq!(serde_json::to_string(&annotation).unwrap(), json);
        assert_eq!(serde_json::to_value(&annotation).unwrap().to_string(), json);
    }
}

#[test]
fn coordinates_and_bounding_boxes_reject_invalid_shapes() {
    for json in [r#"["1",2]"#, "[null,2]", "[true,2]", "[1]", "[1,2,3]"] {
        assert!(serde_json::from_str::<Coordinates>(json).is_err(), "{json}");
    }
    for json in [r#"[1,2,3,"4"]"#, "[1,2,3,null]", "[1,2,3]", "[1,2,3,4,5]"] {
        assert!(serde_json::from_str::<BoundingBox>(json).is_err(), "{json}");
    }
}

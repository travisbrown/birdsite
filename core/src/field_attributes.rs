pub mod range {
    use std::ops::Range;

    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Range<usize>, D::Error> {
        <(usize, usize)>::deserialize(deserializer).map(|(start, end)| start..end)
    }

    pub fn serialize<S: Serializer>(
        value: &Range<usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        (value.start, value.end).serialize(serializer)
    }

    pub mod optional {
        use std::ops::Range;

        use serde::de::{Deserialize, Deserializer};
        use serde::ser::{Serialize, Serializer};

        pub fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Option<Range<usize>>, D::Error> {
            Option::<(usize, usize)>::deserialize(deserializer)
                .map(|value| value.map(|(start, end)| start..end))
        }

        #[allow(clippy::ref_option, reason = "Serde passes fields by reference.")]
        pub fn serialize<S: Serializer>(
            value: &Option<Range<usize>>,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            value
                .as_ref()
                .map(|value| (value.start, value.end))
                .serialize(serializer)
        }
    }
}

pub mod i64_str {
    use archivindex_serde::BorrowableStr;
    use serde::de::{Deserialize, Deserializer, Error};
    use serde::ser::Serializer;

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
        BorrowableStr::deserialize(deserializer)?
            .0
            .parse()
            .map_err(D::Error::custom)
    }

    #[allow(
        clippy::trivially_copy_pass_by_ref,
        reason = "Serde passes fields by reference."
    )]
    pub fn serialize<S: Serializer>(value: &i64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Data {
        #[serde(with = "super::range")]
        range: Range<usize>,
        #[serde(default, with = "super::range::optional")]
        optional: Option<Range<usize>>,
    }

    #[test]
    fn ranges_preserve_array_endpoints() {
        for (start, end) in [(0, 0), (3, 9), (9, 3), (0, usize::MAX)] {
            let json = format!(r#"{{"range":[{start},{end}],"optional":[{start},{end}]}}"#);
            let data: Data = serde_json::from_str(&json).unwrap();
            assert_eq!(data.range, start..end);
            assert_eq!(data.optional, Some(start..end));
            assert_eq!(serde_json::to_string(&data).unwrap(), json);
        }
    }

    #[test]
    fn missing_and_null_optional_ranges_are_absent() {
        for json in [r#"{"range":[0,1]}"#, r#"{"range":[0,1],"optional":null}"#] {
            let data: Data = serde_json::from_str(json).unwrap();
            assert_eq!(data.optional, None);
            assert_eq!(
                serde_json::to_string(&data).unwrap(),
                r#"{"range":[0,1],"optional":null}"#
            );
        }
    }

    #[test]
    fn ranges_reject_invalid_arrays() {
        for value in [
            "[]",
            "[1]",
            "[1,2,3]",
            "[-1,2]",
            "[0,1.5]",
            r#"[0,"1"]"#,
            r#"{"start":0,"end":1}"#,
        ] {
            for json in [
                format!(r#"{{"range":{value}}}"#),
                format!(r#"{{"range":[0,1],"optional":{value}}}"#),
            ] {
                assert!(serde_json::from_str::<Data>(&json).is_err(), "{json}");
            }
        }
        assert!(serde_json::from_str::<Data>(r#"{"range":null}"#).is_err());
    }

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Signed(#[serde(with = "super::i64_str")] i64);

    #[test]
    fn signed_strings_preserve_integer_limits() {
        for value in [i64::MIN, -8_530_644_708_937_920_429, 0, i64::MAX] {
            let json = format!(r#""{value}""#);
            let parsed: Signed = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, Signed(value));
            assert_eq!(serde_json::to_string(&parsed).unwrap(), json);
            assert_eq!(
                serde_json::from_reader::<_, Signed>(json.as_bytes()).unwrap(),
                parsed
            );
            assert_eq!(
                serde_json::from_value::<Signed>(serde_json::json!(value.to_string())).unwrap(),
                parsed
            );
        }
        assert_eq!(
            serde_json::from_str::<Signed>(r#""\u002d1""#).unwrap(),
            Signed(-1)
        );
    }

    #[test]
    fn signed_strings_reject_invalid_or_unquoted_values() {
        for json in [
            "1",
            "-1",
            "null",
            "true",
            r#""""#,
            r#""1.5""#,
            r#""9223372036854775808""#,
            r#""-9223372036854775809""#,
        ] {
            assert!(serde_json::from_str::<Signed>(json).is_err(), "{json}");
        }
    }
}

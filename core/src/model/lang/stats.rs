//! Statistics about language frequency.
//!
//! The counts here are for distinct accounts with languages indicated in the 2025 Twitter leak.
//!
//! The numbers here should not be interpreted as exact distributions for accounts or tweets for
//! any particular time period, but are rather provided as useful general approximations.
use super::Language;
use std::{collections::HashMap, sync::LazyLock};

pub const TOTAL_COUNT: usize = 935214212;

pub static RANKED_LANGUAGE_VALUES: LazyLock<Vec<Language>> = LazyLock::new(|| {
    // If the code doesn't parse, that's a development error.
    RAW_DATA
        .iter()
        .map(|(code, _)| code.parse().unwrap())
        .collect()
});

pub fn count(value: &Language) -> usize {
    // If the map doesn't contain this value, that's a development error.
    *LANGUAGE_VALUE_COUNTS.get(value).unwrap()
}

pub fn percentage(value: &Language) -> f32 {
    // If the map doesn't contain this value, that's a development error.
    count(value) as f32 / TOTAL_COUNT as f32
}

const RAW_DATA: &[(&str, usize)] = &[
    ("en", 474273330),
    ("es", 137775205),
    ("pt", 54386393),
    ("ar", 44822808),
    ("ja", 39213992),
    ("tr", 31689123),
    ("fr", 30270222),
    ("id", 27367798),
    ("ru", 20316846),
    ("it", 11658601),
    ("de", 10961326),
    ("en-gb", 8805812),
    ("ko", 7118634),
    ("th", 6292044),
    ("nl", 4962904),
    ("pl", 3028770),
    ("zh-cn", 2726809),
    ("en-GB", 2538734),
    ("sv", 1593196),
    ("vi", 1550118),
    ("zh-tw", 1412475),
    ("el", 1122933),
    ("zh-CN", 1098407),
    ("hu", 970922),
    ("ro", 885603),
    ("cs", 859737),
    ("fil", 838359),
    ("da", 789708),
    ("fa", 660127),
    ("fi", 626514),
    ("he", 471511),
    ("no", 462262),
    ("uk", 416835),
    ("zh-TW", 415005),
    ("ca", 342030),
    ("zh-Hans", 250100),
    ("hi", 234978),
    ("msa", 229920),
    ("sr", 201292),
    ("hr", 198755),
    ("es-MX", 193251),
    ("bg", 151550),
    ("in", 137226),
    ("sk", 108813),
    ("en-AU", 95997),
    ("bn", 44049),
    ("lv", 42102),
    ("ur", 23177),
    ("en-IN", 18491),
    ("gl", 16488),
    ("ta", 16367),
    ("eu", 16194),
    ("lt", 12086),
    ("mr", 12009),
    ("zh", 11294),
    ("az", 11125),
    ("sl", 10987),
    ("gu", 7757),
    ("et", 5542),
    ("kn", 3329),
    ("km", 2874),
    ("my", 2262),
    ("ka", 1931),
    ("is", 1887),
    ("af", 1515),
    ("ga", 1451),
    ("ckb", 1441),
    ("cy", 1016),
    ("iw", 712),
    ("tl", 446),
    ("pa", 398),
    ("lo", 369),
    ("ml", 348),
    ("ne", 284),
    ("te", 284),
    ("hy", 240),
    ("si", 175),
    ("am", 151),
    ("ps", 96),
    ("ug", 56),
    ("or", 45),
    ("bo", 42),
    ("ht", 29),
    ("dv", 0),
    ("sd", 0),
];

static LANGUAGE_VALUE_COUNTS: LazyLock<HashMap<Language, usize>> = LazyLock::new(|| {
    // If the code doesn't parse, that's a development error.
    RAW_DATA
        .iter()
        .map(|(code, count)| (code.parse().unwrap(), *count))
        .collect()
});

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn raw_data_sorted() {
        let mut sorted = super::RAW_DATA.to_vec();

        sorted.sort_by_key(|(code, count)| (std::cmp::Reverse(*count), *code));

        assert_eq!(super::RAW_DATA, sorted);
    }

    #[test]
    fn ranked_language_values_complete() {
        assert_eq!(
            super::super::LANGUAGE_VALUES
                .iter()
                .collect::<HashSet<_>>()
                .difference(&super::RANKED_LANGUAGE_VALUES.iter().collect())
                .collect::<HashSet<_>>(),
            HashSet::new()
        )
    }
}

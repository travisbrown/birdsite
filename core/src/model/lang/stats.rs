//! Statistics about language frequency.
//!
//! The counts here are for distinct accounts with languages indicated in the 2025 Twitter leak.
//!
//! The numbers here should not be interpreted as exact distributions for accounts or tweets for
//! any particular time period, but are rather provided as useful general approximations.
use super::Language;
use std::{collections::HashMap, sync::LazyLock};

pub const TOTAL_COUNT: usize = 935_214_304;

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

#[must_use]
pub fn percentage(value: &Language) -> f32 {
    // If the map doesn't contain this value, that's a development error.
    count(value) as f32 / TOTAL_COUNT as f32
}

const RAW_DATA: &[(&str, usize)] = &[
    ("en", 474_273_330),
    ("es", 137_775_205),
    ("pt", 54_386_393),
    ("ar", 44_822_808),
    ("ja", 39_213_992),
    ("tr", 31_689_123),
    ("fr", 30_270_222),
    ("id", 27_367_798),
    ("ru", 20_316_846),
    ("it", 11_658_601),
    ("de", 10_961_326),
    ("en-gb", 8_805_812),
    ("ko", 7_118_634),
    ("th", 6_292_044),
    ("nl", 4_962_904),
    ("pl", 3_028_770),
    ("zh-cn", 2_726_809),
    ("en-GB", 2_538_734),
    ("sv", 1_593_196),
    ("vi", 1_550_118),
    ("zh-tw", 1_412_475),
    ("el", 1_122_933),
    ("zh-CN", 1_098_407),
    ("hu", 970_922),
    ("ro", 885_603),
    ("cs", 859_737),
    ("fil", 838_359),
    ("da", 789_708),
    ("fa", 660_127),
    ("fi", 626_514),
    ("he", 471_511),
    ("no", 462_262),
    ("uk", 416_835),
    ("zh-TW", 415_005),
    ("ca", 342_030),
    ("zh-Hans", 250_100),
    ("hi", 234_978),
    ("msa", 229_920),
    ("sr", 201_292),
    ("hr", 198_755),
    ("es-MX", 193_251),
    ("bg", 151_550),
    ("in", 137_226),
    ("ms", 127_522),
    ("sk", 108_813),
    ("nb", 98_503),
    ("en-AU", 95_997),
    ("bn", 44_049),
    ("lv", 42_102),
    ("ur", 23_177),
    ("en-IN", 18_491),
    ("gl", 16_488),
    ("ta", 16_367),
    ("eu", 16_194),
    ("lt", 12_086),
    ("mr", 12_009),
    ("zh", 11_294),
    ("az", 11_125),
    ("sl", 10_987),
    ("gu", 7757),
    ("sq", 7074),
    ("mk", 6847),
    ("et", 5542),
    ("bs", 3419),
    ("kn", 3329),
    ("km", 2874),
    ("my", 2262),
    ("ka", 1931),
    ("is", 1887),
    ("af", 1515),
    ("ga", 1451),
    ("ckb", 1441),
    ("cy", 1016),
    ("mn", 911),
    ("uz", 864),
    ("iw", 712),
    ("tl", 446),
    ("pa", 398),
    ("sw", 371),
    ("lo", 369),
    ("ml", 348),
    ("kk", 344),
    ("ne", 284),
    ("te", 284),
    ("ku", 276),
    ("hy", 240),
    ("si", 175),
    ("am", 151),
    ("su", 117),
    ("ps", 96),
    ("be", 69),
    ("ug", 56),
    ("haw", 55),
    ("so", 46),
    ("or", 45),
    ("bo", 42),
    ("la", 39),
    ("ht", 29),
    ("tg", 23),
    ("mi", 20),
    ("co", 19),
    ("ky", 17),
    ("xh", 17),
    ("lb", 16),
    ("tk", 16),
    ("eo", 14),
    ("zu", 14),
    ("gd", 8),
    ("sn", 8),
    ("ig", 6),
    ("mt", 6),
    ("yo", 5),
    ("mg", 3),
    ("yi", 3),
    ("ha", 1),
    ("ny", 1),
    ("rw", 1),
    ("ceb", 0),
    ("dv", 0),
    ("fil-ph", 0),
    ("fr-CA", 0),
    ("fy", 0),
    ("hmn", 0),
    ("jw", 0),
    ("sd", 0),
    ("sm", 0),
    ("sr-cyrl", 0),
    ("sr-latn", 0),
    ("st", 0),
    ("tt", 0),
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

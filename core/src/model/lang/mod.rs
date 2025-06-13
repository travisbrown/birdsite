use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

pub mod stats;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid lang")]
    Invalid(String),
    #[error("Invalid language code")]
    InvalidLanguage(String),
    #[error("Invalid special language code")]
    InvalidSpecial(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Lang {
    Language(Language),
    Special(Special),
}

pub static LANG_CODES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let mut codes = Vec::with_capacity(LANGUAGE_CODES.len() + SPECIAL_CODES.len());
    codes.extend(LANGUAGE_CODES);
    codes.extend(SPECIAL_CODES);

    codes.sort();

    codes
});

pub static LANG_VALUES: LazyLock<Vec<Lang>> = LazyLock::new(|| {
    let mut values = Vec::with_capacity(LANGUAGE_VALUES.len() + SPECIAL_VALUES.len());

    for language in LANGUAGE_VALUES {
        values.push(Lang::Language(language));
    }

    for special in SPECIAL_VALUES {
        values.push(Lang::Special(special));
    }

    values.sort_by_key(|value| value.as_str());

    values
});

impl Lang {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Language(language) => language.as_str(),
            Self::Special(special) => special.as_str(),
        }
    }
}

impl PartialOrd for Lang {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lang {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl FromStr for Lang {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Language::parse_str(s)
            .map(Self::Language)
            .or_else(|| Special::parse_str(s).map(Self::Special))
            .ok_or_else(|| Error::Invalid(s.to_string()))
    }
}

impl From<&Lang> for &'static str {
    fn from(value: &Lang) -> Self {
        value.as_str()
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for Lang {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = serde::de::Deserialize::deserialize(deserializer)?;

        Language::parse_str(code)
            .map(Self::Language)
            .or_else(|| Special::parse_str(code).map(Self::Special))
            .ok_or_else(|| serde::de::Error::unknown_variant(code, &LANGUAGE_CODES))
    }
}

impl serde::Serialize for Lang {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

const LANGUAGE_CODES: [&str; 129] = [
    "af", "am", "ar", "az", "be", "bg", "bn", "bo", "bs", "ca", "ceb", "ckb", "co", "cs", "cy",
    "da", "de", "dv", "el", "en", "en-AU", "en-GB", "en-IN", "en-gb", "eo", "es", "es-MX", "et",
    "eu", "fa", "fi", "fil", "fil-ph", "fr", "fy", "ga", "gd", "gl", "gu", "ha", "haw", "he", "hi",
    "hmn", "hr", "ht", "hu", "hy", "id", "ig", "in", "is", "it", "iw", "ja", "jw", "ka", "kk",
    "km", "kn", "ko", "ku", "ky", "la", "lb", "lo", "lt", "lv", "mg", "mi", "mk", "ml", "mn", "mr",
    "ms", "msa", "mt", "my", "nb", "ne", "nl", "no", "ny", "or", "pa", "pl", "ps", "pt", "ro",
    "ru", "rw", "sd", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr", "sr-cyrl", "sr-latn", "st",
    "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr", "tt", "ug", "uk", "ur", "uz", "vi",
    "xh", "yi", "yo", "zh", "zh-CN", "zh-Hans", "zh-TW", "zh-cn", "zh-tw", "zu",
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    Afrikaans,
    Albanian,
    Amharic,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bulgarian,
    Bengali,
    Bosnian,
    Burmese,
    Catalan,
    Cebuano,
    CentralKhmer,
    CentralKurdish,
    Chichewa,
    Chinese(Option<ChineseLocale>),
    Corsican,
    Croatian,
    Czech,
    Danish,
    Divehi,
    Dutch,
    English(Option<EnglishLocale>),
    Esperanto,
    Estonian,
    Filipino(Option<FilipinoLocale>),
    Finnish,
    French,
    Gaelic,
    Galician,
    Georgian,
    German,
    Greek,
    Gujarati,
    Haitian,
    Hausa,
    Hawaiian,
    Hebrew { iso_639_1988: bool },
    Hindi,
    Hmong,
    Hungarian,
    Icelandic,
    Igbo,
    Indonesian { iso_639_1988: bool },
    Irish,
    Italian,
    Japanese,
    Javanese,
    Kannada,
    Kazakh,
    Kinyarwanda,
    Korean,
    Kurdish,
    Kyrgyz,
    Lao,
    Latin,
    Latvian,
    Lithuanian,
    Luxembourgish,
    Macedonian,
    Malagasy,
    Malay { iso_639_2: bool },
    Malayam,
    Maltese,
    Maori,
    Marathi,
    Mongolian,
    Nepali,
    Norwegian,
    NorwegianBokmal,
    Oriya,
    Panjabi,
    Pashto,
    Persian,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Samoan,
    Serbian(Option<SerbianScript>),
    Shona,
    Sindhi,
    Sinhalese,
    Slovak,
    Slovenian,
    Somali,
    SouthernSotho,
    Spanish(Option<SpanishLocale>),
    Sundanese,
    Swahili,
    Swedish,
    Tagalog,
    Tajik,
    Tamil,
    Tatar,
    Telugu,
    Thai,
    Tibetan,
    Turkish,
    Turkmen,
    Uighur,
    Ukrainian,
    Urdu,
    Uzbek,
    Vietnamese,
    WesternFrisian,
    Xhosa,
    Yiddish,
    Yoruba,
    Welsh,
    Zulu,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChineseLocale {
    Han,
    Simplified { capitalized: bool },
    TaiwaneseMandarin { capitalized: bool },
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EnglishLocale {
    Australia,
    GreatBritain { capitalized: bool },
    India,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FilipinoLocale {
    Philippines,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SerbianScript {
    Cyrillic,
    Latin,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SpanishLocale {
    Mexico,
}

impl Language {
    fn parse_str(input: &str) -> Option<Self> {
        match input {
            "af" => Some(Self::Afrikaans),
            "am" => Some(Self::Amharic),
            "ar" => Some(Self::Arabic),
            "az" => Some(Self::Azerbaijani),
            "be" => Some(Self::Belarusian),
            "bg" => Some(Self::Bulgarian),
            "bn" => Some(Self::Bengali),
            "bo" => Some(Self::Tibetan),
            "bs" => Some(Self::Bosnian),
            "ca" => Some(Self::Catalan),
            "ceb" => Some(Self::Cebuano),
            "ckb" => Some(Self::CentralKurdish),
            "co" => Some(Self::Corsican),
            "cs" => Some(Self::Czech),
            "cy" => Some(Self::Welsh),
            "da" => Some(Self::Danish),
            "de" => Some(Self::German),
            "dv" => Some(Self::Divehi),
            "el" => Some(Self::Greek),
            "en" => Some(Self::English(None)),
            "en-AU" => Some(Self::English(Some(EnglishLocale::Australia))),
            "en-GB" => Some(Self::English(Some(EnglishLocale::GreatBritain {
                capitalized: true,
            }))),
            "en-IN" => Some(Self::English(Some(EnglishLocale::India))),
            "en-gb" => Some(Self::English(Some(EnglishLocale::GreatBritain {
                capitalized: false,
            }))),
            "eo" => Some(Self::Esperanto),
            "es" => Some(Self::Spanish(None)),
            "es-MX" => Some(Self::Spanish(Some(SpanishLocale::Mexico))),
            "et" => Some(Self::Estonian),
            "eu" => Some(Self::Basque),
            "fa" => Some(Self::Persian),
            "fi" => Some(Self::Finnish),
            "fil" => Some(Self::Filipino(None)),
            "fil-ph" => Some(Self::Filipino(Some(FilipinoLocale::Philippines))),
            "fr" => Some(Self::French),
            "fy" => Some(Self::WesternFrisian),
            "ga" => Some(Self::Irish),
            "gd" => Some(Self::Gaelic),
            "gl" => Some(Self::Galician),
            "gu" => Some(Self::Gujarati),
            "ha" => Some(Self::Hausa),
            "haw" => Some(Self::Hawaiian),
            "he" => Some(Self::Hebrew { iso_639_1988: true }),
            "hi" => Some(Self::Hindi),
            "hmn" => Some(Self::Hmong),
            "hr" => Some(Self::Croatian),
            "ht" => Some(Self::Haitian),
            "hu" => Some(Self::Hungarian),
            "hy" => Some(Self::Armenian),
            "id" => Some(Self::Indonesian {
                iso_639_1988: false,
            }),
            "ig" => Some(Self::Igbo),
            "in" => Some(Self::Indonesian { iso_639_1988: true }),
            "is" => Some(Self::Icelandic),
            "it" => Some(Self::Italian),
            "iw" => Some(Self::Hebrew {
                iso_639_1988: false,
            }),
            "ja" => Some(Self::Japanese),
            "jw" => Some(Self::Javanese),
            "ka" => Some(Self::Georgian),
            "kk" => Some(Self::Kazakh),
            "km" => Some(Self::CentralKhmer),
            "kn" => Some(Self::Kannada),
            "ko" => Some(Self::Korean),
            "ku" => Some(Self::Kurdish),
            "ky" => Some(Self::Kyrgyz),
            "lo" => Some(Self::Lao),
            "la" => Some(Self::Latin),
            "lb" => Some(Self::Luxembourgish),
            "lv" => Some(Self::Latvian),
            "lt" => Some(Self::Lithuanian),
            "ml" => Some(Self::Malayam),
            "mg" => Some(Self::Malagasy),
            "mn" => Some(Self::Mongolian),
            "mi" => Some(Self::Maori),
            "mk" => Some(Self::Macedonian),
            "mr" => Some(Self::Marathi),
            "ms" => Some(Self::Malay { iso_639_2: false }),
            "msa" => Some(Self::Malay { iso_639_2: true }),
            "mt" => Some(Self::Maltese),
            "my" => Some(Self::Burmese),
            "nb" => Some(Self::NorwegianBokmal),
            "ne" => Some(Self::Nepali),
            "nl" => Some(Self::Dutch),
            "no" => Some(Self::Norwegian),
            "ny" => Some(Self::Chichewa),
            "or" => Some(Self::Oriya),
            "pa" => Some(Self::Panjabi),
            "pl" => Some(Self::Polish),
            "ps" => Some(Self::Pashto),
            "pt" => Some(Self::Portuguese),
            "ro" => Some(Self::Romanian),
            "ru" => Some(Self::Russian),
            "rw" => Some(Self::Kinyarwanda),
            "sd" => Some(Self::Sindhi),
            "si" => Some(Self::Sinhalese),
            "sk" => Some(Self::Slovak),
            "sl" => Some(Self::Slovenian),
            "sm" => Some(Self::Samoan),
            "sn" => Some(Self::Shona),
            "so" => Some(Self::Somali),
            "sq" => Some(Self::Albanian),
            "sr" => Some(Self::Serbian(None)),
            "sr-cyrl" => Some(Self::Serbian(Some(SerbianScript::Cyrillic))),
            "sr-latn" => Some(Self::Serbian(Some(SerbianScript::Latin))),
            "st" => Some(Self::SouthernSotho),
            "su" => Some(Self::Sundanese),
            "sv" => Some(Self::Swedish),
            "sw" => Some(Self::Swahili),
            "ta" => Some(Self::Tamil),
            "te" => Some(Self::Telugu),
            "tg" => Some(Self::Tajik),
            "th" => Some(Self::Thai),
            "tk" => Some(Self::Turkmen),
            "tl" => Some(Self::Tagalog),
            "tr" => Some(Self::Turkish),
            "tt" => Some(Self::Tatar),
            "ug" => Some(Self::Uighur),
            "uk" => Some(Self::Ukrainian),
            "ur" => Some(Self::Urdu),
            "uz" => Some(Self::Uzbek),
            "vi" => Some(Self::Vietnamese),
            "xh" => Some(Self::Xhosa),
            "yi" => Some(Self::Yiddish),
            "yo" => Some(Self::Yoruba),
            "zh" => Some(Self::Chinese(None)),
            "zh-CN" => Some(Self::Chinese(Some(ChineseLocale::Simplified {
                capitalized: true,
            }))),
            "zh-Hans" => Some(Self::Chinese(Some(ChineseLocale::Han))),
            "zh-TW" => Some(Self::Chinese(Some(ChineseLocale::TaiwaneseMandarin {
                capitalized: true,
            }))),
            "zh-cn" => Some(Self::Chinese(Some(ChineseLocale::Simplified {
                capitalized: false,
            }))),
            "zh-tw" => Some(Self::Chinese(Some(ChineseLocale::TaiwaneseMandarin {
                capitalized: false,
            }))),
            "zu" => Some(Self::Zulu),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Afrikaans => "af",
            Self::Albanian => "sq",
            Self::Amharic => "am",
            Self::Arabic => "ar",
            Self::Armenian => "hy",
            Self::Azerbaijani => "az",
            Self::Basque => "eu",
            Self::Belarusian => "be",
            Self::Bengali => "bn",
            Self::Bosnian => "bs",
            Self::Bulgarian => "bg",
            Self::Burmese => "my",
            Self::Catalan => "ca",
            Self::Cebuano => "ceb",
            Self::CentralKhmer => "km",
            Self::CentralKurdish => "ckb",
            Self::Chichewa => "ny",
            Self::Chinese(None) => "zh",
            Self::Chinese(Some(ChineseLocale::Han)) => "zh-Hans",
            Self::Chinese(Some(ChineseLocale::Simplified { capitalized: false })) => "zh-cn",
            Self::Chinese(Some(ChineseLocale::Simplified { capitalized: true })) => "zh-CN",
            Self::Chinese(Some(ChineseLocale::TaiwaneseMandarin { capitalized: false })) => "zh-tw",
            Self::Chinese(Some(ChineseLocale::TaiwaneseMandarin { capitalized: true })) => "zh-TW",
            Self::Corsican => "co",
            Self::Croatian => "hr",
            Self::Czech => "cs",
            Self::Danish => "da",
            Self::Divehi => "dv",
            Self::Dutch => "nl",
            Self::English(None) => "en",
            Self::English(Some(EnglishLocale::Australia)) => "en-AU",
            Self::English(Some(EnglishLocale::GreatBritain { capitalized: false })) => "en-gb",
            Self::English(Some(EnglishLocale::GreatBritain { capitalized: true })) => "en-GB",
            Self::English(Some(EnglishLocale::India)) => "en-IN",
            Self::Esperanto => "eo",
            Self::Estonian => "et",
            Self::Filipino(None) => "fil",
            Self::Filipino(Some(FilipinoLocale::Philippines)) => "fil-ph",
            Self::Finnish => "fi",
            Self::French => "fr",
            Self::Gaelic => "gd",
            Self::Galician => "gl",
            Self::Georgian => "ka",
            Self::German => "de",
            Self::Greek => "el",
            Self::Gujarati => "gu",
            Self::Haitian => "ht",
            Self::Hausa => "ha",
            Self::Hawaiian => "haw",
            Self::Hebrew {
                iso_639_1988: false,
            } => "iw",
            Self::Hebrew { iso_639_1988: true } => "he",
            Self::Hindi => "hi",
            Self::Hmong => "hmn",
            Self::Hungarian => "hu",
            Self::Icelandic => "is",
            Self::Igbo => "ig",
            Self::Indonesian {
                iso_639_1988: false,
            } => "id",
            Self::Indonesian { iso_639_1988: true } => "in",
            Self::Irish => "ga",
            Self::Italian => "it",
            Self::Japanese => "ja",
            Self::Javanese => "jw",
            Self::Kannada => "kn",
            Self::Kazakh => "kk",
            Self::Korean => "ko",
            Self::Kurdish => "ku",
            Self::Kyrgyz => "ky",
            Self::Lao => "lo",
            Self::Latin => "la",
            Self::Latvian => "lv",
            Self::Lithuanian => "lt",
            Self::Luxembourgish => "lb",
            Self::Macedonian => "mk",
            Self::Malagasy => "mg",
            Self::Malay { iso_639_2: false } => "ms",
            Self::Malay { iso_639_2: true } => "msa",
            Self::Malayam => "ml",
            Self::Maltese => "mt",
            Self::Maori => "mi",
            Self::Marathi => "mr",
            Self::Mongolian => "mn",
            Self::NorwegianBokmal => "nb",
            Self::Nepali => "ne",
            Self::Norwegian => "no",
            Self::Oriya => "or",
            Self::Panjabi => "pa",
            Self::Pashto => "ps",
            Self::Persian => "fa",
            Self::Polish => "pl",
            Self::Portuguese => "pt",
            Self::Romanian => "ro",
            Self::Russian => "ru",
            Self::Kinyarwanda => "rw",
            Self::Samoan => "sm",
            Self::Shona => "sn",
            Self::Sindhi => "sd",
            Self::Sinhalese => "si",
            Self::Slovak => "sk",
            Self::Slovenian => "sl",
            Self::Somali => "so",
            Self::Serbian(None) => "sr",
            Self::Serbian(Some(SerbianScript::Cyrillic)) => "sr-cyrl",
            Self::Serbian(Some(SerbianScript::Latin)) => "sr-latn",
            Self::SouthernSotho => "st",
            Self::Spanish(None) => "es",
            Self::Spanish(Some(SpanishLocale::Mexico)) => "es-MX",
            Self::Sundanese => "su",
            Self::Swahili => "sw",
            Self::Swedish => "sv",
            Self::Tagalog => "tl",
            Self::Tajik => "tg",
            Self::Tamil => "ta",
            Self::Tatar => "tt",
            Self::Telugu => "te",
            Self::Thai => "th",
            Self::Tibetan => "bo",
            Self::Turkish => "tr",
            Self::Turkmen => "tk",
            Self::Uighur => "ug",
            Self::Ukrainian => "uk",
            Self::Urdu => "ur",
            Self::Uzbek => "uz",
            Self::Vietnamese => "vi",
            Self::WesternFrisian => "fy",
            Self::Xhosa => "xh",
            Self::Yiddish => "yi",
            Self::Yoruba => "yo",
            Self::Welsh => "cy",
            Self::Zulu => "zu",
        }
    }
}

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Language {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl FromStr for Language {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Language::parse_str(s).ok_or_else(|| Error::InvalidLanguage(s.to_string()))
    }
}

impl From<&Language> for &'static str {
    fn from(value: &Language) -> Self {
        value.as_str()
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for Language {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = serde::de::Deserialize::deserialize(deserializer)?;

        Self::parse_str(code)
            .ok_or_else(|| serde::de::Error::unknown_variant(code, &LANGUAGE_CODES))
    }
}

impl serde::Serialize for Language {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.into())
    }
}

const SPECIAL_CODES: [&str; 9] = [
    "art", "qam", "qct", "qht", "qme", "qst", "und", "xx-lc", "zxx",
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Special {
    Art,
    Cashtags,
    Hashtags,
    Lolcatz,
    MediaLinks,
    Mentions,
    NoText,
    ShortText,
    Undefined,
}

impl Special {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Art => "art",
            Self::Cashtags => "qct",
            Self::Hashtags => "qht",
            Self::Lolcatz => "xx-lc",
            Self::MediaLinks => "qme",
            Self::Mentions => "qam",
            Self::NoText => "zxx",
            Self::ShortText => "qst",
            Self::Undefined => "und",
        }
    }

    fn parse_str(input: &str) -> Option<Self> {
        match input {
            "art" => Some(Self::Art),
            "qam" => Some(Self::Mentions),
            "qct" => Some(Self::Cashtags),
            "qht" => Some(Self::Hashtags),
            "qme" => Some(Self::MediaLinks),
            "qst" => Some(Self::ShortText),
            "und" => Some(Self::Undefined),
            "xx-lc" => Some(Self::Lolcatz),
            "zxx" => Some(Self::NoText),
            _ => None,
        }
    }
}

impl PartialOrd for Special {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Special {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl FromStr for Special {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Special::parse_str(s).ok_or_else(|| Error::InvalidSpecial(s.to_string()))
    }
}

impl From<&Special> for &'static str {
    fn from(value: &Special) -> Self {
        value.as_str()
    }
}

impl Display for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Special as Into<&'static str>>::into(self))
    }
}

impl<'de> serde::de::Deserialize<'de> for Special {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = serde::de::Deserialize::deserialize(deserializer)?;

        Self::parse_str(code).ok_or_else(|| serde::de::Error::unknown_variant(code, &SPECIAL_CODES))
    }
}

impl serde::Serialize for Special {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.into())
    }
}

const LANGUAGE_VALUES: [Language; 129] = [
    Language::Afrikaans,
    Language::Amharic,
    Language::Arabic,
    Language::Azerbaijani,
    Language::Belarusian,
    Language::Bulgarian,
    Language::Bengali,
    Language::Tibetan,
    Language::Bosnian,
    Language::Catalan,
    Language::Cebuano,
    Language::CentralKurdish,
    Language::Corsican,
    Language::Czech,
    Language::Welsh,
    Language::Danish,
    Language::German,
    Language::Divehi,
    Language::Greek,
    Language::English(None),
    Language::English(Some(EnglishLocale::Australia)),
    Language::English(Some(EnglishLocale::GreatBritain { capitalized: true })),
    Language::English(Some(EnglishLocale::India)),
    Language::English(Some(EnglishLocale::GreatBritain { capitalized: false })),
    Language::Esperanto,
    Language::Spanish(None),
    Language::Spanish(Some(SpanishLocale::Mexico)),
    Language::Estonian,
    Language::Basque,
    Language::Persian,
    Language::Finnish,
    Language::Filipino(None),
    Language::Filipino(Some(FilipinoLocale::Philippines)),
    Language::French,
    Language::WesternFrisian,
    Language::Irish,
    Language::Gaelic,
    Language::Galician,
    Language::Gujarati,
    Language::Hausa,
    Language::Hawaiian,
    Language::Hebrew { iso_639_1988: true },
    Language::Hindi,
    Language::Hmong,
    Language::Croatian,
    Language::Haitian,
    Language::Hungarian,
    Language::Armenian,
    Language::Indonesian {
        iso_639_1988: false,
    },
    Language::Igbo,
    Language::Indonesian { iso_639_1988: true },
    Language::Icelandic,
    Language::Italian,
    Language::Hebrew {
        iso_639_1988: false,
    },
    Language::Japanese,
    Language::Javanese,
    Language::Georgian,
    Language::Kazakh,
    Language::CentralKhmer,
    Language::Kannada,
    Language::Korean,
    Language::Kurdish,
    Language::Kyrgyz,
    Language::Latin,
    Language::Luxembourgish,
    Language::Lao,
    Language::Lithuanian,
    Language::Latvian,
    Language::Malagasy,
    Language::Maori,
    Language::Macedonian,
    Language::Malayam,
    Language::Mongolian,
    Language::Marathi,
    Language::Malay { iso_639_2: false },
    Language::Malay { iso_639_2: true },
    Language::Maltese,
    Language::Burmese,
    Language::NorwegianBokmal,
    Language::Nepali,
    Language::Dutch,
    Language::Norwegian,
    Language::Chichewa,
    Language::Oriya,
    Language::Panjabi,
    Language::Polish,
    Language::Pashto,
    Language::Portuguese,
    Language::Romanian,
    Language::Russian,
    Language::Kinyarwanda,
    Language::Sindhi,
    Language::Sinhalese,
    Language::Slovak,
    Language::Slovenian,
    Language::Samoan,
    Language::Shona,
    Language::Somali,
    Language::Albanian,
    Language::Serbian(None),
    Language::Serbian(Some(SerbianScript::Cyrillic)),
    Language::Serbian(Some(SerbianScript::Latin)),
    Language::SouthernSotho,
    Language::Sundanese,
    Language::Swedish,
    Language::Swahili,
    Language::Tamil,
    Language::Telugu,
    Language::Tajik,
    Language::Thai,
    Language::Turkmen,
    Language::Tagalog,
    Language::Turkish,
    Language::Tatar,
    Language::Uighur,
    Language::Ukrainian,
    Language::Urdu,
    Language::Uzbek,
    Language::Vietnamese,
    Language::Xhosa,
    Language::Yiddish,
    Language::Yoruba,
    Language::Chinese(None),
    Language::Chinese(Some(ChineseLocale::Simplified { capitalized: true })),
    Language::Chinese(Some(ChineseLocale::Han)),
    Language::Chinese(Some(ChineseLocale::TaiwaneseMandarin { capitalized: true })),
    Language::Chinese(Some(ChineseLocale::Simplified { capitalized: false })),
    Language::Chinese(Some(ChineseLocale::TaiwaneseMandarin {
        capitalized: false,
    })),
    Language::Zulu,
];

const SPECIAL_VALUES: [Special; 9] = [
    Special::Art,
    Special::Mentions,
    Special::Cashtags,
    Special::Hashtags,
    Special::MediaLinks,
    Special::ShortText,
    Special::Undefined,
    Special::Lolcatz,
    Special::NoText,
];

#[cfg(test)]
mod test {
    use super::{Lang, Language, Special};

    #[test]
    fn language_codes_sorted() {
        let mut codes = super::LANGUAGE_CODES.to_vec();
        codes.sort();

        assert_eq!(super::LANGUAGE_CODES, codes.as_slice());
    }

    #[test]
    fn language_values_sorted() {
        let mut values = super::LANGUAGE_VALUES.to_vec();
        values.sort();

        assert_eq!(super::LANGUAGE_VALUES, values.as_slice());
    }

    #[test]
    fn special_codes_sorted() {
        let mut codes = super::SPECIAL_CODES.to_vec();
        codes.sort();

        assert_eq!(super::SPECIAL_CODES, codes.as_slice());
    }

    #[test]
    fn special_values_sorted() {
        let mut values = super::SPECIAL_VALUES.to_vec();
        values.sort();

        assert_eq!(super::SPECIAL_VALUES, values.as_slice());
    }

    #[test]
    fn round_trip_language_codes() {
        for code in super::LANGUAGE_CODES {
            let parsed: Language = code.parse().unwrap();
            let as_string = parsed.to_string();

            assert_eq!(code, as_string);
        }
    }

    #[test]
    fn round_trip_special_codes() {
        for code in super::SPECIAL_CODES {
            let parsed: Special = code.parse().unwrap();
            let as_string = parsed.to_string();

            assert_eq!(code, as_string);
        }
    }

    #[test]
    fn round_trip_language_values() {
        for language in super::LANGUAGE_VALUES {
            let as_string = language.to_string();
            let parsed: Language = as_string.parse().unwrap();

            assert_eq!(language, parsed);
        }
    }

    #[test]
    fn round_trip_special_values() {
        for special in super::SPECIAL_VALUES {
            let as_string = special.to_string();
            let parsed: Special = as_string.parse().unwrap();

            assert_eq!(special, parsed);
        }
    }

    #[test]
    fn validate_constants() {
        assert_eq!(super::LANG_CODES.len(), super::LANG_VALUES.len());
        assert_eq!(super::LANGUAGE_CODES.len(), super::LANGUAGE_VALUES.len());
        assert_eq!(super::SPECIAL_CODES.len(), super::SPECIAL_VALUES.len());
        assert_eq!(
            super::LANG_CODES.len(),
            super::LANGUAGE_CODES.len() + super::SPECIAL_CODES.len()
        );
        assert_eq!(
            super::LANG_VALUES.len(),
            super::LANGUAGE_VALUES.len() + super::SPECIAL_VALUES.len()
        );

        let deduped_from_values = super::LANG_VALUES
            .iter()
            .map(|value| value.as_str())
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(
            deduped_from_values.len(),
            super::LANGUAGE_VALUES.len() + super::SPECIAL_VALUES.len()
        );
    }

    impl quickcheck::Arbitrary for Lang {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // Safe because we know the slice is non-empty.
            *g.choose(&super::LANG_VALUES).unwrap()
        }
    }

    impl quickcheck::Arbitrary for Language {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // Safe because we know the slice is non-empty.
            *g.choose(&super::LANGUAGE_VALUES).unwrap()
        }
    }

    impl quickcheck::Arbitrary for Special {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // Safe because we know the slice is non-empty.
            *g.choose(&super::SPECIAL_VALUES).unwrap()
        }
    }
}

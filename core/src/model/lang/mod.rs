use std::borrow::Cow;
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
    Selection(SelectionLanguage),
}

/// The language of a "select a language" UI placeholder that leaked into the
/// `lang` field (see [`Lang::Selection`]).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SelectionLanguage {
    /// The Arabic selector, shown as `"اختر اللغة..."`.
    Arabic,
    /// The Arabic selector, shown as `"اختر اللغة ..."` (trailing space before the ellipsis).
    ArabicSpaced,
    /// The Brazilian Portuguese selector, shown as `"Selecione o idioma ..."`.
    BrazilianPortuguese,
    /// The Brazilian Portuguese selector, shown as `"Selecione O Idioma ..."`.
    BrazilianPortugueseTitleCase,
    /// The Chinese selector, shown as `"选择语言..."`.
    Chinese,
    /// The Dutch selector, shown as `"Selecteer een taal..."`.
    Dutch,
    /// The English selector, shown as `"Select Language..."`.
    English,
    /// The Filipino selector, shown as `"Pumili ng Wika..."`.
    Filipino,
    /// The formal Spanish selector, shown as `"Seleccione Idioma ..."`.
    FormalSpanish,
    /// The French selector, shown as `"Sélectionnez une langue..."`.
    French,
    /// The French selector, shown as `"Choisir La Langue ..."`.
    FrenchChoisir,
    /// The German selector, shown as `"Sprache auswählen..."`.
    German,
    /// The Greek selector, shown as `"Επιλογή Γλώσσας..."`.
    Greek,
    /// The Hebrew selector, shown as `"בחירת שפה..."`.
    Hebrew,
    /// The Indonesian selector, shown as `"Pilih Bahasa..."`.
    Indonesian,
    /// The Italian selector, shown as `"Seleziona lingua..."`.
    Italian,
    /// The Japanese selector, shown as `"言語を選択..."`.
    Japanese,
    /// The Japanese selector, shown as `"言語を選択してください..."`.
    JapanesePolite,
    /// The Korean selector, shown as `"언어 선택..."`.
    Korean,
    /// The Korean selector, shown as `"언어 선택 ..."` (trailing space before the ellipsis).
    KoreanSpaced,
    /// The Norwegian selector, shown as `"Velg språk ..."`.
    Norwegian,
    /// The Persian selector, shown as `"انتخاب زبان..."`.
    Persian,
    /// The Polish selector, shown as `"Wybierz język ..."`.
    Polish,
    /// The Portuguese selector, shown as `"Selecionar Idioma..."`.
    Portuguese,
    /// The Romanian selector, shown as `"Selectează limba..."`.
    Romanian,
    /// The Russian selector, shown as `"Выберите язык..."`.
    Russian,
    /// The Serbian selector, shown as `"Одаберите језик ..."`.
    Serbian,
    /// The Spanish selector, shown as `"Selecciona un idioma..."`.
    Spanish,
    /// The Spanish selector, shown as `"Selecciona Idioma de la ..."`.
    SpanishDeLa,
    /// The Swedish selector, shown as `"Välj språk..."`.
    Swedish,
    /// The Thai selector, shown as `"เลือกภาษา..."`.
    Thai,
    /// The Traditional Chinese selector, shown as `"選擇語言..."`.
    TraditionalChinese,
    /// The Turkish selector, shown as `"Dil Seç..."`.
    Turkish,
    /// The Turkish selector, shown as `"Dil Seçin ..."`.
    TurkishSecin,
}

impl SelectionLanguage {
    /// Recognizes a localized "select a language" placeholder by its exact text.
    ///
    /// These leak into the `lang` field as values like `"Select Language..."`
    /// (the English UI) or `"Selecciona un idioma..."` (the Spanish UI). Add
    /// further localized variants as new match arms.
    fn parse_str(input: &str) -> Option<Self> {
        match input {
            "Selecione o idioma ..." => Some(Self::BrazilianPortuguese),
            "Selecione O Idioma ..." => Some(Self::BrazilianPortugueseTitleCase),
            "选择语言..." => Some(Self::Chinese),
            "Selecteer een taal..." => Some(Self::Dutch),
            "Select Language..." => Some(Self::English),
            "Pumili ng Wika..." => Some(Self::Filipino),
            "Seleccione Idioma ..." => Some(Self::FormalSpanish),
            "Sélectionnez une langue..." => Some(Self::French),
            "Choisir La Langue ..." => Some(Self::FrenchChoisir),
            "Sprache auswählen..." => Some(Self::German),
            "Επιλογή Γλώσσας..." => Some(Self::Greek),
            "בחירת שפה..." => Some(Self::Hebrew),
            "Pilih Bahasa..." => Some(Self::Indonesian),
            "Seleziona lingua..." => Some(Self::Italian),
            "言語を選択..." => Some(Self::Japanese),
            "言語を選択してください..." => Some(Self::JapanesePolite),
            "언어 선택..." => Some(Self::Korean),
            "언어 선택 ..." => Some(Self::KoreanSpaced),
            "Velg språk ..." => Some(Self::Norwegian),
            "انتخاب زبان..." => Some(Self::Persian),
            "Wybierz język ..." => Some(Self::Polish),
            "Selecionar Idioma..." => Some(Self::Portuguese),
            "Selectează limba..." => Some(Self::Romanian),
            "Выберите язык..." => Some(Self::Russian),
            "Одаберите језик ..." => Some(Self::Serbian),
            "Selecciona un idioma..." => Some(Self::Spanish),
            "Selecciona Idioma de la ..." => Some(Self::SpanishDeLa),
            "Välj språk..." => Some(Self::Swedish),
            "اختر اللغة..." => Some(Self::Arabic),
            "اختر اللغة ..." => Some(Self::ArabicSpaced),
            "เลือกภาษา..." => Some(Self::Thai),
            "選擇語言..." => Some(Self::TraditionalChinese),
            "Dil Seç..." => Some(Self::Turkish),
            "Dil Seçin ..." => Some(Self::TurkishSecin),
            _ => None,
        }
    }

    /// The exact placeholder string this selector is displayed as.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrazilianPortuguese => "Selecione o idioma ...",
            Self::BrazilianPortugueseTitleCase => "Selecione O Idioma ...",
            Self::Chinese => "选择语言...",
            Self::Dutch => "Selecteer een taal...",
            Self::English => "Select Language...",
            Self::Filipino => "Pumili ng Wika...",
            Self::FormalSpanish => "Seleccione Idioma ...",
            Self::French => "Sélectionnez une langue...",
            Self::FrenchChoisir => "Choisir La Langue ...",
            Self::German => "Sprache auswählen...",
            Self::Greek => "Επιλογή Γλώσσας...",
            Self::Hebrew => "בחירת שפה...",
            Self::Indonesian => "Pilih Bahasa...",
            Self::Italian => "Seleziona lingua...",
            Self::Japanese => "言語を選択...",
            Self::JapanesePolite => "言語を選択してください...",
            Self::Korean => "언어 선택...",
            Self::KoreanSpaced => "언어 선택 ...",
            Self::Norwegian => "Velg språk ...",
            Self::Persian => "انتخاب زبان...",
            Self::Polish => "Wybierz język ...",
            Self::Portuguese => "Selecionar Idioma...",
            Self::Romanian => "Selectează limba...",
            Self::Russian => "Выберите язык...",
            Self::Serbian => "Одаберите језик ...",
            Self::Spanish => "Selecciona un idioma...",
            Self::SpanishDeLa => "Selecciona Idioma de la ...",
            Self::Swedish => "Välj språk...",
            Self::Arabic => "اختر اللغة...",
            Self::ArabicSpaced => "اختر اللغة ...",
            Self::Thai => "เลือกภาษา...",
            Self::TraditionalChinese => "選擇語言...",
            Self::Turkish => "Dil Seç...",
            Self::TurkishSecin => "Dil Seçin ...",
        }
    }
}

pub static LANG_CODES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let mut codes = Vec::with_capacity(LANGUAGE_CODES.len() + SPECIAL_CODES.len());
    codes.extend(LANGUAGE_CODES);
    codes.extend(SPECIAL_CODES);

    codes.sort_unstable();

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

    values.sort_by_key(Lang::as_str);

    values
});

impl Lang {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Language(language) => language.as_str(),
            Self::Special(special) => special.as_str(),
            Self::Selection(selection) => selection.as_str(),
        }
    }

    /// Parses a `lang` field value, accepting language codes, special codes, and
    /// the localized "select a language" UI placeholders (see [`Lang::Selection`]).
    fn parse_str(code: &str) -> Option<Self> {
        Language::parse_str(code)
            .map(Self::Language)
            .or_else(|| Special::parse_str(code).map(Self::Special))
            .or_else(|| SelectionLanguage::parse_str(code).map(Self::Selection))
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
        Self::parse_str(s).ok_or_else(|| Error::Invalid(s.to_string()))
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
        // `Cow` borrows the input when possible but falls back to an owned string
        // when serde_json must unescape it (e.g. a `\uXXXX`-encoded placeholder),
        // unlike `&str`, which rejects any string that cannot be borrowed.
        let code: Cow<'de, str> = serde::de::Deserialize::deserialize(deserializer)?;

        Self::parse_str(&code)
            .ok_or_else(|| serde::de::Error::unknown_variant(&code, &LANGUAGE_CODES))
    }
}

impl serde::Serialize for Lang {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

const LANGUAGE_CODES: [&str; 225] = [
    "&退", ")a", "1", "<?", "EN", "Es", "TR", "ae", "af", "am", "ar", "as", "au", "az", "az-Cyrl",
    "be", "bg", "bn", "bo", "bs", "bs-Cyrl", "c", "ca", "ceb", "ch", "chr", "ckb", "co", "cr",
    "cs", "ct", "cy", "d.", "da", "de", "de-AT", "de-CH", "dk", "dv", "dz", "e^", "ee", "el", "en",
    "en-AU", "en-CA", "en-GB", "en-IN", "en-SS", "en-US", "en-gb", "en-ss", "en-us", "en-xx",
    "en_gb", "english", "eo", "es", "es-419", "es-ES", "es-MX", "es_es", "et", "eu", "fa", "fa-IR",
    "fi", "fil", "fil-ph", "fo", "fr", "fr-CA", "fr-CH", "fr-FR", "fr_fr", "fy", "ga", "gb", "gd",
    "gl", "go", "gsw", "gu", "gv", "ha", "haw", "hd", "he", "hi", "hmn", "hr", "ht", "hu", "hy",
    "id", "ig", "in", "is", "it", "it-IT", "it_it", "iu", "iw", "iw_il", "ja", "jp", "jv", "jw",
    "ka", "kk", "km", "kn", "ko", "ks-Arab", "ku", "kw", "ky", "la", "lb", "ld", "le", "lo",
    "lolc", "lt", "luy", "lv", "md", "mg", "mi", "mk", "ml", "mm", "mn", "mr", "ms", "ms-Arab",
    "msa", "mt", "my", "nap", "nb", "nd", "ne", "nl", "nl-BE", "nn", "no", "ny", "or", "pa",
    "pa-Arab", "ph", "pl", "ps", "pt", "pt-BR", "pt-PT", "qu", "ro", "ru", "rus", "rw", "sa",
    "scn", "sd", "se", "sh", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr", "sr-Latn", "sr-cyrl",
    "sr-latn", "st", "su", "sv", "sw", "ta", "te", "tg", "th", "th_th", "tk", "tl", "tlh", "to",
    "tr", "tr_tr", "ts", "tt", "ug", "uk", "ur", "us", "uz", "uz-Arab", "uz-Latn", "vi", "vn",
    "xh", "xx", "yi", "yo", "zh", "zh-CN", "zh-HK", "zh-Hans", "zh-Hant", "zh-MO", "zh-SG",
    "zh-TW", "zh-cn", "zh-tw", "zu", "zz", "ƴ", "ɐ", "‴ure", "滀", "ꁀ",
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    Afrikaans,
    Albanian,
    Avestan,
    Amharic,
    Arabic,
    Armenian,
    Assamese,
    Azerbaijani(Option<AzerbaijaniScript>),
    Basque,
    Belarusian,
    Bulgarian,
    Bengali,
    Bosnian(Option<BosnianScript>),
    Burmese,
    Catalan,
    Cebuano,
    CentralKhmer,
    CentralKurdish,
    Cherokee,
    Chichewa,
    Chinese(Option<ChineseLocale>),
    Corsican,
    Cree,
    Croatian,
    Czech,
    Cornish,
    Danish,
    Divehi,
    Dutch(Option<DutchLocale>),
    Dzongkha,
    Sanskrit,
    SwissGerman,
    English(Option<EnglishLocale>),
    Esperanto,
    Estonian,
    Ewe,
    Faroese,
    Filipino(Option<FilipinoLocale>),
    Finnish,
    French(Option<FrenchLocale>),
    Gaelic,
    Galician,
    Georgian,
    German(Option<GermanLocale>),
    Greek,
    Gujarati,
    Manx,
    Haitian,
    Hausa,
    Hawaiian,
    Hebrew(HebrewForm),
    Hindi,
    Hmong,
    Hungarian,
    Icelandic,
    Igbo,
    Indonesian { iso_639_1988: bool },
    Inuktitut,
    Irish,
    Italian(Option<ItalianLocale>),
    Japanese,
    Javanese { iso_639_1988: bool },
    Kannada,
    Kashmiri(KashmiriScript),
    Kazakh,
    Kinyarwanda,
    Klingon,
    Korean,
    Kurdish,
    Kyrgyz,
    Lao,
    Latin,
    Latvian,
    Lithuanian,
    Luxembourgish,
    Luyia,
    Macedonian,
    Malagasy,
    Malay(MalayScript),
    Malayam,
    Maltese,
    Maori,
    Marathi,
    Mongolian,
    Neapolitan,
    Nepali,
    NorthernNdebele,
    Norwegian,
    NorwegianBokmal,
    NorwegianNynorsk,
    Oriya,
    Tongan,
    Panjabi(Option<PanjabiScript>),
    Pashto,
    Persian(Option<PersianLocale>),
    Polish,
    Portuguese(Option<PortugueseLocale>),
    Quechua,
    Romanian,
    Russian { iso_639_2: bool },
    Samoan,
    Serbian(Option<SerbianScript>),
    SerboCroatian,
    Shona,
    Sicilian,
    Sindhi,
    Sinhalese,
    Slovak,
    Slovenian,
    Somali,
    SouthernSotho,
    Spanish(Option<SpanishLocale>),
    Nonstandard(NonstandardCode),
    PossibleCountryCode(PossibleCountryCode),
    Sundanese,
    Swahili,
    Swedish,
    Tagalog,
    Tajik,
    Tamil,
    Tatar,
    Telugu,
    Thai(Option<ThaiLocale>),
    Tibetan,
    Tsonga,
    Turkish(Option<TurkishLocale>),
    Turkmen,
    Uighur,
    Ukrainian,
    Urdu,
    Uzbek(Option<UzbekScript>),
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
    HongKong,
    Macau,
    Simplified { capitalized: bool },
    Singapore,
    TaiwaneseMandarin { capitalized: bool },
    Traditional,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum KashmiriScript {
    /// `"ks-Arab"` — Kashmiri in Arabic (Perso-Arabic) script.
    Arabic,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PanjabiScript {
    Arabic,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MalayScript {
    /// Latin script: `"ms"` (ISO 639-1) or `"msa"` (ISO 639-2).
    Latin { iso_639_2: bool },
    /// `"ms-Arab"` — Malay in Arabic (Jawi) script.
    Arabic,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AzerbaijaniScript {
    Cyrillic,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortugueseLocale {
    Brazil,
    Portugal,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EnglishLocale {
    Australia,
    Canada,
    GreatBritain {
        capitalized: bool,
    },
    /// `"en_gb"` — Great Britain, an underscore-separated locale form.
    GreatBritainUnderscore,
    India,
    SouthSudan {
        capitalized: bool,
    },
    Unknown,
    UnitedStates {
        capitalized: bool,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FilipinoLocale {
    Philippines,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UzbekScript {
    Arabic,
    Latin,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BosnianScript {
    Cyrillic,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SerbianScript {
    Cyrillic,
    Latin { capitalized: bool },
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FrenchLocale {
    Canada,
    Switzerland,
    /// `"fr-FR"` — France, a hyphen-separated locale form.
    FranceHyphenated,
    /// `"fr_fr"` — France, an underscore-separated locale form.
    France,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GermanLocale {
    Austria,
    Switzerland,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ThaiLocale {
    /// `"th_th"` — Thailand, an underscore-separated locale form.
    Thailand,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TurkishLocale {
    /// `"tr_tr"` — Turkey, an underscore-separated locale form.
    Turkey,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DutchLocale {
    Belgium,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HebrewForm {
    /// Bare code: `"iw"` (ISO 639:1988) or `"he"`.
    Bare { iso_639_1988: bool },
    /// `"iw_il"` — Israel, an underscore-separated locale form.
    Israel,
}

/// A language code that does not conform to any standard but appears in Twitter data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NonstandardCode {
    /// Uppercase English, found in some older Twitter API responses. See for example the tweet
    /// with identifier 1013646272505548803.
    UppercaseEn,
    /// Uppercase Turkish, the analogous uppercased form of the `"tr"` code.
    UppercaseTr,
    /// `"Es"` — a title-cased form of the `"es"` (Spanish) code.
    TitlecaseEs,
    /// Possibly an alternate form of Lolcatz ("xx-lc"). See for example the tweet with identifier
    /// 1018338922752106496.
    Lolc,
    /// Possibly a placeholder found in some Twitter data (e.g. the tweet with identifier
    /// 945125106673704960).
    Zz,
    /// `"xx"` — a placeholder for an unknown or unspecified language.
    Xx,
    /// `"‴ure"` — a garbled non-standard value observed in the data.
    PrimeUre,
    /// `"hd"` — a non-standard language code with no ISO assignment.
    Hd,
    /// `"ld"` — a non-standard language code with no ISO assignment.
    Ld,
    /// `"le"` — a non-standard language code with no ISO assignment.
    Le,
    /// `"ct"` — a non-standard language code with no ISO assignment.
    Ct,
    /// `"d."` — a garbled non-standard value (a letter and a period) observed in the data.
    DDot,
    /// `"e^"` — a garbled non-standard value (a letter and a caret) observed in the data.
    ECaret,
    /// `"滀"` (U+6EC0) — a garbled single-character non-standard value (a CJK ideograph).
    HanChar,
    /// `"<?"` — a garbled non-standard value (an angle bracket and a question mark).
    AngleQuestion,
    /// `"&退"` — a garbled non-standard value (an ampersand and a CJK ideograph).
    AmpHan,
    /// `"ƴ"` (U+01B4) — a garbled single-character non-standard value.
    YHook,
    /// `")a"` — a garbled non-standard value (a parenthesis and a letter) observed in the data.
    ParenA,
    /// `"go"` — a non-standard language code with no ISO assignment.
    Go,
    /// `"c"` — a non-standard single-character value observed in the data.
    C,
    /// `"ꁀ"` — a garbled single-character non-standard value (U+A040).
    YiQot,
    /// `"ɐ"` (U+0250) — a garbled single-character non-standard value.
    TurnedA,
    /// `"1"` — a garbled non-standard value (a bare digit) observed in the data.
    One,
    /// `"english"` — the spelled-out language name used as a non-standard code.
    EnglishWord,
}

/// An ISO 3166-1 alpha-2 country code possibly used in place of a language code by Twitter.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PossibleCountryCode {
    /// `"au"` — Australia, possibly used as an alternative to `"en"` (English).
    Au,
    /// `"ch"` — Switzerland, possibly used in place of a Swiss language code.
    Ch,
    /// `"dk"` — Denmark, possibly used as an alternative to `"da"` (Danish). See for example
    /// the tweet with identifier 1017937553016741899.
    Dk,
    /// `"gb"` — Great Britain, possibly used as an alternative to `"en"` (English).
    Gb,
    /// `"jp"` — Japan, possibly used as an alternative to `"ja"` (Japanese).
    Jp,
    /// `"md"` — Moldova, possibly used as an alternative to `"ro"` (Romanian).
    Md,
    /// `"mm"` — Myanmar, possibly used as an alternative to `"my"` (Burmese).
    Mm,
    /// `"ph"` — the Philippines, possibly used in place of a Philippine language code.
    Ph,
    /// `"se"` — Sweden, possibly used as an alternative to `"sv"` (Swedish).
    Se,
    /// `"us"` — United States, possibly used as an alternative to `"en"` (English). See for
    /// example the tweet with identifier 1014065291884662784.
    Us,
    /// `"vn"` — Vietnam, possibly used as an alternative to `"vi"` (Vietnamese).
    Vn,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ItalianLocale {
    Italy,
    /// `"it_it"` — Italy, an underscore-separated locale form.
    ItalyUnderscore,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PersianLocale {
    Iran,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SpanishLocale {
    Spain,
    /// `"es_es"` — Spain, an underscore-separated locale form.
    SpainUnderscore,
    Mexico,
    LatinAmerica,
}

impl Language {
    fn parse_str(input: &str) -> Option<Self> {
        match input {
            "&退" => Some(Self::Nonstandard(NonstandardCode::AmpHan)),
            ")a" => Some(Self::Nonstandard(NonstandardCode::ParenA)),
            "1" => Some(Self::Nonstandard(NonstandardCode::One)),
            "<?" => Some(Self::Nonstandard(NonstandardCode::AngleQuestion)),
            "EN" => Some(Self::Nonstandard(NonstandardCode::UppercaseEn)),
            "Es" => Some(Self::Nonstandard(NonstandardCode::TitlecaseEs)),
            "TR" => Some(Self::Nonstandard(NonstandardCode::UppercaseTr)),
            "lolc" => Some(Self::Nonstandard(NonstandardCode::Lolc)),
            "ae" => Some(Self::Avestan),
            "af" => Some(Self::Afrikaans),
            "am" => Some(Self::Amharic),
            "ar" => Some(Self::Arabic),
            "as" => Some(Self::Assamese),
            "au" => Some(Self::PossibleCountryCode(PossibleCountryCode::Au)),
            "az" => Some(Self::Azerbaijani(None)),
            "az-Cyrl" => Some(Self::Azerbaijani(Some(AzerbaijaniScript::Cyrillic))),
            "be" => Some(Self::Belarusian),
            "bg" => Some(Self::Bulgarian),
            "bn" => Some(Self::Bengali),
            "bo" => Some(Self::Tibetan),
            "bs" => Some(Self::Bosnian(None)),
            "bs-Cyrl" => Some(Self::Bosnian(Some(BosnianScript::Cyrillic))),
            "c" => Some(Self::Nonstandard(NonstandardCode::C)),
            "ca" => Some(Self::Catalan),
            "ceb" => Some(Self::Cebuano),
            "ch" => Some(Self::PossibleCountryCode(PossibleCountryCode::Ch)),
            "chr" => Some(Self::Cherokee),
            "ckb" => Some(Self::CentralKurdish),
            "co" => Some(Self::Corsican),
            "cr" => Some(Self::Cree),
            "cs" => Some(Self::Czech),
            "ct" => Some(Self::Nonstandard(NonstandardCode::Ct)),
            "cy" => Some(Self::Welsh),
            "d." => Some(Self::Nonstandard(NonstandardCode::DDot)),
            "da" => Some(Self::Danish),
            "de" => Some(Self::German(None)),
            "de-AT" => Some(Self::German(Some(GermanLocale::Austria))),
            "de-CH" => Some(Self::German(Some(GermanLocale::Switzerland))),
            "dk" => Some(Self::PossibleCountryCode(PossibleCountryCode::Dk)),
            "dv" => Some(Self::Divehi),
            "dz" => Some(Self::Dzongkha),
            "e^" => Some(Self::Nonstandard(NonstandardCode::ECaret)),
            "ee" => Some(Self::Ewe),
            "el" => Some(Self::Greek),
            "en" => Some(Self::English(None)),
            "en-AU" => Some(Self::English(Some(EnglishLocale::Australia))),
            "en-CA" => Some(Self::English(Some(EnglishLocale::Canada))),
            "en-GB" => Some(Self::English(Some(EnglishLocale::GreatBritain {
                capitalized: true,
            }))),
            "en-IN" => Some(Self::English(Some(EnglishLocale::India))),
            "en-SS" => Some(Self::English(Some(EnglishLocale::SouthSudan {
                capitalized: true,
            }))),
            "en-ss" => Some(Self::English(Some(EnglishLocale::SouthSudan {
                capitalized: false,
            }))),
            "en-US" => Some(Self::English(Some(EnglishLocale::UnitedStates {
                capitalized: true,
            }))),
            "en-us" => Some(Self::English(Some(EnglishLocale::UnitedStates {
                capitalized: false,
            }))),
            "en-xx" => Some(Self::English(Some(EnglishLocale::Unknown))),
            "en-gb" => Some(Self::English(Some(EnglishLocale::GreatBritain {
                capitalized: false,
            }))),
            "en_gb" => Some(Self::English(Some(EnglishLocale::GreatBritainUnderscore))),
            "english" => Some(Self::Nonstandard(NonstandardCode::EnglishWord)),
            "eo" => Some(Self::Esperanto),
            "es" => Some(Self::Spanish(None)),
            "es-419" => Some(Self::Spanish(Some(SpanishLocale::LatinAmerica))),
            "es-ES" => Some(Self::Spanish(Some(SpanishLocale::Spain))),
            "es-MX" => Some(Self::Spanish(Some(SpanishLocale::Mexico))),
            "es_es" => Some(Self::Spanish(Some(SpanishLocale::SpainUnderscore))),
            "et" => Some(Self::Estonian),
            "eu" => Some(Self::Basque),
            "fa" => Some(Self::Persian(None)),
            "fa-IR" => Some(Self::Persian(Some(PersianLocale::Iran))),
            "fi" => Some(Self::Finnish),
            "fil" => Some(Self::Filipino(None)),
            "fil-ph" => Some(Self::Filipino(Some(FilipinoLocale::Philippines))),
            "fo" => Some(Self::Faroese),
            "fr" => Some(Self::French(None)),
            "fr-CA" => Some(Self::French(Some(FrenchLocale::Canada))),
            "fr-CH" => Some(Self::French(Some(FrenchLocale::Switzerland))),
            "fr-FR" => Some(Self::French(Some(FrenchLocale::FranceHyphenated))),
            "fr_fr" => Some(Self::French(Some(FrenchLocale::France))),
            "fy" => Some(Self::WesternFrisian),
            "ga" => Some(Self::Irish),
            "gb" => Some(Self::PossibleCountryCode(PossibleCountryCode::Gb)),
            "gd" => Some(Self::Gaelic),
            "gl" => Some(Self::Galician),
            "go" => Some(Self::Nonstandard(NonstandardCode::Go)),
            "gsw" => Some(Self::SwissGerman),
            "gu" => Some(Self::Gujarati),
            "gv" => Some(Self::Manx),
            "ha" => Some(Self::Hausa),
            "haw" => Some(Self::Hawaiian),
            "hd" => Some(Self::Nonstandard(NonstandardCode::Hd)),
            "he" => Some(Self::Hebrew(HebrewForm::Bare { iso_639_1988: true })),
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
            "it" => Some(Self::Italian(None)),
            "it-IT" => Some(Self::Italian(Some(ItalianLocale::Italy))),
            "it_it" => Some(Self::Italian(Some(ItalianLocale::ItalyUnderscore))),
            "iu" => Some(Self::Inuktitut),
            "iw" => Some(Self::Hebrew(HebrewForm::Bare {
                iso_639_1988: false,
            })),
            "iw_il" => Some(Self::Hebrew(HebrewForm::Israel)),
            "ja" => Some(Self::Japanese),
            "jp" => Some(Self::PossibleCountryCode(PossibleCountryCode::Jp)),
            "jv" => Some(Self::Javanese {
                iso_639_1988: false,
            }),
            "jw" => Some(Self::Javanese { iso_639_1988: true }),
            "ka" => Some(Self::Georgian),
            "kk" => Some(Self::Kazakh),
            "km" => Some(Self::CentralKhmer),
            "kn" => Some(Self::Kannada),
            "ko" => Some(Self::Korean),
            "ks-Arab" => Some(Self::Kashmiri(KashmiriScript::Arabic)),
            "ku" => Some(Self::Kurdish),
            "kw" => Some(Self::Cornish),
            "ky" => Some(Self::Kyrgyz),
            "lo" => Some(Self::Lao),
            "la" => Some(Self::Latin),
            "lb" => Some(Self::Luxembourgish),
            "ld" => Some(Self::Nonstandard(NonstandardCode::Ld)),
            "le" => Some(Self::Nonstandard(NonstandardCode::Le)),
            "lv" => Some(Self::Latvian),
            "lt" => Some(Self::Lithuanian),
            "luy" => Some(Self::Luyia),
            "ml" => Some(Self::Malayam),
            "mm" => Some(Self::PossibleCountryCode(PossibleCountryCode::Mm)),
            "md" => Some(Self::PossibleCountryCode(PossibleCountryCode::Md)),
            "mg" => Some(Self::Malagasy),
            "mn" => Some(Self::Mongolian),
            "mi" => Some(Self::Maori),
            "mk" => Some(Self::Macedonian),
            "mr" => Some(Self::Marathi),
            "ms" => Some(Self::Malay(MalayScript::Latin { iso_639_2: false })),
            "ms-Arab" => Some(Self::Malay(MalayScript::Arabic)),
            "msa" => Some(Self::Malay(MalayScript::Latin { iso_639_2: true })),
            "mt" => Some(Self::Maltese),
            "my" => Some(Self::Burmese),
            "nap" => Some(Self::Neapolitan),
            "nb" => Some(Self::NorwegianBokmal),
            "nd" => Some(Self::NorthernNdebele),
            "ne" => Some(Self::Nepali),
            "nl" => Some(Self::Dutch(None)),
            "nl-BE" => Some(Self::Dutch(Some(DutchLocale::Belgium))),
            "nn" => Some(Self::NorwegianNynorsk),
            "no" => Some(Self::Norwegian),
            "ny" => Some(Self::Chichewa),
            "or" => Some(Self::Oriya),
            "pa" => Some(Self::Panjabi(None)),
            "pa-Arab" => Some(Self::Panjabi(Some(PanjabiScript::Arabic))),
            "ph" => Some(Self::PossibleCountryCode(PossibleCountryCode::Ph)),
            "pl" => Some(Self::Polish),
            "ps" => Some(Self::Pashto),
            "pt" => Some(Self::Portuguese(None)),
            "pt-BR" => Some(Self::Portuguese(Some(PortugueseLocale::Brazil))),
            "pt-PT" => Some(Self::Portuguese(Some(PortugueseLocale::Portugal))),
            "qu" => Some(Self::Quechua),
            "ro" => Some(Self::Romanian),
            "ru" => Some(Self::Russian { iso_639_2: false }),
            "rus" => Some(Self::Russian { iso_639_2: true }),
            "rw" => Some(Self::Kinyarwanda),
            "sa" => Some(Self::Sanskrit),
            "scn" => Some(Self::Sicilian),
            "sd" => Some(Self::Sindhi),
            "se" => Some(Self::PossibleCountryCode(PossibleCountryCode::Se)),
            "sh" => Some(Self::SerboCroatian),
            "si" => Some(Self::Sinhalese),
            "sk" => Some(Self::Slovak),
            "sl" => Some(Self::Slovenian),
            "sm" => Some(Self::Samoan),
            "sn" => Some(Self::Shona),
            "so" => Some(Self::Somali),
            "sq" => Some(Self::Albanian),
            "sr" => Some(Self::Serbian(None)),
            "sr-Latn" => Some(Self::Serbian(Some(SerbianScript::Latin {
                capitalized: true,
            }))),
            "sr-cyrl" => Some(Self::Serbian(Some(SerbianScript::Cyrillic))),
            "sr-latn" => Some(Self::Serbian(Some(SerbianScript::Latin {
                capitalized: false,
            }))),
            "st" => Some(Self::SouthernSotho),
            "su" => Some(Self::Sundanese),
            "sv" => Some(Self::Swedish),
            "sw" => Some(Self::Swahili),
            "ta" => Some(Self::Tamil),
            "te" => Some(Self::Telugu),
            "tg" => Some(Self::Tajik),
            "th" => Some(Self::Thai(None)),
            "th_th" => Some(Self::Thai(Some(ThaiLocale::Thailand))),
            "tk" => Some(Self::Turkmen),
            "tl" => Some(Self::Tagalog),
            "tlh" => Some(Self::Klingon),
            "to" => Some(Self::Tongan),
            "tr" => Some(Self::Turkish(None)),
            "tr_tr" => Some(Self::Turkish(Some(TurkishLocale::Turkey))),
            "ts" => Some(Self::Tsonga),
            "tt" => Some(Self::Tatar),
            "ug" => Some(Self::Uighur),
            "uk" => Some(Self::Ukrainian),
            "ur" => Some(Self::Urdu),
            "us" => Some(Self::PossibleCountryCode(PossibleCountryCode::Us)),
            "uz" => Some(Self::Uzbek(None)),
            "uz-Arab" => Some(Self::Uzbek(Some(UzbekScript::Arabic))),
            "uz-Latn" => Some(Self::Uzbek(Some(UzbekScript::Latin))),
            "vi" => Some(Self::Vietnamese),
            "vn" => Some(Self::PossibleCountryCode(PossibleCountryCode::Vn)),
            "xh" => Some(Self::Xhosa),
            "xx" => Some(Self::Nonstandard(NonstandardCode::Xx)),
            "yi" => Some(Self::Yiddish),
            "yo" => Some(Self::Yoruba),
            "zh" => Some(Self::Chinese(None)),
            "zh-CN" => Some(Self::Chinese(Some(ChineseLocale::Simplified {
                capitalized: true,
            }))),
            "zh-HK" => Some(Self::Chinese(Some(ChineseLocale::HongKong))),
            "zh-Hans" => Some(Self::Chinese(Some(ChineseLocale::Han))),
            "zh-Hant" => Some(Self::Chinese(Some(ChineseLocale::Traditional))),
            "zh-MO" => Some(Self::Chinese(Some(ChineseLocale::Macau))),
            "zh-SG" => Some(Self::Chinese(Some(ChineseLocale::Singapore))),
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
            "zz" => Some(Self::Nonstandard(NonstandardCode::Zz)),
            "ƴ" => Some(Self::Nonstandard(NonstandardCode::YHook)),
            "ɐ" => Some(Self::Nonstandard(NonstandardCode::TurnedA)),
            "‴ure" => Some(Self::Nonstandard(NonstandardCode::PrimeUre)),
            "滀" => Some(Self::Nonstandard(NonstandardCode::HanChar)),
            "ꁀ" => Some(Self::Nonstandard(NonstandardCode::YiQot)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Avestan => "ae",
            Self::Afrikaans => "af",
            Self::Albanian => "sq",
            Self::Amharic => "am",
            Self::Arabic => "ar",
            Self::Armenian => "hy",
            Self::Assamese => "as",
            Self::Azerbaijani(None) => "az",
            Self::Azerbaijani(Some(AzerbaijaniScript::Cyrillic)) => "az-Cyrl",
            Self::Basque => "eu",
            Self::Belarusian => "be",
            Self::Bengali => "bn",
            Self::Bosnian(None) => "bs",
            Self::Bosnian(Some(BosnianScript::Cyrillic)) => "bs-Cyrl",
            Self::Bulgarian => "bg",
            Self::Burmese => "my",
            Self::Catalan => "ca",
            Self::Cebuano => "ceb",
            Self::Cherokee => "chr",
            Self::CentralKhmer => "km",
            Self::CentralKurdish => "ckb",
            Self::Chichewa => "ny",
            Self::Chinese(None) => "zh",
            Self::Chinese(Some(ChineseLocale::Han)) => "zh-Hans",
            Self::Chinese(Some(ChineseLocale::HongKong)) => "zh-HK",
            Self::Chinese(Some(ChineseLocale::Traditional)) => "zh-Hant",
            Self::Chinese(Some(ChineseLocale::Macau)) => "zh-MO",
            Self::Chinese(Some(ChineseLocale::Singapore)) => "zh-SG",
            Self::Chinese(Some(ChineseLocale::Simplified { capitalized: false })) => "zh-cn",
            Self::Chinese(Some(ChineseLocale::Simplified { capitalized: true })) => "zh-CN",
            Self::Chinese(Some(ChineseLocale::TaiwaneseMandarin { capitalized: false })) => "zh-tw",
            Self::Chinese(Some(ChineseLocale::TaiwaneseMandarin { capitalized: true })) => "zh-TW",
            Self::Corsican => "co",
            Self::Cree => "cr",
            Self::Croatian => "hr",
            Self::Czech => "cs",
            Self::Cornish => "kw",
            Self::Danish => "da",
            Self::Divehi => "dv",
            Self::Dzongkha => "dz",
            Self::Dutch(None) => "nl",
            Self::Dutch(Some(DutchLocale::Belgium)) => "nl-BE",
            Self::Sanskrit => "sa",
            Self::SwissGerman => "gsw",
            Self::English(None) => "en",
            Self::English(Some(EnglishLocale::Australia)) => "en-AU",
            Self::English(Some(EnglishLocale::Canada)) => "en-CA",
            Self::English(Some(EnglishLocale::GreatBritain { capitalized: false })) => "en-gb",
            Self::English(Some(EnglishLocale::GreatBritain { capitalized: true })) => "en-GB",
            Self::English(Some(EnglishLocale::GreatBritainUnderscore)) => "en_gb",
            Self::English(Some(EnglishLocale::India)) => "en-IN",
            Self::English(Some(EnglishLocale::SouthSudan { capitalized: true })) => "en-SS",
            Self::English(Some(EnglishLocale::SouthSudan { capitalized: false })) => "en-ss",
            Self::English(Some(EnglishLocale::Unknown)) => "en-xx",
            Self::English(Some(EnglishLocale::UnitedStates { capitalized: true })) => "en-US",
            Self::English(Some(EnglishLocale::UnitedStates { capitalized: false })) => "en-us",
            Self::Esperanto => "eo",
            Self::Estonian => "et",
            Self::Ewe => "ee",
            Self::Filipino(None) => "fil",
            Self::Filipino(Some(FilipinoLocale::Philippines)) => "fil-ph",
            Self::Finnish => "fi",
            Self::Faroese => "fo",
            Self::French(None) => "fr",
            Self::French(Some(FrenchLocale::Canada)) => "fr-CA",
            Self::French(Some(FrenchLocale::Switzerland)) => "fr-CH",
            Self::French(Some(FrenchLocale::FranceHyphenated)) => "fr-FR",
            Self::French(Some(FrenchLocale::France)) => "fr_fr",
            Self::Gaelic => "gd",
            Self::Galician => "gl",
            Self::Georgian => "ka",
            Self::German(None) => "de",
            Self::German(Some(GermanLocale::Austria)) => "de-AT",
            Self::German(Some(GermanLocale::Switzerland)) => "de-CH",
            Self::Greek => "el",
            Self::Gujarati => "gu",
            Self::Manx => "gv",
            Self::Haitian => "ht",
            Self::Hausa => "ha",
            Self::Hawaiian => "haw",
            Self::Hebrew(HebrewForm::Bare {
                iso_639_1988: false,
            }) => "iw",
            Self::Hebrew(HebrewForm::Bare { iso_639_1988: true }) => "he",
            Self::Hebrew(HebrewForm::Israel) => "iw_il",
            Self::Hindi => "hi",
            Self::Hmong => "hmn",
            Self::Hungarian => "hu",
            Self::Icelandic => "is",
            Self::Igbo => "ig",
            Self::Indonesian {
                iso_639_1988: false,
            } => "id",
            Self::Indonesian { iso_639_1988: true } => "in",
            Self::Inuktitut => "iu",
            Self::Irish => "ga",
            Self::Italian(None) => "it",
            Self::Italian(Some(ItalianLocale::Italy)) => "it-IT",
            Self::Italian(Some(ItalianLocale::ItalyUnderscore)) => "it_it",
            Self::Japanese => "ja",
            Self::Javanese {
                iso_639_1988: false,
            } => "jv",
            Self::Javanese { iso_639_1988: true } => "jw",
            Self::Kannada => "kn",
            Self::Kashmiri(KashmiriScript::Arabic) => "ks-Arab",
            Self::Kazakh => "kk",
            Self::Klingon => "tlh",
            Self::Korean => "ko",
            Self::Kurdish => "ku",
            Self::Kyrgyz => "ky",
            Self::Lao => "lo",
            Self::Latin => "la",
            Self::Latvian => "lv",
            Self::Lithuanian => "lt",
            Self::Luyia => "luy",
            Self::Luxembourgish => "lb",
            Self::Macedonian => "mk",
            Self::Malagasy => "mg",
            Self::Malay(MalayScript::Latin { iso_639_2: false }) => "ms",
            Self::Malay(MalayScript::Latin { iso_639_2: true }) => "msa",
            Self::Malay(MalayScript::Arabic) => "ms-Arab",
            Self::Malayam => "ml",
            Self::Maltese => "mt",
            Self::Maori => "mi",
            Self::Marathi => "mr",
            Self::Mongolian => "mn",
            Self::Neapolitan => "nap",
            Self::NorwegianBokmal => "nb",
            Self::NorthernNdebele => "nd",
            Self::Nepali => "ne",
            Self::NorwegianNynorsk => "nn",
            Self::Norwegian => "no",
            Self::Oriya => "or",
            Self::Panjabi(None) => "pa",
            Self::Panjabi(Some(PanjabiScript::Arabic)) => "pa-Arab",
            Self::Pashto => "ps",
            Self::Persian(None) => "fa",
            Self::Persian(Some(PersianLocale::Iran)) => "fa-IR",
            Self::Polish => "pl",
            Self::Portuguese(None) => "pt",
            Self::Portuguese(Some(PortugueseLocale::Brazil)) => "pt-BR",
            Self::Portuguese(Some(PortugueseLocale::Portugal)) => "pt-PT",
            Self::Quechua => "qu",
            Self::Romanian => "ro",
            Self::Russian { iso_639_2: false } => "ru",
            Self::Russian { iso_639_2: true } => "rus",
            Self::Kinyarwanda => "rw",
            Self::Samoan => "sm",
            Self::Shona => "sn",
            Self::Sicilian => "scn",
            Self::Sindhi => "sd",
            Self::SerboCroatian => "sh",
            Self::Sinhalese => "si",
            Self::Slovak => "sk",
            Self::Slovenian => "sl",
            Self::Somali => "so",
            Self::Serbian(None) => "sr",
            Self::Serbian(Some(SerbianScript::Latin { capitalized: true })) => "sr-Latn",
            Self::Serbian(Some(SerbianScript::Cyrillic)) => "sr-cyrl",
            Self::Serbian(Some(SerbianScript::Latin { capitalized: false })) => "sr-latn",
            Self::SouthernSotho => "st",
            Self::Spanish(None) => "es",
            Self::Spanish(Some(SpanishLocale::Spain)) => "es-ES",
            Self::Spanish(Some(SpanishLocale::SpainUnderscore)) => "es_es",
            Self::Spanish(Some(SpanishLocale::Mexico)) => "es-MX",
            Self::Spanish(Some(SpanishLocale::LatinAmerica)) => "es-419",
            Self::Sundanese => "su",
            Self::Swahili => "sw",
            Self::Swedish => "sv",
            Self::Tagalog => "tl",
            Self::Tongan => "to",
            Self::Tajik => "tg",
            Self::Tamil => "ta",
            Self::Tatar => "tt",
            Self::Telugu => "te",
            Self::Thai(None) => "th",
            Self::Thai(Some(ThaiLocale::Thailand)) => "th_th",
            Self::Tibetan => "bo",
            Self::Tsonga => "ts",
            Self::Turkish(None) => "tr",
            Self::Turkish(Some(TurkishLocale::Turkey)) => "tr_tr",
            Self::Turkmen => "tk",
            Self::Uighur => "ug",
            Self::Ukrainian => "uk",
            Self::Urdu => "ur",
            Self::Uzbek(None) => "uz",
            Self::Uzbek(Some(UzbekScript::Arabic)) => "uz-Arab",
            Self::Uzbek(Some(UzbekScript::Latin)) => "uz-Latn",
            Self::Vietnamese => "vi",
            Self::WesternFrisian => "fy",
            Self::Xhosa => "xh",
            Self::Yiddish => "yi",
            Self::Yoruba => "yo",
            Self::Welsh => "cy",
            Self::Zulu => "zu",
            Self::Nonstandard(NonstandardCode::UppercaseEn) => "EN",
            Self::Nonstandard(NonstandardCode::UppercaseTr) => "TR",
            Self::Nonstandard(NonstandardCode::TitlecaseEs) => "Es",
            Self::Nonstandard(NonstandardCode::Hd) => "hd",
            Self::Nonstandard(NonstandardCode::Ld) => "ld",
            Self::Nonstandard(NonstandardCode::Le) => "le",
            Self::Nonstandard(NonstandardCode::Ct) => "ct",
            Self::Nonstandard(NonstandardCode::DDot) => "d.",
            Self::Nonstandard(NonstandardCode::ECaret) => "e^",
            Self::Nonstandard(NonstandardCode::Go) => "go",
            Self::Nonstandard(NonstandardCode::C) => "c",
            Self::Nonstandard(NonstandardCode::Lolc) => "lolc",
            Self::Nonstandard(NonstandardCode::Xx) => "xx",
            Self::Nonstandard(NonstandardCode::Zz) => "zz",
            Self::Nonstandard(NonstandardCode::YHook) => "ƴ",
            Self::Nonstandard(NonstandardCode::TurnedA) => "ɐ",
            Self::Nonstandard(NonstandardCode::PrimeUre) => "‴ure",
            Self::Nonstandard(NonstandardCode::HanChar) => "滀",
            Self::Nonstandard(NonstandardCode::YiQot) => "ꁀ",
            Self::Nonstandard(NonstandardCode::One) => "1",
            Self::Nonstandard(NonstandardCode::AngleQuestion) => "<?",
            Self::Nonstandard(NonstandardCode::AmpHan) => "&退",
            Self::Nonstandard(NonstandardCode::ParenA) => ")a",
            Self::Nonstandard(NonstandardCode::EnglishWord) => "english",
            Self::PossibleCountryCode(PossibleCountryCode::Au) => "au",
            Self::PossibleCountryCode(PossibleCountryCode::Ch) => "ch",
            Self::PossibleCountryCode(PossibleCountryCode::Dk) => "dk",
            Self::PossibleCountryCode(PossibleCountryCode::Gb) => "gb",
            Self::PossibleCountryCode(PossibleCountryCode::Jp) => "jp",
            Self::PossibleCountryCode(PossibleCountryCode::Md) => "md",
            Self::PossibleCountryCode(PossibleCountryCode::Mm) => "mm",
            Self::PossibleCountryCode(PossibleCountryCode::Ph) => "ph",
            Self::PossibleCountryCode(PossibleCountryCode::Se) => "se",
            Self::PossibleCountryCode(PossibleCountryCode::Us) => "us",
            Self::PossibleCountryCode(PossibleCountryCode::Vn) => "vn",
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
        Self::parse_str(s).ok_or_else(|| Error::InvalidLanguage(s.to_string()))
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
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
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
        Self::parse_str(s).ok_or_else(|| Error::InvalidSpecial(s.to_string()))
    }
}

impl From<&Special> for &'static str {
    fn from(value: &Special) -> Self {
        value.as_str()
    }
}

impl Display for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Self as Into<&'static str>>::into(self))
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

const LANGUAGE_VALUES: [Language; 225] = [
    Language::Nonstandard(NonstandardCode::AmpHan),
    Language::Nonstandard(NonstandardCode::ParenA),
    Language::Nonstandard(NonstandardCode::One),
    Language::Nonstandard(NonstandardCode::AngleQuestion),
    Language::Nonstandard(NonstandardCode::UppercaseEn),
    Language::Nonstandard(NonstandardCode::TitlecaseEs),
    Language::Nonstandard(NonstandardCode::UppercaseTr),
    Language::Avestan,
    Language::Afrikaans,
    Language::Amharic,
    Language::Arabic,
    Language::Assamese,
    Language::PossibleCountryCode(PossibleCountryCode::Au),
    Language::Azerbaijani(None),
    Language::Azerbaijani(Some(AzerbaijaniScript::Cyrillic)),
    Language::Belarusian,
    Language::Bulgarian,
    Language::Bengali,
    Language::Tibetan,
    Language::Bosnian(None),
    Language::Bosnian(Some(BosnianScript::Cyrillic)),
    Language::Nonstandard(NonstandardCode::C),
    Language::Catalan,
    Language::Cebuano,
    Language::PossibleCountryCode(PossibleCountryCode::Ch),
    Language::Cherokee,
    Language::CentralKurdish,
    Language::Corsican,
    Language::Cree,
    Language::Czech,
    Language::Nonstandard(NonstandardCode::Ct),
    Language::Welsh,
    Language::Nonstandard(NonstandardCode::DDot),
    Language::Danish,
    Language::German(None),
    Language::German(Some(GermanLocale::Austria)),
    Language::German(Some(GermanLocale::Switzerland)),
    Language::PossibleCountryCode(PossibleCountryCode::Dk),
    Language::Divehi,
    Language::Dzongkha,
    Language::Nonstandard(NonstandardCode::ECaret),
    Language::Ewe,
    Language::Greek,
    Language::English(None),
    Language::English(Some(EnglishLocale::Australia)),
    Language::English(Some(EnglishLocale::Canada)),
    Language::English(Some(EnglishLocale::GreatBritain { capitalized: true })),
    Language::English(Some(EnglishLocale::India)),
    Language::English(Some(EnglishLocale::SouthSudan { capitalized: true })),
    Language::English(Some(EnglishLocale::UnitedStates { capitalized: true })),
    Language::English(Some(EnglishLocale::GreatBritain { capitalized: false })),
    Language::English(Some(EnglishLocale::SouthSudan { capitalized: false })),
    Language::English(Some(EnglishLocale::UnitedStates { capitalized: false })),
    Language::English(Some(EnglishLocale::Unknown)),
    Language::English(Some(EnglishLocale::GreatBritainUnderscore)),
    Language::Nonstandard(NonstandardCode::EnglishWord),
    Language::Esperanto,
    Language::Spanish(None),
    Language::Spanish(Some(SpanishLocale::LatinAmerica)),
    Language::Spanish(Some(SpanishLocale::Spain)),
    Language::Spanish(Some(SpanishLocale::Mexico)),
    Language::Spanish(Some(SpanishLocale::SpainUnderscore)),
    Language::Estonian,
    Language::Basque,
    Language::Persian(None),
    Language::Persian(Some(PersianLocale::Iran)),
    Language::Finnish,
    Language::Filipino(None),
    Language::Filipino(Some(FilipinoLocale::Philippines)),
    Language::Faroese,
    Language::French(None),
    Language::French(Some(FrenchLocale::Canada)),
    Language::French(Some(FrenchLocale::Switzerland)),
    Language::French(Some(FrenchLocale::FranceHyphenated)),
    Language::French(Some(FrenchLocale::France)),
    Language::WesternFrisian,
    Language::Irish,
    Language::PossibleCountryCode(PossibleCountryCode::Gb),
    Language::Gaelic,
    Language::Galician,
    Language::Nonstandard(NonstandardCode::Go),
    Language::SwissGerman,
    Language::Gujarati,
    Language::Manx,
    Language::Hausa,
    Language::Hawaiian,
    Language::Nonstandard(NonstandardCode::Hd),
    Language::Hebrew(HebrewForm::Bare { iso_639_1988: true }),
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
    Language::Italian(None),
    Language::Italian(Some(ItalianLocale::Italy)),
    Language::Italian(Some(ItalianLocale::ItalyUnderscore)),
    Language::Inuktitut,
    Language::Hebrew(HebrewForm::Bare {
        iso_639_1988: false,
    }),
    Language::Hebrew(HebrewForm::Israel),
    Language::Japanese,
    Language::PossibleCountryCode(PossibleCountryCode::Jp),
    Language::Javanese {
        iso_639_1988: false,
    },
    Language::Javanese { iso_639_1988: true },
    Language::Georgian,
    Language::Kazakh,
    Language::CentralKhmer,
    Language::Kannada,
    Language::Korean,
    Language::Kashmiri(KashmiriScript::Arabic),
    Language::Kurdish,
    Language::Cornish,
    Language::Kyrgyz,
    Language::Latin,
    Language::Luxembourgish,
    Language::Nonstandard(NonstandardCode::Ld),
    Language::Nonstandard(NonstandardCode::Le),
    Language::Lao,
    Language::Nonstandard(NonstandardCode::Lolc),
    Language::Lithuanian,
    Language::Luyia,
    Language::Latvian,
    Language::PossibleCountryCode(PossibleCountryCode::Md),
    Language::Malagasy,
    Language::Maori,
    Language::Macedonian,
    Language::Malayam,
    Language::PossibleCountryCode(PossibleCountryCode::Mm),
    Language::Mongolian,
    Language::Marathi,
    Language::Malay(MalayScript::Latin { iso_639_2: false }),
    Language::Malay(MalayScript::Arabic),
    Language::Malay(MalayScript::Latin { iso_639_2: true }),
    Language::Maltese,
    Language::Burmese,
    Language::Neapolitan,
    Language::NorwegianBokmal,
    Language::NorthernNdebele,
    Language::Nepali,
    Language::Dutch(None),
    Language::Dutch(Some(DutchLocale::Belgium)),
    Language::NorwegianNynorsk,
    Language::Norwegian,
    Language::Chichewa,
    Language::Oriya,
    Language::Panjabi(None),
    Language::Panjabi(Some(PanjabiScript::Arabic)),
    Language::PossibleCountryCode(PossibleCountryCode::Ph),
    Language::Polish,
    Language::Pashto,
    Language::Portuguese(None),
    Language::Portuguese(Some(PortugueseLocale::Brazil)),
    Language::Portuguese(Some(PortugueseLocale::Portugal)),
    Language::Quechua,
    Language::Romanian,
    Language::Russian { iso_639_2: false },
    Language::Russian { iso_639_2: true },
    Language::Kinyarwanda,
    Language::Sanskrit,
    Language::Sicilian,
    Language::Sindhi,
    Language::PossibleCountryCode(PossibleCountryCode::Se),
    Language::SerboCroatian,
    Language::Sinhalese,
    Language::Slovak,
    Language::Slovenian,
    Language::Samoan,
    Language::Shona,
    Language::Somali,
    Language::Albanian,
    Language::Serbian(None),
    Language::Serbian(Some(SerbianScript::Latin { capitalized: true })),
    Language::Serbian(Some(SerbianScript::Cyrillic)),
    Language::Serbian(Some(SerbianScript::Latin { capitalized: false })),
    Language::SouthernSotho,
    Language::Sundanese,
    Language::Swedish,
    Language::Swahili,
    Language::Tamil,
    Language::Telugu,
    Language::Tajik,
    Language::Thai(None),
    Language::Thai(Some(ThaiLocale::Thailand)),
    Language::Turkmen,
    Language::Tagalog,
    Language::Klingon,
    Language::Tongan,
    Language::Turkish(None),
    Language::Turkish(Some(TurkishLocale::Turkey)),
    Language::Tsonga,
    Language::Tatar,
    Language::Uighur,
    Language::Ukrainian,
    Language::Urdu,
    Language::PossibleCountryCode(PossibleCountryCode::Us),
    Language::Uzbek(None),
    Language::Uzbek(Some(UzbekScript::Arabic)),
    Language::Uzbek(Some(UzbekScript::Latin)),
    Language::Vietnamese,
    Language::PossibleCountryCode(PossibleCountryCode::Vn),
    Language::Xhosa,
    Language::Nonstandard(NonstandardCode::Xx),
    Language::Yiddish,
    Language::Yoruba,
    Language::Chinese(None),
    Language::Chinese(Some(ChineseLocale::Simplified { capitalized: true })),
    Language::Chinese(Some(ChineseLocale::HongKong)),
    Language::Chinese(Some(ChineseLocale::Han)),
    Language::Chinese(Some(ChineseLocale::Traditional)),
    Language::Chinese(Some(ChineseLocale::Macau)),
    Language::Chinese(Some(ChineseLocale::Singapore)),
    Language::Chinese(Some(ChineseLocale::TaiwaneseMandarin { capitalized: true })),
    Language::Chinese(Some(ChineseLocale::Simplified { capitalized: false })),
    Language::Chinese(Some(ChineseLocale::TaiwaneseMandarin {
        capitalized: false,
    })),
    Language::Zulu,
    Language::Nonstandard(NonstandardCode::Zz),
    Language::Nonstandard(NonstandardCode::YHook),
    Language::Nonstandard(NonstandardCode::TurnedA),
    Language::Nonstandard(NonstandardCode::PrimeUre),
    Language::Nonstandard(NonstandardCode::HanChar),
    Language::Nonstandard(NonstandardCode::YiQot),
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
    use super::{Lang, Language, SelectionLanguage, Special};

    #[test]
    fn parses_selection_placeholders() {
        let cases = [
            (
                "Selecione o idioma ...",
                SelectionLanguage::BrazilianPortuguese,
            ),
            (
                "Selecione O Idioma ...",
                SelectionLanguage::BrazilianPortugueseTitleCase,
            ),
            ("选择语言...", SelectionLanguage::Chinese),
            ("Selecteer een taal...", SelectionLanguage::Dutch),
            ("Select Language...", SelectionLanguage::English),
            ("Pumili ng Wika...", SelectionLanguage::Filipino),
            ("Seleccione Idioma ...", SelectionLanguage::FormalSpanish),
            ("Sélectionnez une langue...", SelectionLanguage::French),
            ("Choisir La Langue ...", SelectionLanguage::FrenchChoisir),
            ("Sprache auswählen...", SelectionLanguage::German),
            ("Επιλογή Γλώσσας...", SelectionLanguage::Greek),
            ("בחירת שפה...", SelectionLanguage::Hebrew),
            ("Pilih Bahasa...", SelectionLanguage::Indonesian),
            ("Seleziona lingua...", SelectionLanguage::Italian),
            ("言語を選択...", SelectionLanguage::Japanese),
            (
                "言語を選択してください...",
                SelectionLanguage::JapanesePolite,
            ),
            ("언어 선택...", SelectionLanguage::Korean),
            ("언어 선택 ...", SelectionLanguage::KoreanSpaced),
            ("Velg språk ...", SelectionLanguage::Norwegian),
            ("انتخاب زبان...", SelectionLanguage::Persian),
            ("Wybierz język ...", SelectionLanguage::Polish),
            ("Selecionar Idioma...", SelectionLanguage::Portuguese),
            ("Selectează limba...", SelectionLanguage::Romanian),
            ("Выберите язык...", SelectionLanguage::Russian),
            ("Одаберите језик ...", SelectionLanguage::Serbian),
            ("Selecciona un idioma...", SelectionLanguage::Spanish),
            (
                "Selecciona Idioma de la ...",
                SelectionLanguage::SpanishDeLa,
            ),
            ("Välj språk...", SelectionLanguage::Swedish),
            ("اختر اللغة...", SelectionLanguage::Arabic),
            ("اختر اللغة ...", SelectionLanguage::ArabicSpaced),
            ("เลือกภาษา...", SelectionLanguage::Thai),
            ("選擇語言...", SelectionLanguage::TraditionalChinese),
            ("Dil Seç...", SelectionLanguage::Turkish),
            ("Dil Seçin ...", SelectionLanguage::TurkishSecin),
        ];

        for (text, variant) in cases {
            let lang = Lang::Selection(variant);
            assert_eq!(text.parse::<Lang>().unwrap(), lang, "parsing {text:?}");
            // Each selector round-trips exactly through `as_str`.
            assert_eq!(lang.as_str(), text, "round-trip {variant:?}");
        }
    }

    #[test]
    fn deserializes_unborrowable_placeholder() {
        // The `é` arrives `é`-escaped, so serde_json must unescape it into an
        // owned buffer; deserialization must not require a borrowed string.
        let json = r#""S\u00e9lectionnez une langue...""#;
        let lang: Lang = serde_json::from_str(json).unwrap();

        assert_eq!(lang, Lang::Selection(SelectionLanguage::French));
    }

    #[test]
    fn language_codes_sorted() {
        let mut codes = super::LANGUAGE_CODES.to_vec();
        codes.sort_unstable();

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
        codes.sort_unstable();

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
            .map(super::Lang::as_str)
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

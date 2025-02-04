#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Lang {
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "ca")]
    Catalan,
    #[serde(rename = "ckb")]
    CentralKurdish,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "cy")]
    Welsh,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "en-AU")]
    EnglishAu,
    #[serde(rename = "en-gb", alias = "en-GB")]
    EnglishGb,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "et")]
    Estonian,
    #[serde(rename = "eu")]
    Basque,
    #[serde(rename = "fa")]
    Persian,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "gl")]
    Galician,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "ht")]
    Haitian,
    #[serde(rename = "hu")]
    Hungarian,
    // Twitter generally seems to use the older 639:1988 abbreviation.
    #[serde(rename = "in", alias = "id")]
    Indonesian,
    #[serde(rename = "is")]
    Icelandic,
    #[serde(rename = "it")]
    Italian,
    /// Note that Twitter often uses the old ISO-639 language abbreviation for Hebrew.
    #[serde(rename = "iw", alias = "he")]
    Hebrew,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "km")]
    CentralKhmer,
    #[serde(rename = "kn")]
    Kannada,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "mr")]
    Marathi,
    #[serde(rename = "my")]
    Burmese,
    #[serde(rename = "ne")]
    Nepali,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "ps")]
    Pashto,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "sl")]
    Slovenian,
    #[serde(rename = "sr")]
    Serbian,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "ta")]
    Tamil,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "te")]
    Telugu,
    #[serde(rename = "tl")]
    Tagalog,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "uk")]
    Ukrainian,
    #[serde(rename = "ur")]
    Urdu,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "zh-Hans")]
    ChineseHan,
    #[serde(rename = "zh-TW", alias = "zh-tw")]
    TaiwaneseMandarin,
    #[serde(rename = "zh-cn")]
    SimplifiedChinese,
    #[serde(rename = "und")]
    Und,
    #[serde(rename = "art")]
    Art,
    #[serde(rename = "qam")]
    Mention,
    #[serde(rename = "qct")]
    Cashtag,
    #[serde(rename = "qht")]
    Hashtag,
    #[serde(rename = "qme")]
    MediaLink,
    #[serde(rename = "qst")]
    ShortText,
    #[serde(rename = "zxx")]
    NoText,
}

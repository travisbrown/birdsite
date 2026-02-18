#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Location {
    Africa,
    Albania,
    Algeria,
    Argentina,
    Australasia,
    Australia,
    Austria,
    Bahrain,
    Bangladesh,
    Belgium,
    #[serde(rename = "Bosnia and Herzegovina")]
    BosniaAndHerzegovina,
    Brazil,
    Bulgaria,
    Cambodia,
    Canada,
    Chile,
    Colombia,
    #[serde(rename = "Costa Rica")]
    CostaRica,
    Croatia,
    Cuba,
    Cyprus,
    #[serde(rename = "Czech Republic")]
    CzechRepublic,
    Denmark,
    #[serde(rename = "East Asia & Pacific")]
    EastAsiaPacific,
    #[serde(rename = "Eastern Europe (Non-EU)")]
    EasternEuropeNonEu,
    Ecuador,
    Egypt,
    #[serde(rename = "El Salvador")]
    ElSalvador,
    Estonia,
    Europe,
    Finland,
    France,
    Georgia,
    Germany,
    Ghana,
    Greece,
    #[serde(rename = "Hong Kong")]
    HongKong,
    Hungary,
    India,
    Indonesia,
    Iraq,
    Ireland,
    #[serde(rename = "Isle of Man")]
    IsleOfMan,
    Israel,
    Italy,
    Japan,
    Jordan,
    Kenya,
    Lebanon,
    Libya,
    Lithuania,
    Luxembourg,
    Macedonia,
    Malaysia,
    Maldives,
    Malta,
    Mexico,
    Moldova,
    Montenegro,
    Morocco,
    Namibia,
    Nepal,
    Netherlands,
    #[serde(rename = "New Zealand")]
    NewZealand,
    Nicaragua,
    Nigeria,
    #[serde(rename = "North Africa")]
    NorthAfrica,
    #[serde(rename = "North America")]
    NorthAmerica,
    Norway,
    Pakistan,
    Panama,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    Romania,
    #[serde(rename = "Russian Federation")]
    RussianFederation,
    #[serde(rename = "Saudi Arabia")]
    SaudiArabia,
    Serbia,
    Singapore,
    Slovakia,
    Slovenia,
    Somalia,
    #[serde(rename = "South Africa")]
    SouthAfrica,
    #[serde(rename = "South America")]
    SouthAmerica,
    #[serde(rename = "South Asia")]
    SouthAsia,
    Spain,
    Sweden,
    Switzerland,
    #[serde(rename = "Syrian Arab Republic")]
    SyrianArabRepublic,
    Taiwan,
    Thailand,
    Togo,
    Tunisia,
    Turkey,
    Uganda,
    Ukraine,
    #[serde(rename = "United Arab Emirates")]
    UnitedArabEmirates,
    #[serde(rename = "United Kingdom")]
    UnitedKingdom,
    #[serde(rename = "United States")]
    UnitedStates,
    Uruguay,
    #[serde(rename = "Viet Nam")]
    VietNam,
    #[serde(rename = "West Asia")]
    WestAsia,
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(deny_unknown_fields)]
pub enum Location {
    Afghanistan,
    Africa,
    Albania,
    Algeria,
    Andorra,
    Angola,
    #[serde(rename = "Antigua and Barbuda")]
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Australasia,
    Australia,
    Austria,
    Azerbaijan,
    Bahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bermuda,
    Bhutan,
    Bolivia,
    Bonaire,
    #[serde(rename = "Bosnia and Herzegovina")]
    BosniaAndHerzegovina,
    Botswana,
    Brazil,
    #[serde(rename = "British Virgin Islands")]
    BritishVirginIslands,
    #[serde(rename = "Brunei Darussalam")]
    BruneiDarussalam,
    Bulgaria,
    #[serde(rename = "Burkina Faso")]
    BurkinaFaso,
    Burundi,
    Cambodia,
    Cameroon,
    Canada,
    #[serde(rename = "Cape Verde")]
    CapeVerde,
    Caribbean,
    #[serde(rename = "Cayman Islands")]
    CaymanIslands,
    #[serde(rename = "Central Asia")]
    CentralAsia,
    Chad,
    Chile,
    China,
    Colombia,
    Comoros,
    Congo,
    #[serde(rename = "Costa Rica")]
    CostaRica,
    #[serde(rename = "Côte d'Ivoire")]
    CoteDIvoire,
    Croatia,
    Cuba,
    #[serde(rename = "Curaçao")]
    Curacao,
    Cyprus,
    #[serde(rename = "Czech Republic")]
    CzechRepublic,
    Denmark,
    Djibouti,
    Dominica,
    #[serde(rename = "Dominican Republic")]
    DominicanRepublic,
    #[serde(rename = "East Asia & Pacific")]
    EastAsiaPacific,
    #[serde(rename = "Eastern Europe (Non-EU)")]
    EasternEuropeNonEu,
    Ecuador,
    Egypt,
    #[serde(rename = "El Salvador")]
    ElSalvador,
    #[serde(rename = "Equatorial Guinea")]
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Ethiopia,
    Europe,
    #[serde(rename = "Faroe Islands")]
    FaroeIslands,
    Fiji,
    Finland,
    France,
    #[serde(rename = "French Guiana")]
    FrenchGuiana,
    #[serde(rename = "French Polynesia")]
    FrenchPolynesia,
    Gabon,
    Gambia,
    Georgia,
    Germany,
    Ghana,
    Gibraltar,
    Greece,
    Greenland,
    Grenada,
    Guadeloupe,
    Guam,
    Guatemala,
    Guernsey,
    Guinea,
    #[serde(rename = "Guinea-Bissau")]
    GuineaBissau,
    Guyana,
    Haiti,
    Honduras,
    #[serde(rename = "Hong Kong")]
    HongKong,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    Ireland,
    #[serde(rename = "Isle of Man")]
    IsleOfMan,
    Israel,
    Italy,
    Jamaica,
    Japan,
    Jersey,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Korea,
    Kosovo,
    Kuwait,
    Kyrgyzstan,
    #[serde(rename = "Lao People's Democratic Republic")]
    LaoPeoplesDemocraticRepublic,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Macao,
    Macedonia,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    Martinique,
    Mauritania,
    Mauritius,
    Mexico,
    Micronesia,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nepal,
    Netherlands,
    #[serde(rename = "New Caledonia")]
    NewCaledonia,
    #[serde(rename = "New Zealand")]
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    #[serde(rename = "North Africa")]
    NorthAfrica,
    #[serde(rename = "North America")]
    NorthAmerica,
    #[serde(rename = "Northern Mariana Islands")]
    NorthernMarianaIslands,
    Norway,
    Oman,
    Pakistan,
    Palestine,
    Panama,
    #[serde(rename = "Papua New Guinea")]
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    #[serde(rename = "Puerto Rico")]
    PuertoRico,
    Qatar,
    #[serde(rename = "Réunion")]
    Reunion,
    Romania,
    #[serde(rename = "Russian Federation")]
    RussianFederation,
    Rwanda,
    #[serde(rename = "Saint Kitts and Nevis")]
    SaintKittsAndNevis,
    #[serde(rename = "Saint Lucia")]
    SaintLucia,
    #[serde(rename = "Saint Vincent and the Grenadines")]
    SaintVincentAndTheGrenadines,
    Samoa,
    #[serde(rename = "Sao Tome and Principe")]
    SaoTomeAndPrincipe,
    #[serde(rename = "Saudi Arabia")]
    SaudiArabia,
    Senegal,
    Serbia,
    Seychelles,
    #[serde(rename = "Sierra Leone")]
    SierraLeone,
    Singapore,
    Slovakia,
    Slovenia,
    #[serde(rename = "Solomon Islands")]
    SolomonIslands,
    Somalia,
    #[serde(rename = "South Africa")]
    SouthAfrica,
    #[serde(rename = "South America")]
    SouthAmerica,
    #[serde(rename = "South Asia")]
    SouthAsia,
    #[serde(rename = "South Sudan")]
    SouthSudan,
    Spain,
    #[serde(rename = "Sri Lanka")]
    SriLanka,
    Sudan,
    Suriname,
    Swaziland,
    Sweden,
    Switzerland,
    #[serde(rename = "Syrian Arab Republic")]
    SyrianArabRepublic,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    #[serde(rename = "Timor-Leste")]
    TimorLeste,
    Togo,
    Tonga,
    #[serde(rename = "Trinidad and Tobago")]
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    #[serde(rename = "Türkiye")]
    Turkiye,
    #[serde(rename = "Turks and Caicos Islands")]
    TurksAndCaicosIslands,
    Uganda,
    Ukraine,
    #[serde(rename = "United Arab Emirates")]
    UnitedArabEmirates,
    #[serde(rename = "United Kingdom")]
    UnitedKingdom,
    #[serde(rename = "United States")]
    UnitedStates,
    Uruguay,
    #[serde(rename = "US Virgin Islands")]
    UsVirginIslands,
    Uzbekistan,
    Vanuatu,
    Venezuela,
    #[serde(rename = "Viet Nam")]
    VietNam,
    #[serde(rename = "West Asia")]
    WestAsia,
    Yemen,
    Zambia,
    Zimbabwe,
    // Country codes appear briefly in November 2025 (e.g. 2025-11-26).
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GB", alias = "gb")]
    Gb,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "rs")]
    Rs,
    #[serde(rename = "SE", alias = "se")]
    Se,
    #[serde(rename = "US", alias = "us")]
    Us,
}

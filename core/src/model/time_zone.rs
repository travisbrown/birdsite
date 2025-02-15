#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TimeZone {
    #[serde(rename = "CDT")]
    Cdt,
    #[serde(rename = "CET")]
    Cet,
    #[serde(rename = "CST")]
    Cst,
    #[serde(rename = "Central Time (US & Canada)")]
    Central,
    #[serde(rename = "Mountain Time (US & Canada)")]
    Mountain,
    #[serde(rename = "Pacific Time (US & Canada)")]
    Pacific,
    #[serde(rename = "Eastern Time (US & Canada)")]
    Eastern,
    #[serde(rename = "Indiana (East)")]
    IndianaEast,
    #[serde(rename = "Atlantic Time (Canada)")]
    AtlanticCanada,
    #[serde(rename = "Africa/Algiers")]
    AfricaAlgiers,
    #[serde(rename = "Africa/Cairo")]
    AfricaCairo,
    #[serde(rename = "Africa/Dakar")]
    AfricaDakar,
    #[serde(rename = "Africa/Lagos")]
    AfricaLagos,
    #[serde(rename = "America/Anchorage")]
    AmericaAnchorage,
    #[serde(rename = "America/Argentina/Buenos_Aires")]
    AmericaArgentinaBuenosAires,
    #[serde(rename = "America/Boise")]
    AmericaBoise,
    #[serde(rename = "America/Caracas")]
    AmericaCaracas,
    #[serde(rename = "America/Chicago")]
    AmericaChicago,
    #[serde(rename = "America/Dawson")]
    AmericaDawson,
    #[serde(rename = "America/Denver")]
    AmericaDenver,
    #[serde(rename = "America/Detroit")]
    AmericaDetroit,
    #[serde(rename = "America/Edmonton")]
    AmericaEdmonton,
    #[serde(rename = "America/Lima")]
    AmericaLima,
    #[serde(rename = "America/Los_Angeles")]
    AmericaLosAngeles,
    #[serde(rename = "America/Mexico_City")]
    AmericaMexicoCity,
    #[serde(rename = "America/New_York")]
    AmericaNewYork,
    #[serde(rename = "America/Phoenix")]
    AmericaPhoenix,
    #[serde(rename = "America/Toronto")]
    AmericaToronto,
    #[serde(rename = "America/Vancouver")]
    AmericaVancouver,
    #[serde(rename = "Asia/Calcutta")]
    AsiaCalcutta,
    #[serde(rename = "Asia/Dubai")]
    AsiaDubai,
    #[serde(rename = "Asia/Kabul")]
    AsiaKabul,
    #[serde(rename = "Asia/Riyadh")]
    AsiaRiyadh,
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
    #[serde(rename = "Australia/Hobart")]
    AustraliaHobart,
    #[serde(rename = "Australia/Sydney")]
    AustraliaSydney,
    #[serde(rename = "Europe/Amsterdam")]
    EuropeAmsterdam,
    #[serde(rename = "Europe/Belfast")]
    EuropeBelfast,
    #[serde(rename = "Europe/Berlin")]
    EuropeBerlin,
    #[serde(rename = "Europe/Dublin")]
    EuropeDublin,
    #[serde(rename = "Europe/London")]
    EuropeLondon,
    #[serde(rename = "Europe/Paris")]
    EuropeParis,
    #[serde(rename = "Europe/Rome")]
    EuropeRome,
    #[serde(rename = "Europe/Stockholm")]
    EuropeStockholm,
    #[serde(rename = "Europe/Vienna")]
    EuropeVienna,
    #[serde(rename = "Europe/Warsaw")]
    EuropeWarsaw,
    #[serde(rename = "Pacific/Honolulu")]
    PacificHonolulu,
    #[serde(rename = "Abu Dhabi")]
    AbuDhabi,
    #[serde(rename = "Buenos Aires")]
    BuenosAires,
    #[serde(rename = "Cape Verde Is.")]
    CapeVerdeIs,
    #[serde(rename = "Central America")]
    CentralAmerica,
    #[serde(rename = "International Date Line West")]
    InternationalDateLineWest,
    #[serde(rename = "Hong Kong")]
    HongKong,
    #[serde(rename = "Kuala Lumpur")]
    KualaLumpur,
    #[serde(rename = "Mexico City")]
    MexicoCity,
    #[serde(rename = "Mid-Atlantic")]
    MidAtlantic,
    #[serde(rename = "Midway Island")]
    MidwayIsland,
    #[serde(rename = "New Caledonia")]
    NewCaledonia,
    #[serde(rename = "New Delhi")]
    NewDelhi,
    #[serde(rename = "Solomon Is.")]
    SolomonIs,
    #[serde(rename = "St. Petersburg")]
    StPetersburg,
    #[serde(rename = "West Central Africa")]
    WestCentralAfrica,
    #[serde(rename = "BST")]
    Bst,
    #[serde(rename = "EDT")]
    Edt,
    #[serde(rename = "UTC")]
    Utc,
    Adelaide,
    Alaska,
    Almaty,
    Amsterdam,
    Arizona,
    Athens,
    Auckland,
    Baghdad,
    Bangkok,
    Beijing,
    Belgrade,
    Berlin,
    Bern,
    Bogota,
    Brasilia,
    Brisbane,
    Brussels,
    Bucharest,
    Budapest,
    Cairo,
    Caracas,
    Casablanca,
    Chennai,
    Chongqing,
    Copenhagen,
    Dublin,
    Edinburgh,
    Ekaterinburg,
    Fiji,
    Georgetown,
    Greenland,
    Guam,
    Hanoi,
    Harare,
    Hawaii,
    Helsinki,
    Hobart,
    Irkutsk,
    Islamabad,
    Istanbul,
    Jakarta,
    Jerusalem,
    Kabul,
    Karachi,
    Kolkata,
    Kuwait,
    Kyiv,
    Lima,
    Lisbon,
    Ljubljana,
    London,
    Madrid,
    Mazatlan,
    Melbourne,
    Minsk,
    Monterrey,
    Moscow,
    Mumbai,
    Muscat,
    Nairobi,
    Newfoundland,
    Novosibirsk,
    Osaka,
    Paris,
    Perth,
    Prague,
    Pretoria,
    Quito,
    Riga,
    Riyadh,
    Rome,
    Santiago,
    Sarajevo,
    Saskatchewan,
    Seoul,
    Singapore,
    Sofia,
    Stockholm,
    Sydney,
    Taipei,
    Tehran,
    Tijuana,
    Tokyo,
    Urumqi,
    Vienna,
    Volgograd,
    Warsaw,
    Wellington,
    Yakutsk,
    Yerevan,
    Zagreb,
}

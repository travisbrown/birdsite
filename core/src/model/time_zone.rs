#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TimeZone {
    #[serde(rename = "BST")]
    Bst,
    #[serde(rename = "CDT")]
    Cdt,
    #[serde(rename = "CET")]
    Cet,
    #[serde(rename = "CST")]
    Cst,
    #[serde(rename = "EDT")]
    Edt,
    #[serde(rename = "EST")]
    Est,
    #[serde(rename = "GMT")]
    Gmt,
    #[serde(rename = "GMT+8")]
    Gmt8,
    #[serde(rename = "JST")]
    Jst,
    #[serde(rename = "MDT")]
    Mdt,
    #[serde(rename = "MST")]
    Mst,
    #[serde(rename = "PDT")]
    Pdt,
    #[serde(rename = "PST")]
    Pst,
    #[serde(rename = "UTC")]
    Utc,
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
    #[serde(rename = "Africa/Bangui")]
    AfricaBangui,
    #[serde(rename = "Africa/Cairo")]
    AfricaCairo,
    #[serde(rename = "Africa/Casablanca")]
    AfricaCasablanca,
    #[serde(rename = "Africa/Dakar")]
    AfricaDakar,
    #[serde(rename = "Africa/Harare")]
    AfricaHarare,
    #[serde(rename = "Africa/Johannesburg")]
    AfricaJohannesburg,
    #[serde(rename = "Africa/Lagos")]
    AfricaLagos,
    #[serde(rename = "Africa/Nairobi")]
    AfricaNairobi,
    #[serde(rename = "Africa/Nouakchott")]
    AfricaNouakchott,
    #[serde(rename = "America/Anchorage")]
    AmericaAnchorage,
    #[serde(rename = "America/Antigua")]
    AmericaAntigua,
    #[serde(rename = "America/Argentina/Buenos_Aires")]
    AmericaArgentinaBuenosAires,
    #[serde(rename = "America/Bahia_Banderas")]
    AmericaBahiaBanderas,
    #[serde(rename = "America/Bogota")]
    AmericaBogota,
    #[serde(rename = "America/Boise")]
    AmericaBoise,
    #[serde(rename = "America/Buenos_Aires")]
    AmericaBuenosAires,
    #[serde(rename = "America/Cancun")]
    AmericaCancun,
    #[serde(rename = "America/Caracas")]
    AmericaCaracas,
    #[serde(rename = "America/Chicago")]
    AmericaChicago,
    #[serde(rename = "America/Costa_Rica")]
    AmericaCostaRica,
    #[serde(rename = "America/Dawson")]
    AmericaDawson,
    #[serde(rename = "America/Denver")]
    AmericaDenver,
    #[serde(rename = "America/Detroit")]
    AmericaDetroit,
    #[serde(rename = "America/Edmonton")]
    AmericaEdmonton,
    #[serde(rename = "America/El_Salvador")]
    AmericaElSalvador,
    #[serde(rename = "America/Indiana/Indianapolis")]
    AmericaIndianaIndianapolis,
    #[serde(rename = "America/La_Paz")]
    AmericaLaPaz,
    #[serde(rename = "America/Lima")]
    AmericaLima,
    #[serde(rename = "America/Los_Angeles")]
    AmericaLosAngeles,
    #[serde(rename = "America/Manaus")]
    AmericaManaus,
    #[serde(rename = "America/Mexico_City")]
    AmericaMexicoCity,
    #[serde(rename = "America/Montevideo")]
    AmericaMontevideo,
    #[serde(rename = "America/New_York")]
    AmericaNewYork,
    #[serde(rename = "America/Phoenix")]
    AmericaPhoenix,
    #[serde(rename = "America/Puerto_Rico")]
    AmericaPuertoRico,
    #[serde(rename = "America/Recife")]
    AmericaRecife,
    #[serde(rename = "America/Regina")]
    AmericaRegina,
    #[serde(rename = "America/Resolute")]
    AmericaResolute,
    #[serde(rename = "America/Santiago")]
    AmericaSantiago,
    #[serde(rename = "America/Sao_Paulo")]
    AmericaSaoPaulo,
    #[serde(rename = "America/Toronto")]
    AmericaToronto,
    #[serde(rename = "America/Vancouver")]
    AmericaVancouver,
    #[serde(rename = "Asia/Amman")]
    AsiaAmman,
    #[serde(rename = "Asia/Bangkok")]
    AsiaBangkok,
    #[serde(rename = "Asia/Calcutta")]
    AsiaCalcutta,
    #[serde(rename = "Asia/Dhaka")]
    AsiaDhaka,
    #[serde(rename = "Asia/Dubai")]
    AsiaDubai,
    #[serde(rename = "Asia/Jerusalem")]
    AsiaJerusalem,
    #[serde(rename = "Asia/Kabul")]
    AsiaKabul,
    #[serde(rename = "Asia/Karachi")]
    AsiaKarachi,
    #[serde(rename = "Asia/Kolkata")]
    AsiaKolkata,
    #[serde(rename = "Asia/Manila")]
    AsiaManila,
    #[serde(rename = "Asia/Riyadh")]
    AsiaRiyadh,
    #[serde(rename = "Asia/Seoul")]
    AsiaSeoul,
    #[serde(rename = "Asia/Shanghai")]
    AsiaShanghai,
    #[serde(rename = "Asia/Singapore")]
    AsiaSingapore,
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
    #[serde(rename = "Atlantic/Madeira")]
    AtlanticMadeira,
    #[serde(rename = "Australia/Hobart")]
    AustraliaHobart,
    #[serde(rename = "Australia/Perth")]
    AustraliaPerth,
    #[serde(rename = "Australia/Sydney")]
    AustraliaSydney,
    #[serde(rename = "Europe/Amsterdam")]
    EuropeAmsterdam,
    #[serde(rename = "Europe/Belfast")]
    EuropeBelfast,
    #[serde(rename = "Europe/Belgrade")]
    EuropeBelgrade,
    #[serde(rename = "Europe/Berlin")]
    EuropeBerlin,
    #[serde(rename = "Europe/Brussels")]
    EuropeBrussels,
    #[serde(rename = "Europe/Dublin")]
    EuropeDublin,
    #[serde(rename = "Europe/Helsinki")]
    EuropeHelsinki,
    #[serde(rename = "Europe/Istanbul")]
    EuropeIstanbul,
    #[serde(rename = "Europe/Kiev")]
    EuropeKiev,
    #[serde(rename = "Europe/London")]
    EuropeLondon,
    #[serde(rename = "Europe/Luxembourg")]
    EuropeLuxembourg,
    #[serde(rename = "Europe/Madrid")]
    EuropeMadrid,
    #[serde(rename = "Europe/Moscow")]
    EuropeMoscow,
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
    #[serde(rename = "Europe/Zurich")]
    EuropeZurich,
    #[serde(rename = "Pacific/Auckland")]
    PacificAuckland,
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
    #[serde(rename = "La Paz")]
    LaPaz,
    #[serde(rename = "Marshall Is.")]
    MarshallIs,
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
    #[serde(rename = "Nuku'alofa")]
    NukuAlofa,
    #[serde(rename = "Port Moresby")]
    PortMoresby,
    #[serde(rename = "Solomon Is.")]
    SolomonIs,
    #[serde(rename = "Sri Jayawardenepura")]
    SriJayawardenepura,
    #[serde(rename = "St. Petersburg")]
    StPetersburg,
    #[serde(rename = "Ulaan Bataar")]
    UlaanBataar,
    #[serde(rename = "West Central Africa")]
    WestCentralAfrica,
    Adelaide,
    Alaska,
    Almaty,
    Amsterdam,
    Arizona,
    Athens,
    Auckland,
    Azores,
    Baghdad,
    Baku,
    Bangkok,
    Beijing,
    Belgrade,
    Berlin,
    Bern,
    Bogota,
    Brasilia,
    Bratislava,
    Brisbane,
    Brussels,
    Bucharest,
    Budapest,
    Cairo,
    Canberra,
    Caracas,
    Casablanca,
    Chennai,
    Chihuahua,
    Chongqing,
    Copenhagen,
    Darwin,
    Dhaka,
    Dublin,
    Edinburgh,
    Ekaterinburg,
    Fiji,
    Georgetown,
    Greenland,
    Guadalajara,
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
    Kamchatka,
    Karachi,
    Kathmandu,
    Kiev,
    Kolkata,
    Krasnoyarsk,
    Kuwait,
    Kyiv,
    Lima,
    Lisbon,
    Ljubljana,
    London,
    Madrid,
    Magadan,
    Mazatlan,
    Melbourne,
    Minsk,
    Monrovia,
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
    Rangoon,
    Riga,
    Riyadh,
    Rome,
    Samoa,
    Santiago,
    Sapporo,
    Sarajevo,
    Saskatchewan,
    Seoul,
    Singapore,
    Skopje,
    Sofia,
    Stockholm,
    Sydney,
    Taipei,
    Tallinn,
    Tashkent,
    Tbilisi,
    Tehran,
    Tijuana,
    Tokyo,
    Urumqi,
    Vienna,
    Vilnius,
    Volgograd,
    Warsaw,
    Wellington,
    Yakutsk,
    Yerevan,
    Zagreb,
}

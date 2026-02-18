#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Source {
    #[serde(rename = "Africa Android App")]
    AfricaAndroidApp,
    #[serde(rename = "Algeria Android App")]
    AlgeriaAndroidApp,
    #[serde(rename = "Argentina Android App")]
    ArgentinaAndroidApp,
    #[serde(rename = "Argentina App Store")]
    ArgentinaAppStore,
    #[serde(rename = "Australasia Android App")]
    AustralasiaAndroidApp,
    #[serde(rename = "Australasia App Store")]
    AustralasiaAppStore,
    #[serde(rename = "Australia Android App")]
    AustraliaAndroidApp,
    #[serde(rename = "Australia App Store")]
    AustraliaAppStore,
    #[serde(rename = "Austria Android App")]
    AustriaAndroidApp,
    #[serde(rename = "Austria App Store")]
    AustriaAppStore,
    #[serde(rename = "Bangladesh Android App")]
    BangladeshAndroidApp,
    #[serde(rename = "Belgium Android App")]
    BelgiumAndroidApp,
    #[serde(rename = "Belgium App Store")]
    BelgiumAppStore,
    #[serde(rename = "Bosnia and Herzegovina Android App")]
    BosniaAndHerzegovinaAndroidApp,
    #[serde(rename = "Brazil Android App")]
    BrazilAndroidApp,
    #[serde(rename = "Brazil App Store")]
    BrazilAppStore,
    #[serde(rename = "Bulgaria Android App")]
    BulgariaAndroidApp,
    #[serde(rename = "Bulgaria App Store")]
    BulgariaAppStore,
    #[serde(rename = "Cambodia Android App")]
    CambodiaAndroidApp,
    #[serde(rename = "Canada Android App")]
    CanadaAndroidApp,
    #[serde(rename = "Canada App Store")]
    CanadaAppStore,
    #[serde(rename = "Chile Android App")]
    ChileAndroidApp,
    #[serde(rename = "Chile App Store")]
    ChileAppStore,
    #[serde(rename = "China Android App")]
    ChinaAndroidApp,
    #[serde(rename = "China App Store")]
    ChinaAppStore,
    #[serde(rename = "Colombia Android App")]
    ColombiaAndroidApp,
    #[serde(rename = "Colombia App Store")]
    ColombiaAppStore,
    #[serde(rename = "Costa Rica Android App")]
    CostaRicaAndroidApp,
    #[serde(rename = "Costa Rica App Store")]
    CostaRicaAppStore,
    #[serde(rename = "Croatia Android App")]
    CroatiaAndroidApp,
    #[serde(rename = "Croatia App Store")]
    CroatiaAppStore,
    #[serde(rename = "Cuba Android App")]
    CubaAndroidApp,
    #[serde(rename = "Cyprus Android App")]
    CyprusAndroidApp,
    #[serde(rename = "Cyprus App Store")]
    CyprusAppStore,
    #[serde(rename = "Czech Republic Android App")]
    CzechRepublicAndroidApp,
    #[serde(rename = "Czech Republic App Store")]
    CzechRepublicAppStore,
    #[serde(rename = "Denmark Android App")]
    DenmarkAndroidApp,
    #[serde(rename = "Denmark App Store")]
    DenmarkAppStore,
    #[serde(rename = "East Asia & Pacific Android App")]
    EastAsiaPacificAndroidApp,
    #[serde(rename = "East Asia & Pacific App Store")]
    EastAsiaPacificAppStore,
    #[serde(rename = "Eastern Europe (Non-EU) Android App")]
    EasternEuropeNonEuAndroidApp,
    #[serde(rename = "Ecuador Android App")]
    EcuadorAndroidApp,
    #[serde(rename = "Ecuador App Store")]
    EcuadorAppStore,
    #[serde(rename = "Egypt Android App")]
    EgyptAndroidApp,
    #[serde(rename = "El Salvador Android App")]
    ElSalvadorAndroidApp,
    #[serde(rename = "Europe Android App")]
    EuropeAndroidApp,
    #[serde(rename = "Europe App Store")]
    EuropeAppStore,
    #[serde(rename = "Finland Android App")]
    FinlandAndroidApp,
    #[serde(rename = "Finland App Store")]
    FinlandAppStore,
    #[serde(rename = "France Android App")]
    FranceAndroidApp,
    #[serde(rename = "France App Store")]
    FranceAppStore,
    #[serde(rename = "Georgia App Store")]
    GeorgiaAppStore,
    #[serde(rename = "Germany Android App")]
    GermanyAndroidApp,
    #[serde(rename = "Germany App Store")]
    GermanyAppStore,
    #[serde(rename = "Ghana Android App")]
    GhanaAndroidApp,
    #[serde(rename = "Greece Android App")]
    GreeceAndroidApp,
    #[serde(rename = "Greece App Store")]
    GreeceAppStore,
    #[serde(rename = "Hong Kong App Store")]
    HongKongAppStore,
    #[serde(rename = "Hungary Android App")]
    HungaryAndroidApp,
    #[serde(rename = "Hungary App Store")]
    HungaryAppStore,
    #[serde(rename = "India Android App")]
    IndiaAndroidApp,
    #[serde(rename = "India App Store")]
    IndiaAppStore,
    #[serde(rename = "Indonesia Android App")]
    IndonesiaAndroidApp,
    #[serde(rename = "Indonesia App Store")]
    IndonesiaAppStore,
    #[serde(rename = "Iran Android App")]
    IranAndroidApp,
    #[serde(rename = "Iraq Android App")]
    IraqAndroidApp,
    #[serde(rename = "Ireland Android App")]
    IrelandAndroidApp,
    #[serde(rename = "Ireland App Store")]
    IrelandAppStore,
    #[serde(rename = "Israel App Store")]
    IsraelAppStore,
    #[serde(rename = "Italy Android App")]
    ItalyAndroidApp,
    #[serde(rename = "Italy App Store")]
    ItalyAppStore,
    #[serde(rename = "Japan Android App")]
    JapanAndroidApp,
    #[serde(rename = "Japan App Store")]
    JapanAppStore,
    #[serde(rename = "Jersey Android App")]
    JerseyAndroidApp,
    #[serde(rename = "Kenya Android App")]
    KenyaAndroidApp,
    #[serde(rename = "Kuwait App Store")]
    KuwaitAppStore,
    #[serde(rename = "Lebanon App Store")]
    LebanonAppStore,
    #[serde(rename = "Lithuania Android App")]
    LithuaniaAndroidApp,
    #[serde(rename = "Lithuania App Store")]
    LithuaniaAppStore,
    #[serde(rename = "Luxembourg Android App")]
    LuxembourgAndroidApp,
    #[serde(rename = "Macedonia Android App")]
    MacedoniaAndroidApp,
    #[serde(rename = "Macedonia App Store")]
    MacedoniaAppStore,
    #[serde(rename = "Malaysia Android App")]
    MalaysiaAndroidApp,
    #[serde(rename = "Malta App Store")]
    MaltaAppStore,
    #[serde(rename = "Mexico Android App")]
    MexicoAndroidApp,
    #[serde(rename = "Mexico App Store")]
    MexicoAppStore,
    #[serde(rename = "Moldova Android App")]
    MoldovaAndroidApp,
    #[serde(rename = "Montenegro Android App")]
    MontenegroAndroidApp,
    #[serde(rename = "Morocco Android App")]
    MoroccoAndroidApp,
    #[serde(rename = "Netherlands Android App")]
    NetherlandsAndroidApp,
    #[serde(rename = "Netherlands App Store")]
    NetherlandsAppStore,
    #[serde(rename = "New Zealand Android App")]
    NewZealandAndroidApp,
    #[serde(rename = "New Zealand App Store")]
    NewZealandAppStore,
    #[serde(rename = "Nicaragua Android App")]
    NicaraguaAndroidApp,
    #[serde(rename = "Nigeria Android App")]
    NigeriaAndroidApp,
    #[serde(rename = "Nigeria App Store")]
    NigeriaAppStore,
    #[serde(rename = "North Africa Android App")]
    NorthAfricaAndroidApp,
    #[serde(rename = "North America Android App")]
    NorthAmericaAndroidApp,
    #[serde(rename = "North America App Store")]
    NorthAmericaAppStore,
    #[serde(rename = "Norway Android App")]
    NorwayAndroidApp,
    #[serde(rename = "Norway App Store")]
    NorwayAppStore,
    #[serde(rename = "Pakistan Android App")]
    PakistanAndroidApp,
    #[serde(rename = "Panama App Store")]
    PanamaAppStore,
    #[serde(rename = "Paraguay Android App")]
    ParaguayAndroidApp,
    #[serde(rename = "Philippines Android App")]
    PhilippinesAndroidApp,
    #[serde(rename = "Philippines App Store")]
    PhilippinesAppStore,
    #[serde(rename = "Poland Android App")]
    PolandAndroidApp,
    #[serde(rename = "Poland App Store")]
    PolandAppStore,
    #[serde(rename = "Portugal Android App")]
    PortugalAndroidApp,
    #[serde(rename = "Portugal App Store")]
    PortugalAppStore,
    #[serde(rename = "Romania Android App")]
    RomaniaAndroidApp,
    #[serde(rename = "Romania App Store")]
    RomaniaAppStore,
    #[serde(rename = "Russian Federation Android App")]
    RussianFederationAndroidApp,
    #[serde(rename = "Russian Federation App Store")]
    RussianFederationAppStore,
    #[serde(rename = "Saudi Arabia Android App")]
    SaudiArabiaAndroidApp,
    #[serde(rename = "Serbia Android App")]
    SerbiaAndroidApp,
    #[serde(rename = "Singapore Android App")]
    SingaporeAndroidApp,
    #[serde(rename = "Singapore App Store")]
    SingaporeAppStore,
    #[serde(rename = "Slovakia Android App")]
    SlovakiaAndroidApp,
    #[serde(rename = "Slovakia App Store")]
    SlovakiaAppStore,
    #[serde(rename = "Slovenia Android App")]
    SloveniaAndroidApp,
    #[serde(rename = "Somalia Android App")]
    SomaliaAndroidApp,
    #[serde(rename = "South Africa Android App")]
    SouthAfricaAndroidApp,
    #[serde(rename = "South Africa App Store")]
    SouthAfricaAppStore,
    #[serde(rename = "South America Android App")]
    SouthAmericaAndroidApp,
    #[serde(rename = "South Asia Android App")]
    SouthAsiaAndroidApp,
    #[serde(rename = "Spain Android App")]
    SpainAndroidApp,
    #[serde(rename = "Spain App Store")]
    SpainAppStore,
    #[serde(rename = "Sweden Android App")]
    SwedenAndroidApp,
    #[serde(rename = "Sweden App Store")]
    SwedenAppStore,
    #[serde(rename = "Switzerland Android App")]
    SwitzerlandAndroidApp,
    #[serde(rename = "Switzerland App Store")]
    SwitzerlandAppStore,
    #[serde(rename = "Syrian Arab Republic Android App")]
    SyrianArabRepublicAndroidApp,
    #[serde(rename = "Thailand Android App")]
    ThailandAndroidApp,
    #[serde(rename = "Thailand App Store")]
    ThailandAppStore,
    #[serde(rename = "Togo Android App")]
    TogoAndroidApp,
    #[serde(rename = "Tunisia Android App")]
    TunisiaAndroidApp,
    #[serde(rename = "Turkey Android App")]
    TurkeyAndroidApp,
    #[serde(rename = "Turkey App Store")]
    TurkeyAppStore,
    #[serde(rename = "Uganda App Store")]
    UgandaAppStore,
    #[serde(rename = "Ukraine Android App")]
    UkraineAndroidApp,
    #[serde(rename = "United Arab Emirates Android App")]
    UnitedArabEmiratesAndroidApp,
    #[serde(rename = "United Arab Emirates App Store")]
    UnitedArabEmiratesAppStore,
    #[serde(rename = "United Kingdom Android App")]
    UnitedKingdomAndroidApp,
    #[serde(rename = "United Kingdom App Store")]
    UnitedKingdomAppStore,
    #[serde(rename = "United States Android App")]
    UnitedStatesAndroidApp,
    #[serde(rename = "United States App Store")]
    UnitedStatesAppStore,
    #[serde(rename = "Uruguay Android App")]
    UruguayAndroidApp,
    #[serde(rename = "Uzbekistan App Store")]
    UzbekistanAppStore,
    #[serde(rename = "Viet Nam Android App")]
    VietNamAndroidApp,
    #[serde(rename = "Web")]
    Web,
    #[serde(rename = "West Asia Android App")]
    WestAsiaAndroidApp,
    #[serde(rename = "West Asia App Store")]
    WestAsiaAppStore,
}

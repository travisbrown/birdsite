#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(deny_unknown_fields)]
pub enum Source {
    #[serde(rename = "Africa Android App")]
    AfricaAndroidApp,
    #[serde(rename = "Africa App Store")]
    AfricaAppStore,
    #[serde(rename = "Afghanistan Android App")]
    AfghanistanAndroidApp,
    #[serde(rename = "Afghanistan App Store")]
    AfghanistanAppStore,
    #[serde(rename = "Albania Android App")]
    AlbaniaAndroidApp,
    #[serde(rename = "Albania App Store")]
    AlbaniaAppStore,
    #[serde(rename = "Algeria Android App")]
    AlgeriaAndroidApp,
    #[serde(rename = "Algeria App Store")]
    AlgeriaAppStore,
    #[serde(rename = "Angola Android App")]
    AngolaAndroidApp,
    #[serde(rename = "Angola App Store")]
    AngolaAppStore,
    #[serde(rename = "Antigua and Barbuda App Store")]
    AntiguaAndBarbudaAppStore,
    #[serde(rename = "Argentina Android App")]
    ArgentinaAndroidApp,
    #[serde(rename = "Argentina App Store")]
    ArgentinaAppStore,
    #[serde(rename = "Armenia App Store")]
    ArmeniaAppStore,
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
    #[serde(rename = "Azerbaijan Android App")]
    AzerbaijanAndroidApp,
    #[serde(rename = "Azerbaijan App Store")]
    AzerbaijanAppStore,
    #[serde(rename = "Bahamas Android App")]
    BahamasAndroidApp,
    #[serde(rename = "Bahamas App Store")]
    BahamasAppStore,
    #[serde(rename = "Bahrain Android App")]
    BahrainAndroidApp,
    #[serde(rename = "Bahrain App Store")]
    BahrainAppStore,
    #[serde(rename = "Bangladesh Android App")]
    BangladeshAndroidApp,
    #[serde(rename = "Belarus Android App")]
    BelarusAndroidApp,
    #[serde(rename = "Belarus App Store")]
    BelarusAppStore,
    #[serde(rename = "Belgium Android App")]
    BelgiumAndroidApp,
    #[serde(rename = "Belgium App Store")]
    BelgiumAppStore,
    #[serde(rename = "Belize Android App")]
    BelizeAndroidApp,
    #[serde(rename = "Benin Android App")]
    BeninAndroidApp,
    #[serde(rename = "Benin App Store")]
    BeninAppStore,
    #[serde(rename = "Bermuda App Store")]
    BermudaAppStore,
    #[serde(rename = "Bhutan Android App")]
    BhutanAndroidApp,
    #[serde(rename = "Bolivia Android App")]
    BoliviaAndroidApp,
    #[serde(rename = "Bolivia App Store")]
    BoliviaAppStore,
    #[serde(rename = "Bosnia and Herzegovina Android App")]
    BosniaAndHerzegovinaAndroidApp,
    #[serde(rename = "Bosnia and Herzegovina App Store")]
    BosniaAndHerzegovinaAppStore,
    #[serde(rename = "Botswana Android App")]
    BotswanaAndroidApp,
    #[serde(rename = "Botswana App Store")]
    BotswanaAppStore,
    #[serde(rename = "Brazil Android App")]
    BrazilAndroidApp,
    #[serde(rename = "Brazil App Store")]
    BrazilAppStore,
    #[serde(rename = "British Virgin Islands App Store")]
    BritishVirginIslandsAppStore,
    #[serde(rename = "Brunei Darussalam Android App")]
    BruneiDarussalamAndroidApp,
    #[serde(rename = "Bulgaria Android App")]
    BulgariaAndroidApp,
    #[serde(rename = "Bulgaria App Store")]
    BulgariaAppStore,
    #[serde(rename = "Burkina Faso Android App")]
    BurkinaFasoAndroidApp,
    #[serde(rename = "Burkina Faso App Store")]
    BurkinaFasoAppStore,
    #[serde(rename = "Burundi Android App")]
    BurundiAndroidApp,
    #[serde(rename = "Cambodia Android App")]
    CambodiaAndroidApp,
    #[serde(rename = "Cambodia App Store")]
    CambodiaAppStore,
    #[serde(rename = "Cameroon Android App")]
    CameroonAndroidApp,
    #[serde(rename = "Cameroon App Store")]
    CameroonAppStore,
    #[serde(rename = "Canada Android App")]
    CanadaAndroidApp,
    #[serde(rename = "Canada App Store")]
    CanadaAppStore,
    #[serde(rename = "Cape Verde Android App")]
    CapeVerdeAndroidApp,
    #[serde(rename = "Caribbean Android App")]
    CaribbeanAndroidApp,
    #[serde(rename = "Caribbean App Store")]
    CaribbeanAppStore,
    #[serde(rename = "Cayman Islands Android App")]
    CaymanIslandsAndroidApp,
    #[serde(rename = "Central African Republic Android App")]
    CentralAfricanRepublicAndroidApp,
    #[serde(rename = "Chad Android App")]
    ChadAndroidApp,
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
    #[serde(rename = "Congo Android App")]
    CongoAndroidApp,
    #[serde(rename = "Congo App Store")]
    CongoAppStore,
    #[serde(rename = "Costa Rica Android App")]
    CostaRicaAndroidApp,
    #[serde(rename = "Costa Rica App Store")]
    CostaRicaAppStore,
    #[serde(rename = "Côte d'Ivoire Android App")]
    CoteDIvoireAndroidApp,
    #[serde(rename = "Côte d'Ivoire App Store")]
    CoteDIvoireAppStore,
    #[serde(rename = "Croatia Android App")]
    CroatiaAndroidApp,
    #[serde(rename = "Croatia App Store")]
    CroatiaAppStore,
    #[serde(rename = "Cuba Android App")]
    CubaAndroidApp,
    #[serde(rename = "Curaçao Android App")]
    CuracaoAndroidApp,
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
    #[serde(rename = "Djibouti Android App")]
    DjiboutiAndroidApp,
    #[serde(rename = "Dominica Android App")]
    DominicaAndroidApp,
    #[serde(rename = "Dominican Republic Android App")]
    DominicanRepublicAndroidApp,
    #[serde(rename = "Dominican Republic App Store")]
    DominicanRepublicAppStore,
    #[serde(rename = "East Asia & Pacific Android App")]
    EastAsiaPacificAndroidApp,
    #[serde(rename = "East Asia & Pacific App Store")]
    EastAsiaPacificAppStore,
    #[serde(rename = "Eastern Europe (Non-EU) Android App")]
    EasternEuropeNonEuAndroidApp,
    #[serde(rename = "Eastern Europe (Non-EU) App Store")]
    EasternEuropeNonEuAppStore,
    #[serde(rename = "Ecuador Android App")]
    EcuadorAndroidApp,
    #[serde(rename = "Ecuador App Store")]
    EcuadorAppStore,
    #[serde(rename = "Egypt Android App")]
    EgyptAndroidApp,
    #[serde(rename = "Egypt App Store")]
    EgyptAppStore,
    #[serde(rename = "El Salvador Android App")]
    ElSalvadorAndroidApp,
    #[serde(rename = "El Salvador App Store")]
    ElSalvadorAppStore,
    #[serde(rename = "Eritrea Android App")]
    EritreaAndroidApp,
    #[serde(rename = "Estonia Android App")]
    EstoniaAndroidApp,
    #[serde(rename = "Estonia App Store")]
    EstoniaAppStore,
    #[serde(rename = "Ethiopia Android App")]
    EthiopiaAndroidApp,
    #[serde(rename = "Europe Android App")]
    EuropeAndroidApp,
    #[serde(rename = "Europe App Store")]
    EuropeAppStore,
    #[serde(rename = "Fiji Android App")]
    FijiAndroidApp,
    #[serde(rename = "Fiji App Store")]
    FijiAppStore,
    #[serde(rename = "Finland Android App")]
    FinlandAndroidApp,
    #[serde(rename = "Finland App Store")]
    FinlandAppStore,
    #[serde(rename = "France Android App")]
    FranceAndroidApp,
    #[serde(rename = "France App Store")]
    FranceAppStore,
    #[serde(rename = "Gabon Android App")]
    GabonAndroidApp,
    #[serde(rename = "Gabon App Store")]
    GabonAppStore,
    #[serde(rename = "Gambia Android App")]
    GambiaAndroidApp,
    #[serde(rename = "Gambia App Store")]
    GambiaAppStore,
    #[serde(rename = "Georgia Android App")]
    GeorgiaAndroidApp,
    #[serde(rename = "Georgia App Store")]
    GeorgiaAppStore,
    #[serde(rename = "Germany Android App")]
    GermanyAndroidApp,
    #[serde(rename = "Germany App Store")]
    GermanyAppStore,
    #[serde(rename = "Ghana Android App")]
    GhanaAndroidApp,
    #[serde(rename = "Ghana App Store")]
    GhanaAppStore,
    #[serde(rename = "Greece Android App")]
    GreeceAndroidApp,
    #[serde(rename = "Greece App Store")]
    GreeceAppStore,
    #[serde(rename = "Guadeloupe Android App")]
    GuadeloupeAndroidApp,
    #[serde(rename = "Guam Android App")]
    GuamAndroidApp,
    #[serde(rename = "Guatemala Android App")]
    GuatemalaAndroidApp,
    #[serde(rename = "Guatemala App Store")]
    GuatemalaAppStore,
    #[serde(rename = "Guinea Android App")]
    GuineaAndroidApp,
    #[serde(rename = "Guinea-Bissau App Store")]
    GuineaBissauAppStore,
    #[serde(rename = "Haiti Android App")]
    HaitiAndroidApp,
    #[serde(rename = "Honduras Android App")]
    HondurasAndroidApp,
    #[serde(rename = "Honduras App Store")]
    HondurasAppStore,
    #[serde(rename = "Hong Kong Android App")]
    HongKongAndroidApp,
    #[serde(rename = "Hong Kong App Store")]
    HongKongAppStore,
    #[serde(rename = "Hungary Android App")]
    HungaryAndroidApp,
    #[serde(rename = "Hungary App Store")]
    HungaryAppStore,
    #[serde(rename = "Iceland Android App")]
    IcelandAndroidApp,
    #[serde(rename = "Iceland App Store")]
    IcelandAppStore,
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
    #[serde(rename = "Iraq App Store")]
    IraqAppStore,
    #[serde(rename = "Ireland Android App")]
    IrelandAndroidApp,
    #[serde(rename = "Ireland App Store")]
    IrelandAppStore,
    #[serde(rename = "Isle of Man Android App")]
    IsleOfManAndroidApp,
    #[serde(rename = "Israel Android App")]
    IsraelAndroidApp,
    #[serde(rename = "Israel App Store")]
    IsraelAppStore,
    #[serde(rename = "Italy Android App")]
    ItalyAndroidApp,
    #[serde(rename = "Italy App Store")]
    ItalyAppStore,
    #[serde(rename = "Jamaica Android App")]
    JamaicaAndroidApp,
    #[serde(rename = "Jamaica App Store")]
    JamaicaAppStore,
    #[serde(rename = "Japan Android App")]
    JapanAndroidApp,
    #[serde(rename = "Japan App Store")]
    JapanAppStore,
    #[serde(rename = "Jersey Android App")]
    JerseyAndroidApp,
    #[serde(rename = "Jordan Android App")]
    JordanAndroidApp,
    #[serde(rename = "Jordan App Store")]
    JordanAppStore,
    #[serde(rename = "Kazakhstan Android App")]
    KazakhstanAndroidApp,
    #[serde(rename = "Kazakhstan App Store")]
    KazakhstanAppStore,
    #[serde(rename = "Kenya Android App")]
    KenyaAndroidApp,
    #[serde(rename = "Kenya App Store")]
    KenyaAppStore,
    #[serde(rename = "Kiribati Android App")]
    KiribatiAndroidApp,
    #[serde(rename = "Korea Android App")]
    KoreaAndroidApp,
    #[serde(rename = "Korea App Store")]
    KoreaAppStore,
    #[serde(rename = "Kosovo Android App")]
    KosovoAndroidApp,
    #[serde(rename = "Kuwait Android App")]
    KuwaitAndroidApp,
    #[serde(rename = "Kuwait App Store")]
    KuwaitAppStore,
    #[serde(rename = "Kyrgyzstan App Store")]
    KyrgyzstanAppStore,
    #[serde(rename = "Lao People's Democratic Republic Android App")]
    LaoPeoplesDemocraticRepublicAndroidApp,
    #[serde(rename = "Lao People's Democratic Republic App Store")]
    LaoPeoplesDemocraticRepublicAppStore,
    #[serde(rename = "Latvia Android App")]
    LatviaAndroidApp,
    #[serde(rename = "Latvia App Store")]
    LatviaAppStore,
    #[serde(rename = "Lebanon Android App")]
    LebanonAndroidApp,
    #[serde(rename = "Lebanon App Store")]
    LebanonAppStore,
    #[serde(rename = "Lesotho Android App")]
    LesothoAndroidApp,
    #[serde(rename = "Liberia Android App")]
    LiberiaAndroidApp,
    #[serde(rename = "Liberia App Store")]
    LiberiaAppStore,
    #[serde(rename = "Libya Android App")]
    LibyaAndroidApp,
    #[serde(rename = "Libya App Store")]
    LibyaAppStore,
    #[serde(rename = "Lithuania Android App")]
    LithuaniaAndroidApp,
    #[serde(rename = "Lithuania App Store")]
    LithuaniaAppStore,
    #[serde(rename = "Luxembourg Android App")]
    LuxembourgAndroidApp,
    #[serde(rename = "Luxembourg App Store")]
    LuxembourgAppStore,
    #[serde(rename = "Macao App Store")]
    MacaoAppStore,
    #[serde(rename = "Macedonia Android App")]
    MacedoniaAndroidApp,
    #[serde(rename = "Macedonia App Store")]
    MacedoniaAppStore,
    #[serde(rename = "Madagascar Android App")]
    MadagascarAndroidApp,
    #[serde(rename = "Malawi Android App")]
    MalawiAndroidApp,
    #[serde(rename = "Malaysia Android App")]
    MalaysiaAndroidApp,
    #[serde(rename = "Malaysia App Store")]
    MalaysiaAppStore,
    #[serde(rename = "Maldives Android App")]
    MaldivesAndroidApp,
    #[serde(rename = "Maldives App Store")]
    MaldivesAppStore,
    #[serde(rename = "Mali Android App")]
    MaliAndroidApp,
    #[serde(rename = "Mali App Store")]
    MaliAppStore,
    #[serde(rename = "Malta Android App")]
    MaltaAndroidApp,
    #[serde(rename = "Malta App Store")]
    MaltaAppStore,
    #[serde(rename = "Mauritania Android App")]
    MauritaniaAndroidApp,
    #[serde(rename = "Mauritania App Store")]
    MauritaniaAppStore,
    #[serde(rename = "Mauritius Android App")]
    MauritiusAndroidApp,
    #[serde(rename = "Mexico Android App")]
    MexicoAndroidApp,
    #[serde(rename = "Mexico App Store")]
    MexicoAppStore,
    #[serde(rename = "Moldova Android App")]
    MoldovaAndroidApp,
    #[serde(rename = "Moldova App Store")]
    MoldovaAppStore,
    #[serde(rename = "Mongolia Android App")]
    MongoliaAndroidApp,
    #[serde(rename = "Mongolia App Store")]
    MongoliaAppStore,
    #[serde(rename = "Montenegro Android App")]
    MontenegroAndroidApp,
    #[serde(rename = "Montenegro App Store")]
    MontenegroAppStore,
    #[serde(rename = "Morocco Android App")]
    MoroccoAndroidApp,
    #[serde(rename = "Morocco App Store")]
    MoroccoAppStore,
    #[serde(rename = "Mozambique Android App")]
    MozambiqueAndroidApp,
    #[serde(rename = "Mozambique App Store")]
    MozambiqueAppStore,
    #[serde(rename = "Myanmar Android App")]
    MyanmarAndroidApp,
    #[serde(rename = "Myanmar App Store")]
    MyanmarAppStore,
    #[serde(rename = "Namibia Android App")]
    NamibiaAndroidApp,
    #[serde(rename = "Namibia App Store")]
    NamibiaAppStore,
    #[serde(rename = "Nepal Android App")]
    NepalAndroidApp,
    #[serde(rename = "Nepal App Store")]
    NepalAppStore,
    #[serde(rename = "Netherlands Android App")]
    NetherlandsAndroidApp,
    #[serde(rename = "Netherlands App Store")]
    NetherlandsAppStore,
    #[serde(rename = "New Caledonia Android App")]
    NewCaledoniaAndroidApp,
    #[serde(rename = "New Zealand Android App")]
    NewZealandAndroidApp,
    #[serde(rename = "New Zealand App Store")]
    NewZealandAppStore,
    #[serde(rename = "Nicaragua Android App")]
    NicaraguaAndroidApp,
    #[serde(rename = "Nicaragua App Store")]
    NicaraguaAppStore,
    #[serde(rename = "Niger Android App")]
    NigerAndroidApp,
    #[serde(rename = "Niger App Store")]
    NigerAppStore,
    #[serde(rename = "Nigeria Android App")]
    NigeriaAndroidApp,
    #[serde(rename = "Nigeria App Store")]
    NigeriaAppStore,
    #[serde(rename = "North Africa Android App")]
    NorthAfricaAndroidApp,
    #[serde(rename = "North Africa App Store")]
    NorthAfricaAppStore,
    #[serde(rename = "North America Android App")]
    NorthAmericaAndroidApp,
    #[serde(rename = "North America App Store")]
    NorthAmericaAppStore,
    #[serde(rename = "Norway Android App")]
    NorwayAndroidApp,
    #[serde(rename = "Norway App Store")]
    NorwayAppStore,
    #[serde(rename = "Oman Android App")]
    OmanAndroidApp,
    #[serde(rename = "Oman App Store")]
    OmanAppStore,
    #[serde(rename = "Pakistan Android App")]
    PakistanAndroidApp,
    #[serde(rename = "Pakistan App Store")]
    PakistanAppStore,
    #[serde(rename = "Palau App Store")]
    PalauAppStore,
    #[serde(rename = "Panama Android App")]
    PanamaAndroidApp,
    #[serde(rename = "Panama App Store")]
    PanamaAppStore,
    #[serde(rename = "Papua New Guinea Android App")]
    PapuaNewGuineaAndroidApp,
    #[serde(rename = "Papua New Guinea App Store")]
    PapuaNewGuineaAppStore,
    #[serde(rename = "Paraguay Android App")]
    ParaguayAndroidApp,
    #[serde(rename = "Paraguay App Store")]
    ParaguayAppStore,
    #[serde(rename = "Peru Android App")]
    PeruAndroidApp,
    #[serde(rename = "Peru App Store")]
    PeruAppStore,
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
    #[serde(rename = "Puerto Rico Android App")]
    PuertoRicoAndroidApp,
    #[serde(rename = "Qatar Android App")]
    QatarAndroidApp,
    #[serde(rename = "Qatar App Store")]
    QatarAppStore,
    #[serde(rename = "Réunion Android App")]
    ReunionAndroidApp,
    #[serde(rename = "Romania Android App")]
    RomaniaAndroidApp,
    #[serde(rename = "Romania App Store")]
    RomaniaAppStore,
    #[serde(rename = "Russian Federation Android App")]
    RussianFederationAndroidApp,
    #[serde(rename = "Russian Federation App Store")]
    RussianFederationAppStore,
    #[serde(rename = "Rwanda Android App")]
    RwandaAndroidApp,
    #[serde(rename = "Rwanda App Store")]
    RwandaAppStore,
    #[serde(rename = "Saint Vincent and the Grenadines Android App")]
    SaintVincentAndTheGrenadinesAndroidApp,
    #[serde(rename = "Saint Vincent and the Grenadines App Store")]
    SaintVincentAndTheGrenadinesAppStore,
    #[serde(rename = "Saudi Arabia Android App")]
    SaudiArabiaAndroidApp,
    #[serde(rename = "Saudi Arabia App Store")]
    SaudiArabiaAppStore,
    #[serde(rename = "Senegal Android App")]
    SenegalAndroidApp,
    #[serde(rename = "Senegal App Store")]
    SenegalAppStore,
    #[serde(rename = "Serbia Android App")]
    SerbiaAndroidApp,
    #[serde(rename = "Serbia App Store")]
    SerbiaAppStore,
    #[serde(rename = "Seychelles Android App")]
    SeychellesAndroidApp,
    #[serde(rename = "Sierra Leone Android App")]
    SierraLeoneAndroidApp,
    #[serde(rename = "Sierra Leone App Store")]
    SierraLeoneAppStore,
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
    #[serde(rename = "Slovenia App Store")]
    SloveniaAppStore,
    #[serde(rename = "Solomon Islands Android App")]
    SolomonIslandsAndroidApp,
    #[serde(rename = "Somalia Android App")]
    SomaliaAndroidApp,
    #[serde(rename = "South Africa Android App")]
    SouthAfricaAndroidApp,
    #[serde(rename = "South Africa App Store")]
    SouthAfricaAppStore,
    #[serde(rename = "South America Android App")]
    SouthAmericaAndroidApp,
    #[serde(rename = "South America App Store")]
    SouthAmericaAppStore,
    #[serde(rename = "South Asia Android App")]
    SouthAsiaAndroidApp,
    #[serde(rename = "South Asia App Store")]
    SouthAsiaAppStore,
    #[serde(rename = "South Sudan Android App")]
    SouthSudanAndroidApp,
    #[serde(rename = "Spain Android App")]
    SpainAndroidApp,
    #[serde(rename = "Spain App Store")]
    SpainAppStore,
    #[serde(rename = "Sri Lanka Android App")]
    SriLankaAndroidApp,
    #[serde(rename = "Sri Lanka App Store")]
    SriLankaAppStore,
    #[serde(rename = "Sudan Android App")]
    SudanAndroidApp,
    #[serde(rename = "Suriname Android App")]
    SurinameAndroidApp,
    #[serde(rename = "Suriname App Store")]
    SurinameAppStore,
    #[serde(rename = "Swaziland Android App")]
    SwazilandAndroidApp,
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
    #[serde(rename = "Taiwan Android App")]
    TaiwanAndroidApp,
    #[serde(rename = "Taiwan App Store")]
    TaiwanAppStore,
    #[serde(rename = "Tanzania Android App")]
    TanzaniaAndroidApp,
    #[serde(rename = "Tanzania App Store")]
    TanzaniaAppStore,
    #[serde(rename = "Thailand Android App")]
    ThailandAndroidApp,
    #[serde(rename = "Thailand App Store")]
    ThailandAppStore,
    #[serde(rename = "Togo Android App")]
    TogoAndroidApp,
    #[serde(rename = "Trinidad and Tobago Android App")]
    TrinidadAndTobagoAndroidApp,
    #[serde(rename = "Trinidad and Tobago App Store")]
    TrinidadAndTobagoAppStore,
    #[serde(rename = "Tunisia Android App")]
    TunisiaAndroidApp,
    #[serde(rename = "Tunisia App Store")]
    TunisiaAppStore,
    #[serde(rename = "Turkey Android App")]
    TurkeyAndroidApp,
    #[serde(rename = "Turkey App Store")]
    TurkeyAppStore,
    #[serde(rename = "Turkmenistan Android App")]
    TurkmenistanAndroidApp,
    #[serde(rename = "Turkmenistan App Store")]
    TurkmenistanAppStore,
    #[serde(rename = "Turks and Caicos Islands App Store")]
    TurksAndCaicosIslandsAppStore,
    #[serde(rename = "Uganda Android App")]
    UgandaAndroidApp,
    #[serde(rename = "Uganda App Store")]
    UgandaAppStore,
    #[serde(rename = "Ukraine Android App")]
    UkraineAndroidApp,
    #[serde(rename = "Ukraine App Store")]
    UkraineAppStore,
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
    #[serde(rename = "Uruguay App Store")]
    UruguayAppStore,
    #[serde(rename = "Uzbekistan Android App")]
    UzbekistanAndroidApp,
    #[serde(rename = "Uzbekistan App Store")]
    UzbekistanAppStore,
    #[serde(rename = "Vanuatu Android App")]
    VanuatuAndroidApp,
    #[serde(rename = "Venezuela Android App")]
    VenezuelaAndroidApp,
    #[serde(rename = "Venezuela App Store")]
    VenezuelaAppStore,
    #[serde(rename = "Viet Nam Android App")]
    VietNamAndroidApp,
    #[serde(rename = "Viet Nam App Store")]
    VietNamAppStore,
    #[serde(rename = "Web")]
    Web,
    #[serde(rename = "West Asia Android App")]
    WestAsiaAndroidApp,
    #[serde(rename = "West Asia App Store")]
    WestAsiaAppStore,
    #[serde(rename = "Yemen Android App")]
    YemenAndroidApp,
    #[serde(rename = "Yemen App Store")]
    YemenAppStore,
    #[serde(rename = "Zambia Android App")]
    ZambiaAndroidApp,
    #[serde(rename = "Zambia App Store")]
    ZambiaAppStore,
    #[serde(rename = "Zimbabwe Android App")]
    ZimbabweAndroidApp,
    #[serde(rename = "Zimbabwe App Store")]
    ZimbabweAppStore,
}

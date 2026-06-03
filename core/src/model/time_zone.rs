//! Twitter user time zone labels.
//!
//! Historical Twitter user objects carry a free-form `time_zone` string drawn
//! from several overlapping families of labels:
//!
//! - **IANA identifiers** such as `"Asia/Tokyo"` ([`IanaTimeZone`]).
//! - **Deprecated IANA aliases** such as `"Asia/Calcutta"` ([`Deprecated`]).
//! - **Abbreviations** such as `"JST"` or `"GMT+9"` ([`Abbreviation`]).
//! - **Display names** such as `"Tokyo"` or `"Central Time (US & Canada)"` ([`Named`]).
//!
//! [`TimeZone`] is the wire type and preserves whichever label was used, so it
//! round-trips exactly. Every label additionally resolves to a canonical
//! [`IanaTimeZone`] via [`TimeZone::canonical`], so two different labels for the same
//! zone compare equal under [`TimeZone::same_zone`].
//!
//! [`TIME_ZONE_VALUES`] enumerates every label, ordered by its string form, which
//! is also the order imposed by [`TimeZone`]'s [`Ord`] implementation.
use std::sync::LazyLock;

/// A time zone label as it appears in a Twitter user object.
///
/// Preserves the original label family so serialization round-trips exactly.
/// Use [`canonical`](Self::canonical) or [`same_zone`](Self::same_zone) to compare
/// labels for zone equivalence. Ordering is by the string label ([`as_str`](Self::as_str)).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum TimeZone {
    /// A canonical IANA identifier such as `Asia/Tokyo`.
    Iana(IanaTimeZone),
    /// A deprecated IANA alias such as `Asia/Calcutta`.
    Deprecated(Deprecated),
    /// An abbreviation or UTC offset such as `JST` or `GMT+9`.
    Abbreviation(Abbreviation),
    /// A display name such as `Tokyo`.
    Named(Named),
}

impl TimeZone {
    /// The exact wire label for this time zone.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Iana(zone) => zone.as_str(),
            Self::Deprecated(deprecated) => deprecated.as_str(),
            Self::Abbreviation(abbreviation) => abbreviation.as_str(),
            Self::Named(named) => named.as_str(),
        }
    }

    /// Resolves this label to its canonical [`IanaTimeZone`].
    #[must_use]
    pub const fn canonical(self) -> IanaTimeZone {
        match self {
            Self::Iana(zone) => zone,
            Self::Deprecated(deprecated) => deprecated.canonical(),
            Self::Abbreviation(abbreviation) => abbreviation.canonical(),
            Self::Named(named) => named.canonical(),
        }
    }

    /// Returns `true` if both labels denote the same canonical zone.
    #[must_use]
    pub fn same_zone(self, other: Self) -> bool {
        self.canonical() == other.canonical()
    }

    /// Parses a wire `time_zone` label, returning `None` if it matches no known
    /// IANA identifier, deprecated alias, abbreviation, or display name.
    ///
    /// [`TIME_ZONE_VALUES`] is sorted by [`as_str`](Self::as_str), so this is a
    /// binary search over every known label.
    #[must_use]
    pub fn parse(label: &str) -> Option<Self> {
        TIME_ZONE_VALUES
            .binary_search_by(|zone| zone.as_str().cmp(label))
            .ok()
            .map(|index| TIME_ZONE_VALUES[index])
    }
}

impl<'de> serde::Deserialize<'de> for TimeZone {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Borrow when possible, fall back to an owned buffer for escaped input,
        // so the unknown value can be surfaced in the error either way.
        let label: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;

        Self::parse(&label).ok_or_else(|| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&label),
                &"a known Twitter time zone label",
            )
        })
    }
}

impl PartialOrd for TimeZone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeZone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl std::fmt::Display for TimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Every [`TimeZone`] label, sorted by its string representation.
pub static TIME_ZONE_VALUES: LazyLock<Vec<TimeZone>> = LazyLock::new(|| {
    let mut values = Vec::with_capacity(
        IanaTimeZone::ALL.len()
            + Deprecated::ALL.len()
            + Abbreviation::ALL.len()
            + Named::ALL.len(),
    );
    values.extend(IanaTimeZone::ALL.into_iter().map(TimeZone::Iana));
    values.extend(Deprecated::ALL.into_iter().map(TimeZone::Deprecated));
    values.extend(Abbreviation::ALL.into_iter().map(TimeZone::Abbreviation));
    values.extend(Named::ALL.into_iter().map(TimeZone::Named));
    values.sort_unstable();
    values
});

/// A canonical IANA time zone identifier, e.g. `Asia/Tokyo`.
///
/// Deprecated aliases that also appear in the data are modelled separately as
/// [`Deprecated`]; every value here is already canonical.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum IanaTimeZone {
    // Africa
    #[serde(rename = "Africa/Abidjan")]
    AfricaAbidjan,
    #[serde(rename = "Africa/Accra")]
    AfricaAccra,
    #[serde(rename = "Africa/Addis_Ababa")]
    AfricaAddisAbaba,
    #[serde(rename = "Africa/Algiers")]
    AfricaAlgiers,
    #[serde(rename = "Africa/Asmara")]
    AfricaAsmara,
    #[serde(rename = "Africa/Bamako")]
    AfricaBamako,
    #[serde(rename = "Africa/Bangui")]
    AfricaBangui,
    #[serde(rename = "Africa/Banjul")]
    AfricaBanjul,
    #[serde(rename = "Africa/Bissau")]
    AfricaBissau,
    #[serde(rename = "Africa/Blantyre")]
    AfricaBlantyre,
    #[serde(rename = "Africa/Bujumbura")]
    AfricaBujumbura,
    #[serde(rename = "Africa/Brazzaville")]
    AfricaBrazzaville,
    #[serde(rename = "Africa/Cairo")]
    AfricaCairo,
    #[serde(rename = "Africa/Casablanca")]
    AfricaCasablanca,
    #[serde(rename = "Africa/Ceuta")]
    AfricaCeuta,
    #[serde(rename = "Africa/Conakry")]
    AfricaConakry,
    #[serde(rename = "Africa/Dakar")]
    AfricaDakar,
    #[serde(rename = "Africa/Dar_es_Salaam")]
    AfricaDarEsSalaam,
    #[serde(rename = "Africa/Djibouti")]
    AfricaDjibouti,
    #[serde(rename = "Africa/Douala")]
    AfricaDouala,
    #[serde(rename = "Africa/Freetown")]
    AfricaFreetown,
    #[serde(rename = "Africa/Gaborone")]
    AfricaGaborone,
    #[serde(rename = "Africa/Harare")]
    AfricaHarare,
    #[serde(rename = "Africa/Johannesburg")]
    AfricaJohannesburg,
    #[serde(rename = "Africa/Juba")]
    AfricaJuba,
    #[serde(rename = "Africa/Kampala")]
    AfricaKampala,
    #[serde(rename = "Africa/Khartoum")]
    AfricaKhartoum,
    #[serde(rename = "Africa/Kigali")]
    AfricaKigali,
    #[serde(rename = "Africa/Kinshasa")]
    AfricaKinshasa,
    #[serde(rename = "Africa/Lagos")]
    AfricaLagos,
    #[serde(rename = "Africa/Libreville")]
    AfricaLibreville,
    #[serde(rename = "Africa/Lome")]
    AfricaLome,
    #[serde(rename = "Africa/Luanda")]
    AfricaLuanda,
    #[serde(rename = "Africa/Lubumbashi")]
    AfricaLubumbashi,
    #[serde(rename = "Africa/Lusaka")]
    AfricaLusaka,
    #[serde(rename = "Africa/Malabo")]
    AfricaMalabo,
    #[serde(rename = "Africa/Maputo")]
    AfricaMaputo,
    #[serde(rename = "Africa/Maseru")]
    AfricaMaseru,
    #[serde(rename = "Africa/Mbabane")]
    AfricaMbabane,
    #[serde(rename = "Africa/Mogadishu")]
    AfricaMogadishu,
    #[serde(rename = "Africa/Monrovia")]
    AfricaMonrovia,
    #[serde(rename = "Africa/Nairobi")]
    AfricaNairobi,
    #[serde(rename = "Africa/Ndjamena")]
    AfricaNdjamena,
    #[serde(rename = "Africa/Niamey")]
    AfricaNiamey,
    #[serde(rename = "Africa/Nouakchott")]
    AfricaNouakchott,
    #[serde(rename = "Africa/Ouagadougou")]
    AfricaOuagadougou,
    #[serde(rename = "Africa/Porto-Novo")]
    AfricaPortoNovo,
    #[serde(rename = "Africa/Tripoli")]
    AfricaTripoli,
    #[serde(rename = "Africa/Tunis")]
    AfricaTunis,
    #[serde(rename = "Africa/Windhoek")]
    AfricaWindhoek,

    // America
    #[serde(rename = "America/Anchorage")]
    AmericaAnchorage,
    #[serde(rename = "America/Adak")]
    AmericaAdak,
    #[serde(rename = "America/Anguilla")]
    AmericaAnguilla,
    #[serde(rename = "America/Araguaina")]
    AmericaAraguaina,
    #[serde(rename = "America/Antigua")]
    AmericaAntigua,
    #[serde(rename = "America/Argentina/Buenos_Aires")]
    AmericaArgentinaBuenosAires,
    #[serde(rename = "America/Argentina/Cordoba")]
    AmericaArgentinaCordoba,
    #[serde(rename = "America/Argentina/San_Juan")]
    AmericaArgentinaSanJuan,
    #[serde(rename = "America/Argentina/San_Luis")]
    AmericaArgentinaSanLuis,
    #[serde(rename = "America/Argentina/Ushuaia")]
    AmericaArgentinaUshuaia,
    #[serde(rename = "America/Aruba")]
    AmericaAruba,
    #[serde(rename = "America/Asuncion")]
    AmericaAsuncion,
    #[serde(rename = "America/Atikokan")]
    AmericaAtikokan,
    #[serde(rename = "America/Bahia")]
    AmericaBahia,
    #[serde(rename = "America/Bahia_Banderas")]
    AmericaBahiaBanderas,
    #[serde(rename = "America/Barbados")]
    AmericaBarbados,
    #[serde(rename = "America/Belem")]
    AmericaBelem,
    #[serde(rename = "America/Belize")]
    AmericaBelize,
    #[serde(rename = "America/Blanc-Sablon")]
    AmericaBlancSablon,
    #[serde(rename = "America/Bogota")]
    AmericaBogota,
    #[serde(rename = "America/Boa_Vista")]
    AmericaBoaVista,
    #[serde(rename = "America/Boise")]
    AmericaBoise,
    #[serde(rename = "America/Cambridge_Bay")]
    AmericaCambridgeBay,
    #[serde(rename = "America/Campo_Grande")]
    AmericaCampoGrande,
    #[serde(rename = "America/Cancun")]
    AmericaCancun,
    #[serde(rename = "America/Caracas")]
    AmericaCaracas,
    #[serde(rename = "America/Cayenne")]
    AmericaCayenne,
    #[serde(rename = "America/Cayman")]
    AmericaCayman,
    #[serde(rename = "America/Chicago")]
    AmericaChicago,
    #[serde(rename = "America/Chihuahua")]
    AmericaChihuahua,
    #[serde(rename = "America/Coral_Harbour")]
    AmericaCoralHarbour,
    #[serde(rename = "America/Cordoba")]
    AmericaCordoba,
    #[serde(rename = "America/Costa_Rica")]
    AmericaCostaRica,
    #[serde(rename = "America/Creston")]
    AmericaCreston,
    #[serde(rename = "America/Cuiaba")]
    AmericaCuiaba,
    #[serde(rename = "America/Curacao")]
    AmericaCuracao,
    #[serde(rename = "America/Dawson")]
    AmericaDawson,
    #[serde(rename = "America/Dawson_Creek")]
    AmericaDawsonCreek,
    #[serde(rename = "America/Denver")]
    AmericaDenver,
    #[serde(rename = "America/Detroit")]
    AmericaDetroit,
    #[serde(rename = "America/Dominica")]
    AmericaDominica,
    #[serde(rename = "America/Edmonton")]
    AmericaEdmonton,
    #[serde(rename = "America/Eirunepe")]
    AmericaEirunepe,
    #[serde(rename = "America/El_Salvador")]
    AmericaElSalvador,
    #[serde(rename = "America/Fortaleza")]
    AmericaFortaleza,
    #[serde(rename = "America/Glace_Bay")]
    AmericaGlaceBay,
    #[serde(rename = "America/Godthab")]
    AmericaGodthab,
    #[serde(rename = "America/Grand_Turk")]
    AmericaGrandTurk,
    #[serde(rename = "America/Grenada")]
    AmericaGrenada,
    #[serde(rename = "America/Guadeloupe")]
    AmericaGuadeloupe,
    #[serde(rename = "America/Guatemala")]
    AmericaGuatemala,
    #[serde(rename = "America/Guayaquil")]
    AmericaGuayaquil,
    #[serde(rename = "America/Guyana")]
    AmericaGuyana,
    #[serde(rename = "America/Halifax")]
    AmericaHalifax,
    #[serde(rename = "America/Havana")]
    AmericaHavana,
    #[serde(rename = "America/Hermosillo")]
    AmericaHermosillo,
    #[serde(rename = "America/Indiana/Indianapolis")]
    AmericaIndianaIndianapolis,
    #[serde(rename = "America/Indiana/Vincennes")]
    AmericaIndianaVincennes,
    #[serde(rename = "America/Inuvik")]
    AmericaInuvik,
    #[serde(rename = "America/Iqaluit")]
    AmericaIqaluit,
    #[serde(rename = "America/Jamaica")]
    AmericaJamaica,
    #[serde(rename = "America/Juneau")]
    AmericaJuneau,
    #[serde(rename = "America/Kentucky/Louisville")]
    AmericaKentuckyLouisville,
    #[serde(rename = "America/Kentucky/Monticello")]
    AmericaKentuckyMonticello,
    #[serde(rename = "America/La_Paz")]
    AmericaLaPaz,
    #[serde(rename = "America/Lima")]
    AmericaLima,
    #[serde(rename = "America/Los_Angeles")]
    AmericaLosAngeles,
    #[serde(rename = "America/Lower_Princes")]
    AmericaLowerPrinces,
    #[serde(rename = "America/Maceio")]
    AmericaMaceio,
    #[serde(rename = "America/Managua")]
    AmericaManagua,
    #[serde(rename = "America/Manaus")]
    AmericaManaus,
    #[serde(rename = "America/Matamoros")]
    AmericaMatamoros,
    #[serde(rename = "America/Mazatlan")]
    AmericaMazatlan,
    #[serde(rename = "America/Menominee")]
    AmericaMenominee,
    #[serde(rename = "America/Merida")]
    AmericaMerida,
    #[serde(rename = "America/Mexico_City")]
    AmericaMexicoCity,
    #[serde(rename = "America/Miquelon")]
    AmericaMiquelon,
    #[serde(rename = "America/Moncton")]
    AmericaMoncton,
    #[serde(rename = "America/Monterrey")]
    AmericaMonterrey,
    #[serde(rename = "America/Montevideo")]
    AmericaMontevideo,
    #[serde(rename = "America/Montreal")]
    AmericaMontreal,
    #[serde(rename = "America/Montserrat")]
    AmericaMontserrat,
    #[serde(rename = "America/Nassau")]
    AmericaNassau,
    #[serde(rename = "America/New_York")]
    AmericaNewYork,
    #[serde(rename = "America/Nipigon")]
    AmericaNipigon,
    #[serde(rename = "America/Noronha")]
    AmericaNoronha,
    #[serde(rename = "America/North_Dakota/Beulah")]
    AmericaNorthDakotaBeulah,
    #[serde(rename = "America/North_Dakota/Center")]
    AmericaNorthDakotaCenter,
    #[serde(rename = "America/Ojinaga")]
    AmericaOjinaga,
    #[serde(rename = "America/Panama")]
    AmericaPanama,
    #[serde(rename = "America/Paramaribo")]
    AmericaParamaribo,
    #[serde(rename = "America/Phoenix")]
    AmericaPhoenix,
    #[serde(rename = "America/Port-au-Prince")]
    AmericaPortAuPrince,
    #[serde(rename = "America/Port_of_Spain")]
    AmericaPortOfSpain,
    #[serde(rename = "America/Porto_Velho")]
    AmericaPortoVelho,
    #[serde(rename = "America/Puerto_Rico")]
    AmericaPuertoRico,
    #[serde(rename = "America/Recife")]
    AmericaRecife,
    #[serde(rename = "America/Regina")]
    AmericaRegina,
    #[serde(rename = "America/Resolute")]
    AmericaResolute,
    #[serde(rename = "America/Rio_Branco")]
    AmericaRioBranco,
    #[serde(rename = "America/Santiago")]
    AmericaSantiago,
    #[serde(rename = "America/Santo_Domingo")]
    AmericaSantoDomingo,
    #[serde(rename = "America/Sao_Paulo")]
    AmericaSaoPaulo,
    #[serde(rename = "America/Sitka")]
    AmericaSitka,
    #[serde(rename = "America/St_Johns")]
    AmericaStJohns,
    #[serde(rename = "America/St_Kitts")]
    AmericaStKitts,
    #[serde(rename = "America/St_Lucia")]
    AmericaStLucia,
    #[serde(rename = "America/St_Thomas")]
    AmericaStThomas,
    #[serde(rename = "America/St_Vincent")]
    AmericaStVincent,
    #[serde(rename = "America/Swift_Current")]
    AmericaSwiftCurrent,
    #[serde(rename = "America/Tegucigalpa")]
    AmericaTegucigalpa,
    #[serde(rename = "America/Thunder_Bay")]
    AmericaThunderBay,
    #[serde(rename = "America/Tijuana")]
    AmericaTijuana,
    #[serde(rename = "America/Toronto")]
    AmericaToronto,
    #[serde(rename = "America/Tortola")]
    AmericaTortola,
    #[serde(rename = "America/Vancouver")]
    AmericaVancouver,
    #[serde(rename = "America/Winnipeg")]
    AmericaWinnipeg,

    // Antarctica
    #[serde(rename = "Antarctica/Casey")]
    AntarcticaCasey,
    #[serde(rename = "Antarctica/Davis")]
    AntarcticaDavis,
    #[serde(rename = "Antarctica/DumontDUrville")]
    AntarcticaDumontDUrville,
    #[serde(rename = "Antarctica/Mawson")]
    AntarcticaMawson,
    #[serde(rename = "Antarctica/Palmer")]
    AntarcticaPalmer,
    #[serde(rename = "Antarctica/Syowa")]
    AntarcticaSyowa,
    #[serde(rename = "Antarctica/Vostok")]
    AntarcticaVostok,

    // Asia
    #[serde(rename = "Asia/Aden")]
    AsiaAden,
    #[serde(rename = "Asia/Almaty")]
    AsiaAlmaty,
    #[serde(rename = "Asia/Amman")]
    AsiaAmman,
    #[serde(rename = "Asia/Anadyr")]
    AsiaAnadyr,
    #[serde(rename = "Asia/Aqtau")]
    AsiaAqtau,
    #[serde(rename = "Asia/Ashgabat")]
    AsiaAshgabat,
    #[serde(rename = "Asia/Baghdad")]
    AsiaBaghdad,
    #[serde(rename = "Asia/Bahrain")]
    AsiaBahrain,
    #[serde(rename = "Asia/Baku")]
    AsiaBaku,
    #[serde(rename = "Asia/Bangkok")]
    AsiaBangkok,
    #[serde(rename = "Asia/Beirut")]
    AsiaBeirut,
    #[serde(rename = "Asia/Bishkek")]
    AsiaBishkek,
    #[serde(rename = "Asia/Brunei")]
    AsiaBrunei,
    #[serde(rename = "Asia/Choibalsan")]
    AsiaChoibalsan,
    #[serde(rename = "Asia/Chongqing")]
    AsiaChongqing,
    #[serde(rename = "Asia/Colombo")]
    AsiaColombo,
    #[serde(rename = "Asia/Damascus")]
    AsiaDamascus,
    #[serde(rename = "Asia/Dhaka")]
    AsiaDhaka,
    #[serde(rename = "Asia/Dili")]
    AsiaDili,
    #[serde(rename = "Asia/Dubai")]
    AsiaDubai,
    #[serde(rename = "Asia/Dushanbe")]
    AsiaDushanbe,
    #[serde(rename = "Asia/Gaza")]
    AsiaGaza,
    #[serde(rename = "Asia/Ho_Chi_Minh")]
    AsiaHoChiMinh,
    #[serde(rename = "Asia/Hong_Kong")]
    AsiaHongKong,
    #[serde(rename = "Asia/Irkutsk")]
    AsiaIrkutsk,
    #[serde(rename = "Asia/Istanbul")]
    AsiaIstanbul,
    #[serde(rename = "Asia/Jakarta")]
    AsiaJakarta,
    #[serde(rename = "Asia/Jayapura")]
    AsiaJayapura,
    #[serde(rename = "Asia/Jerusalem")]
    AsiaJerusalem,
    #[serde(rename = "Asia/Kabul")]
    AsiaKabul,
    #[serde(rename = "Asia/Kamchatka")]
    AsiaKamchatka,
    #[serde(rename = "Asia/Karachi")]
    AsiaKarachi,
    #[serde(rename = "Asia/Kathmandu")]
    AsiaKathmandu,
    #[serde(rename = "Asia/Kolkata")]
    AsiaKolkata,
    #[serde(rename = "Asia/Krasnoyarsk")]
    AsiaKrasnoyarsk,
    #[serde(rename = "Asia/Kuala_Lumpur")]
    AsiaKualaLumpur,
    #[serde(rename = "Asia/Kuching")]
    AsiaKuching,
    #[serde(rename = "Asia/Kuwait")]
    AsiaKuwait,
    #[serde(rename = "Asia/Macao")]
    AsiaMacao,
    #[serde(rename = "Asia/Macau")]
    AsiaMacau,
    #[serde(rename = "Asia/Magadan")]
    AsiaMagadan,
    #[serde(rename = "Asia/Makassar")]
    AsiaMakassar,
    #[serde(rename = "Asia/Manila")]
    AsiaManila,
    #[serde(rename = "Asia/Muscat")]
    AsiaMuscat,
    #[serde(rename = "Asia/Nicosia")]
    AsiaNicosia,
    #[serde(rename = "Asia/Novosibirsk")]
    AsiaNovosibirsk,
    #[serde(rename = "Asia/Omsk")]
    AsiaOmsk,
    #[serde(rename = "Asia/Oral")]
    AsiaOral,
    #[serde(rename = "Asia/Phnom_Penh")]
    AsiaPhnomPenh,
    #[serde(rename = "Asia/Pontianak")]
    AsiaPontianak,
    #[serde(rename = "Asia/Pyongyang")]
    AsiaPyongyang,
    #[serde(rename = "Asia/Qatar")]
    AsiaQatar,
    #[serde(rename = "Asia/Qyzylorda")]
    AsiaQyzylorda,
    #[serde(rename = "Asia/Rangoon")]
    AsiaRangoon,
    #[serde(rename = "Asia/Riyadh")]
    AsiaRiyadh,
    #[serde(rename = "Asia/Sakhalin")]
    AsiaSakhalin,
    #[serde(rename = "Asia/Samarkand")]
    AsiaSamarkand,
    #[serde(rename = "Asia/Seoul")]
    AsiaSeoul,
    #[serde(rename = "Asia/Shanghai")]
    AsiaShanghai,
    #[serde(rename = "Asia/Singapore")]
    AsiaSingapore,
    #[serde(rename = "Asia/Taipei")]
    AsiaTaipei,
    #[serde(rename = "Asia/Tashkent")]
    AsiaTashkent,
    #[serde(rename = "Asia/Tbilisi")]
    AsiaTbilisi,
    #[serde(rename = "Asia/Tehran")]
    AsiaTehran,
    #[serde(rename = "Asia/Thimphu")]
    AsiaThimphu,
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
    #[serde(rename = "Asia/Ulaanbaatar")]
    AsiaUlaanbaatar,
    #[serde(rename = "Asia/Urumqi")]
    AsiaUrumqi,
    #[serde(rename = "Asia/Vientiane")]
    AsiaVientiane,
    #[serde(rename = "Asia/Vladivostok")]
    AsiaVladivostok,
    #[serde(rename = "Asia/Yakutsk")]
    AsiaYakutsk,
    #[serde(rename = "Asia/Yekaterinburg")]
    AsiaYekaterinburg,
    #[serde(rename = "Asia/Yerevan")]
    AsiaYerevan,

    // Atlantic
    #[serde(rename = "Atlantic/Azores")]
    AtlanticAzores,
    #[serde(rename = "Atlantic/Bermuda")]
    AtlanticBermuda,
    #[serde(rename = "Atlantic/Cape_Verde")]
    AtlanticCapeVerde,
    #[serde(rename = "Atlantic/Canary")]
    AtlanticCanary,
    #[serde(rename = "Atlantic/Faroe")]
    AtlanticFaroe,
    #[serde(rename = "Atlantic/Madeira")]
    AtlanticMadeira,
    #[serde(rename = "Atlantic/Reykjavik")]
    AtlanticReykjavik,
    #[serde(rename = "Atlantic/South_Georgia")]
    AtlanticSouthGeorgia,
    #[serde(rename = "Atlantic/Stanley")]
    AtlanticStanley,

    // Australia
    #[serde(rename = "Australia/Adelaide")]
    AustraliaAdelaide,
    #[serde(rename = "Australia/Brisbane")]
    AustraliaBrisbane,
    #[serde(rename = "Australia/Currie")]
    AustraliaCurrie,
    #[serde(rename = "Australia/Darwin")]
    AustraliaDarwin,
    #[serde(rename = "Australia/Eucla")]
    AustraliaEucla,
    #[serde(rename = "Australia/Hobart")]
    AustraliaHobart,
    #[serde(rename = "Australia/Lord_Howe")]
    AustraliaLordHowe,
    #[serde(rename = "Australia/Melbourne")]
    AustraliaMelbourne,
    #[serde(rename = "Australia/Perth")]
    AustraliaPerth,
    #[serde(rename = "Australia/Sydney")]
    AustraliaSydney,

    // Etc
    #[serde(rename = "Etc/GMT")]
    EtcGmt,
    #[serde(rename = "Etc/GMT+10")]
    EtcGmtPlus10,
    #[serde(rename = "Etc/GMT+3")]
    EtcGmtPlus3,
    #[serde(rename = "Etc/GMT+4")]
    EtcGmtPlus4,
    #[serde(rename = "Etc/GMT+5")]
    EtcGmtPlus5,
    #[serde(rename = "Etc/GMT+6")]
    EtcGmtPlus6,
    #[serde(rename = "Etc/GMT-1")]
    EtcGmtMinus1,
    #[serde(rename = "Etc/GMT-10")]
    EtcGmtMinus10,
    #[serde(rename = "Etc/GMT-12")]
    EtcGmtMinus12,
    #[serde(rename = "Etc/GMT-2")]
    EtcGmtMinus2,
    #[serde(rename = "Etc/GMT-3")]
    EtcGmtMinus3,
    #[serde(rename = "Etc/GMT-4")]
    EtcGmtMinus4,
    #[serde(rename = "Etc/GMT-5")]
    EtcGmtMinus5,
    #[serde(rename = "Etc/GMT-8")]
    EtcGmtMinus8,
    #[serde(rename = "Etc/GMT-9")]
    EtcGmtMinus9,
    #[serde(rename = "Etc/UTC")]
    EtcUtc,

    // Europe
    #[serde(rename = "Europe/Amsterdam")]
    EuropeAmsterdam,
    #[serde(rename = "Europe/Andorra")]
    EuropeAndorra,
    #[serde(rename = "Europe/Athens")]
    EuropeAthens,
    #[serde(rename = "Europe/Belfast")]
    EuropeBelfast,
    #[serde(rename = "Europe/Belgrade")]
    EuropeBelgrade,
    #[serde(rename = "Europe/Berlin")]
    EuropeBerlin,
    #[serde(rename = "Europe/Bratislava")]
    EuropeBratislava,
    #[serde(rename = "Europe/Brussels")]
    EuropeBrussels,
    #[serde(rename = "Europe/Bucharest")]
    EuropeBucharest,
    #[serde(rename = "Europe/Budapest")]
    EuropeBudapest,
    #[serde(rename = "Europe/Chisinau")]
    EuropeChisinau,
    #[serde(rename = "Europe/Copenhagen")]
    EuropeCopenhagen,
    #[serde(rename = "Europe/Dublin")]
    EuropeDublin,
    #[serde(rename = "Europe/Gibraltar")]
    EuropeGibraltar,
    #[serde(rename = "Europe/Guernsey")]
    EuropeGuernsey,
    #[serde(rename = "Europe/Helsinki")]
    EuropeHelsinki,
    #[serde(rename = "Europe/Isle_of_Man")]
    EuropeIsleOfMan,
    #[serde(rename = "Europe/Istanbul")]
    EuropeIstanbul,
    #[serde(rename = "Europe/Jersey")]
    EuropeJersey,
    #[serde(rename = "Europe/Kaliningrad")]
    EuropeKaliningrad,
    #[serde(rename = "Europe/Kiev")]
    EuropeKiev,
    #[serde(rename = "Europe/Lisbon")]
    EuropeLisbon,
    #[serde(rename = "Europe/Ljubljana")]
    EuropeLjubljana,
    #[serde(rename = "Europe/London")]
    EuropeLondon,
    #[serde(rename = "Europe/Luxembourg")]
    EuropeLuxembourg,
    #[serde(rename = "Europe/Madrid")]
    EuropeMadrid,
    #[serde(rename = "Europe/Malta")]
    EuropeMalta,
    #[serde(rename = "Europe/Mariehamn")]
    EuropeMariehamn,
    #[serde(rename = "Europe/Minsk")]
    EuropeMinsk,
    #[serde(rename = "Europe/Monaco")]
    EuropeMonaco,
    #[serde(rename = "Europe/Moscow")]
    EuropeMoscow,
    #[serde(rename = "Europe/Oslo")]
    EuropeOslo,
    #[serde(rename = "Europe/Paris")]
    EuropeParis,
    #[serde(rename = "Europe/Podgorica")]
    EuropePodgorica,
    #[serde(rename = "Europe/Prague")]
    EuropePrague,
    #[serde(rename = "Europe/Riga")]
    EuropeRiga,
    #[serde(rename = "Europe/Rome")]
    EuropeRome,
    #[serde(rename = "Europe/San_Marino")]
    EuropeSanMarino,
    #[serde(rename = "Europe/Samara")]
    EuropeSamara,
    #[serde(rename = "Europe/Simferopol")]
    EuropeSimferopol,
    #[serde(rename = "Europe/Sarajevo")]
    EuropeSarajevo,
    #[serde(rename = "Europe/Skopje")]
    EuropeSkopje,
    #[serde(rename = "Europe/Sofia")]
    EuropeSofia,
    #[serde(rename = "Europe/Stockholm")]
    EuropeStockholm,
    #[serde(rename = "Europe/Tallinn")]
    EuropeTallinn,
    #[serde(rename = "Europe/Tirane")]
    EuropeTirane,
    #[serde(rename = "Europe/Uzhgorod")]
    EuropeUzhgorod,
    #[serde(rename = "Europe/Vaduz")]
    EuropeVaduz,
    #[serde(rename = "Europe/Vienna")]
    EuropeVienna,
    #[serde(rename = "Europe/Vilnius")]
    EuropeVilnius,
    #[serde(rename = "Europe/Volgograd")]
    EuropeVolgograd,
    #[serde(rename = "Europe/Warsaw")]
    EuropeWarsaw,
    #[serde(rename = "Europe/Zagreb")]
    EuropeZagreb,
    #[serde(rename = "Europe/Zurich")]
    EuropeZurich,

    // Indian
    #[serde(rename = "Indian/Antananarivo")]
    IndianAntananarivo,
    #[serde(rename = "Indian/Chagos")]
    IndianChagos,
    #[serde(rename = "Indian/Christmas")]
    IndianChristmas,
    #[serde(rename = "Indian/Mahe")]
    IndianMahe,
    #[serde(rename = "Indian/Maldives")]
    IndianMaldives,
    #[serde(rename = "Indian/Mauritius")]
    IndianMauritius,
    #[serde(rename = "Indian/Mayotte")]
    IndianMayotte,
    #[serde(rename = "Indian/Reunion")]
    IndianReunion,

    // Pacific
    #[serde(rename = "Pacific/Apia")]
    PacificApia,
    #[serde(rename = "Pacific/Auckland")]
    PacificAuckland,
    #[serde(rename = "Pacific/Easter")]
    PacificEaster,
    #[serde(rename = "Pacific/Efate")]
    PacificEfate,
    #[serde(rename = "Pacific/Fiji")]
    PacificFiji,
    #[serde(rename = "Pacific/Galapagos")]
    PacificGalapagos,
    #[serde(rename = "Pacific/Guadalcanal")]
    PacificGuadalcanal,
    #[serde(rename = "Pacific/Guam")]
    PacificGuam,
    #[serde(rename = "Pacific/Honolulu")]
    PacificHonolulu,
    #[serde(rename = "Pacific/Majuro")]
    PacificMajuro,
    #[serde(rename = "Pacific/Midway")]
    PacificMidway,
    #[serde(rename = "Pacific/Noumea")]
    PacificNoumea,
    #[serde(rename = "Pacific/Palau")]
    PacificPalau,
    #[serde(rename = "Pacific/Pitcairn")]
    PacificPitcairn,
    #[serde(rename = "Pacific/Port_Moresby")]
    PacificPortMoresby,
    #[serde(rename = "Pacific/Rarotonga")]
    PacificRarotonga,
    #[serde(rename = "Pacific/Tahiti")]
    PacificTahiti,
    #[serde(rename = "Pacific/Tongatapu")]
    PacificTongatapu,
}

impl IanaTimeZone {
    /// Every [`IanaTimeZone`] variant, in declaration order.
    pub const ALL: [Self; 362] = [
        Self::AfricaAbidjan,
        Self::AfricaAccra,
        Self::AfricaAddisAbaba,
        Self::AfricaAlgiers,
        Self::AfricaAsmara,
        Self::AfricaBamako,
        Self::AfricaBangui,
        Self::AfricaBanjul,
        Self::AfricaBissau,
        Self::AfricaBlantyre,
        Self::AfricaBujumbura,
        Self::AfricaBrazzaville,
        Self::AfricaCairo,
        Self::AfricaCasablanca,
        Self::AfricaCeuta,
        Self::AfricaConakry,
        Self::AfricaDakar,
        Self::AfricaDarEsSalaam,
        Self::AfricaDjibouti,
        Self::AfricaDouala,
        Self::AfricaFreetown,
        Self::AfricaGaborone,
        Self::AfricaHarare,
        Self::AfricaJohannesburg,
        Self::AfricaJuba,
        Self::AfricaKampala,
        Self::AfricaKhartoum,
        Self::AfricaKigali,
        Self::AfricaKinshasa,
        Self::AfricaLagos,
        Self::AfricaLibreville,
        Self::AfricaLome,
        Self::AfricaLuanda,
        Self::AfricaLubumbashi,
        Self::AfricaLusaka,
        Self::AfricaMalabo,
        Self::AfricaMaputo,
        Self::AfricaMaseru,
        Self::AfricaMbabane,
        Self::AfricaMogadishu,
        Self::AfricaMonrovia,
        Self::AfricaNairobi,
        Self::AfricaNdjamena,
        Self::AfricaNiamey,
        Self::AfricaNouakchott,
        Self::AfricaOuagadougou,
        Self::AfricaPortoNovo,
        Self::AfricaTripoli,
        Self::AfricaTunis,
        Self::AfricaWindhoek,
        Self::AmericaAnchorage,
        Self::AmericaAdak,
        Self::AmericaAnguilla,
        Self::AmericaAraguaina,
        Self::AmericaAntigua,
        Self::AmericaArgentinaBuenosAires,
        Self::AmericaArgentinaCordoba,
        Self::AmericaArgentinaSanJuan,
        Self::AmericaArgentinaSanLuis,
        Self::AmericaArgentinaUshuaia,
        Self::AmericaAruba,
        Self::AmericaAsuncion,
        Self::AmericaAtikokan,
        Self::AmericaBahia,
        Self::AmericaBahiaBanderas,
        Self::AmericaBarbados,
        Self::AmericaBelem,
        Self::AmericaBelize,
        Self::AmericaBlancSablon,
        Self::AmericaBogota,
        Self::AmericaBoaVista,
        Self::AmericaBoise,
        Self::AmericaCambridgeBay,
        Self::AmericaCampoGrande,
        Self::AmericaCancun,
        Self::AmericaCaracas,
        Self::AmericaCayenne,
        Self::AmericaCayman,
        Self::AmericaChicago,
        Self::AmericaChihuahua,
        Self::AmericaCoralHarbour,
        Self::AmericaCordoba,
        Self::AmericaCostaRica,
        Self::AmericaCreston,
        Self::AmericaCuiaba,
        Self::AmericaCuracao,
        Self::AmericaDawson,
        Self::AmericaDawsonCreek,
        Self::AmericaDenver,
        Self::AmericaDetroit,
        Self::AmericaDominica,
        Self::AmericaEdmonton,
        Self::AmericaEirunepe,
        Self::AmericaElSalvador,
        Self::AmericaFortaleza,
        Self::AmericaGlaceBay,
        Self::AmericaGodthab,
        Self::AmericaGrandTurk,
        Self::AmericaGrenada,
        Self::AmericaGuadeloupe,
        Self::AmericaGuatemala,
        Self::AmericaGuayaquil,
        Self::AmericaGuyana,
        Self::AmericaHalifax,
        Self::AmericaHavana,
        Self::AmericaHermosillo,
        Self::AmericaIndianaIndianapolis,
        Self::AmericaIndianaVincennes,
        Self::AmericaInuvik,
        Self::AmericaIqaluit,
        Self::AmericaJamaica,
        Self::AmericaJuneau,
        Self::AmericaKentuckyLouisville,
        Self::AmericaKentuckyMonticello,
        Self::AmericaLaPaz,
        Self::AmericaLima,
        Self::AmericaLosAngeles,
        Self::AmericaLowerPrinces,
        Self::AmericaMaceio,
        Self::AmericaManagua,
        Self::AmericaManaus,
        Self::AmericaMatamoros,
        Self::AmericaMazatlan,
        Self::AmericaMenominee,
        Self::AmericaMerida,
        Self::AmericaMexicoCity,
        Self::AmericaMiquelon,
        Self::AmericaMoncton,
        Self::AmericaMonterrey,
        Self::AmericaMontevideo,
        Self::AmericaMontreal,
        Self::AmericaMontserrat,
        Self::AmericaNassau,
        Self::AmericaNewYork,
        Self::AmericaNipigon,
        Self::AmericaNoronha,
        Self::AmericaNorthDakotaBeulah,
        Self::AmericaNorthDakotaCenter,
        Self::AmericaOjinaga,
        Self::AmericaPanama,
        Self::AmericaParamaribo,
        Self::AmericaPhoenix,
        Self::AmericaPortAuPrince,
        Self::AmericaPortOfSpain,
        Self::AmericaPortoVelho,
        Self::AmericaPuertoRico,
        Self::AmericaRecife,
        Self::AmericaRegina,
        Self::AmericaResolute,
        Self::AmericaRioBranco,
        Self::AmericaSantiago,
        Self::AmericaSantoDomingo,
        Self::AmericaSaoPaulo,
        Self::AmericaSitka,
        Self::AmericaStJohns,
        Self::AmericaStKitts,
        Self::AmericaStLucia,
        Self::AmericaStThomas,
        Self::AmericaStVincent,
        Self::AmericaSwiftCurrent,
        Self::AmericaTegucigalpa,
        Self::AmericaThunderBay,
        Self::AmericaTijuana,
        Self::AmericaToronto,
        Self::AmericaTortola,
        Self::AmericaVancouver,
        Self::AmericaWinnipeg,
        Self::AntarcticaCasey,
        Self::AntarcticaDavis,
        Self::AntarcticaDumontDUrville,
        Self::AntarcticaMawson,
        Self::AntarcticaPalmer,
        Self::AntarcticaSyowa,
        Self::AntarcticaVostok,
        Self::AsiaAden,
        Self::AsiaAlmaty,
        Self::AsiaAmman,
        Self::AsiaAnadyr,
        Self::AsiaAqtau,
        Self::AsiaAshgabat,
        Self::AsiaBaghdad,
        Self::AsiaBahrain,
        Self::AsiaBaku,
        Self::AsiaBangkok,
        Self::AsiaBeirut,
        Self::AsiaBishkek,
        Self::AsiaBrunei,
        Self::AsiaChoibalsan,
        Self::AsiaChongqing,
        Self::AsiaColombo,
        Self::AsiaDamascus,
        Self::AsiaDhaka,
        Self::AsiaDili,
        Self::AsiaDubai,
        Self::AsiaDushanbe,
        Self::AsiaGaza,
        Self::AsiaHoChiMinh,
        Self::AsiaHongKong,
        Self::AsiaIrkutsk,
        Self::AsiaIstanbul,
        Self::AsiaJakarta,
        Self::AsiaJayapura,
        Self::AsiaJerusalem,
        Self::AsiaKabul,
        Self::AsiaKamchatka,
        Self::AsiaKarachi,
        Self::AsiaKathmandu,
        Self::AsiaKolkata,
        Self::AsiaKrasnoyarsk,
        Self::AsiaKualaLumpur,
        Self::AsiaKuching,
        Self::AsiaKuwait,
        Self::AsiaMacao,
        Self::AsiaMacau,
        Self::AsiaMagadan,
        Self::AsiaMakassar,
        Self::AsiaManila,
        Self::AsiaMuscat,
        Self::AsiaNicosia,
        Self::AsiaNovosibirsk,
        Self::AsiaOmsk,
        Self::AsiaOral,
        Self::AsiaPhnomPenh,
        Self::AsiaPontianak,
        Self::AsiaPyongyang,
        Self::AsiaQatar,
        Self::AsiaQyzylorda,
        Self::AsiaRangoon,
        Self::AsiaRiyadh,
        Self::AsiaSakhalin,
        Self::AsiaSamarkand,
        Self::AsiaSeoul,
        Self::AsiaShanghai,
        Self::AsiaSingapore,
        Self::AsiaTaipei,
        Self::AsiaTashkent,
        Self::AsiaTbilisi,
        Self::AsiaTehran,
        Self::AsiaThimphu,
        Self::AsiaTokyo,
        Self::AsiaUlaanbaatar,
        Self::AsiaUrumqi,
        Self::AsiaVientiane,
        Self::AsiaVladivostok,
        Self::AsiaYakutsk,
        Self::AsiaYekaterinburg,
        Self::AsiaYerevan,
        Self::AtlanticAzores,
        Self::AtlanticBermuda,
        Self::AtlanticCapeVerde,
        Self::AtlanticCanary,
        Self::AtlanticFaroe,
        Self::AtlanticMadeira,
        Self::AtlanticReykjavik,
        Self::AtlanticSouthGeorgia,
        Self::AtlanticStanley,
        Self::AustraliaAdelaide,
        Self::AustraliaBrisbane,
        Self::AustraliaCurrie,
        Self::AustraliaDarwin,
        Self::AustraliaEucla,
        Self::AustraliaHobart,
        Self::AustraliaLordHowe,
        Self::AustraliaMelbourne,
        Self::AustraliaPerth,
        Self::AustraliaSydney,
        Self::EtcGmt,
        Self::EtcGmtPlus10,
        Self::EtcGmtPlus3,
        Self::EtcGmtPlus4,
        Self::EtcGmtPlus5,
        Self::EtcGmtPlus6,
        Self::EtcGmtMinus1,
        Self::EtcGmtMinus10,
        Self::EtcGmtMinus12,
        Self::EtcGmtMinus2,
        Self::EtcGmtMinus3,
        Self::EtcGmtMinus4,
        Self::EtcGmtMinus5,
        Self::EtcGmtMinus8,
        Self::EtcGmtMinus9,
        Self::EtcUtc,
        Self::EuropeAmsterdam,
        Self::EuropeAndorra,
        Self::EuropeAthens,
        Self::EuropeBelfast,
        Self::EuropeBelgrade,
        Self::EuropeBerlin,
        Self::EuropeBratislava,
        Self::EuropeBrussels,
        Self::EuropeBucharest,
        Self::EuropeBudapest,
        Self::EuropeChisinau,
        Self::EuropeCopenhagen,
        Self::EuropeDublin,
        Self::EuropeGibraltar,
        Self::EuropeGuernsey,
        Self::EuropeHelsinki,
        Self::EuropeIsleOfMan,
        Self::EuropeIstanbul,
        Self::EuropeJersey,
        Self::EuropeKaliningrad,
        Self::EuropeKiev,
        Self::EuropeLisbon,
        Self::EuropeLjubljana,
        Self::EuropeLondon,
        Self::EuropeLuxembourg,
        Self::EuropeMadrid,
        Self::EuropeMalta,
        Self::EuropeMariehamn,
        Self::EuropeMinsk,
        Self::EuropeMonaco,
        Self::EuropeMoscow,
        Self::EuropeOslo,
        Self::EuropeParis,
        Self::EuropePodgorica,
        Self::EuropePrague,
        Self::EuropeRiga,
        Self::EuropeRome,
        Self::EuropeSanMarino,
        Self::EuropeSamara,
        Self::EuropeSimferopol,
        Self::EuropeSarajevo,
        Self::EuropeSkopje,
        Self::EuropeSofia,
        Self::EuropeStockholm,
        Self::EuropeTallinn,
        Self::EuropeTirane,
        Self::EuropeUzhgorod,
        Self::EuropeVaduz,
        Self::EuropeVienna,
        Self::EuropeVilnius,
        Self::EuropeVolgograd,
        Self::EuropeWarsaw,
        Self::EuropeZagreb,
        Self::EuropeZurich,
        Self::IndianAntananarivo,
        Self::IndianChagos,
        Self::IndianChristmas,
        Self::IndianMahe,
        Self::IndianMaldives,
        Self::IndianMauritius,
        Self::IndianMayotte,
        Self::IndianReunion,
        Self::PacificApia,
        Self::PacificAuckland,
        Self::PacificEaster,
        Self::PacificEfate,
        Self::PacificFiji,
        Self::PacificGalapagos,
        Self::PacificGuadalcanal,
        Self::PacificGuam,
        Self::PacificHonolulu,
        Self::PacificMajuro,
        Self::PacificMidway,
        Self::PacificNoumea,
        Self::PacificPalau,
        Self::PacificPitcairn,
        Self::PacificPortMoresby,
        Self::PacificRarotonga,
        Self::PacificTahiti,
        Self::PacificTongatapu,
    ];

    /// The exact wire label for this variant.
    // A flat one-arm-per-variant lookup table.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AfricaAbidjan => "Africa/Abidjan",
            Self::AfricaAccra => "Africa/Accra",
            Self::AfricaAddisAbaba => "Africa/Addis_Ababa",
            Self::AfricaAlgiers => "Africa/Algiers",
            Self::AfricaAsmara => "Africa/Asmara",
            Self::AfricaBamako => "Africa/Bamako",
            Self::AfricaBangui => "Africa/Bangui",
            Self::AfricaBanjul => "Africa/Banjul",
            Self::AfricaBissau => "Africa/Bissau",
            Self::AfricaBlantyre => "Africa/Blantyre",
            Self::AfricaBujumbura => "Africa/Bujumbura",
            Self::AfricaBrazzaville => "Africa/Brazzaville",
            Self::AfricaCairo => "Africa/Cairo",
            Self::AfricaCasablanca => "Africa/Casablanca",
            Self::AfricaCeuta => "Africa/Ceuta",
            Self::AfricaConakry => "Africa/Conakry",
            Self::AfricaDakar => "Africa/Dakar",
            Self::AfricaDarEsSalaam => "Africa/Dar_es_Salaam",
            Self::AfricaDjibouti => "Africa/Djibouti",
            Self::AfricaDouala => "Africa/Douala",
            Self::AfricaFreetown => "Africa/Freetown",
            Self::AfricaGaborone => "Africa/Gaborone",
            Self::AfricaHarare => "Africa/Harare",
            Self::AfricaJohannesburg => "Africa/Johannesburg",
            Self::AfricaJuba => "Africa/Juba",
            Self::AfricaKampala => "Africa/Kampala",
            Self::AfricaKhartoum => "Africa/Khartoum",
            Self::AfricaKigali => "Africa/Kigali",
            Self::AfricaKinshasa => "Africa/Kinshasa",
            Self::AfricaLagos => "Africa/Lagos",
            Self::AfricaLibreville => "Africa/Libreville",
            Self::AfricaLome => "Africa/Lome",
            Self::AfricaLuanda => "Africa/Luanda",
            Self::AfricaLubumbashi => "Africa/Lubumbashi",
            Self::AfricaLusaka => "Africa/Lusaka",
            Self::AfricaMalabo => "Africa/Malabo",
            Self::AfricaMaputo => "Africa/Maputo",
            Self::AfricaMaseru => "Africa/Maseru",
            Self::AfricaMbabane => "Africa/Mbabane",
            Self::AfricaMogadishu => "Africa/Mogadishu",
            Self::AfricaMonrovia => "Africa/Monrovia",
            Self::AfricaNairobi => "Africa/Nairobi",
            Self::AfricaNdjamena => "Africa/Ndjamena",
            Self::AfricaNiamey => "Africa/Niamey",
            Self::AfricaNouakchott => "Africa/Nouakchott",
            Self::AfricaOuagadougou => "Africa/Ouagadougou",
            Self::AfricaPortoNovo => "Africa/Porto-Novo",
            Self::AfricaTripoli => "Africa/Tripoli",
            Self::AfricaTunis => "Africa/Tunis",
            Self::AfricaWindhoek => "Africa/Windhoek",
            Self::AmericaAnchorage => "America/Anchorage",
            Self::AmericaAdak => "America/Adak",
            Self::AmericaAnguilla => "America/Anguilla",
            Self::AmericaAraguaina => "America/Araguaina",
            Self::AmericaAntigua => "America/Antigua",
            Self::AmericaArgentinaBuenosAires => "America/Argentina/Buenos_Aires",
            Self::AmericaArgentinaCordoba => "America/Argentina/Cordoba",
            Self::AmericaArgentinaSanJuan => "America/Argentina/San_Juan",
            Self::AmericaArgentinaSanLuis => "America/Argentina/San_Luis",
            Self::AmericaArgentinaUshuaia => "America/Argentina/Ushuaia",
            Self::AmericaAruba => "America/Aruba",
            Self::AmericaAsuncion => "America/Asuncion",
            Self::AmericaAtikokan => "America/Atikokan",
            Self::AmericaBahia => "America/Bahia",
            Self::AmericaBahiaBanderas => "America/Bahia_Banderas",
            Self::AmericaBarbados => "America/Barbados",
            Self::AmericaBelem => "America/Belem",
            Self::AmericaBelize => "America/Belize",
            Self::AmericaBlancSablon => "America/Blanc-Sablon",
            Self::AmericaBogota => "America/Bogota",
            Self::AmericaBoaVista => "America/Boa_Vista",
            Self::AmericaBoise => "America/Boise",
            Self::AmericaCambridgeBay => "America/Cambridge_Bay",
            Self::AmericaCampoGrande => "America/Campo_Grande",
            Self::AmericaCancun => "America/Cancun",
            Self::AmericaCaracas => "America/Caracas",
            Self::AmericaCayenne => "America/Cayenne",
            Self::AmericaCayman => "America/Cayman",
            Self::AmericaChicago => "America/Chicago",
            Self::AmericaChihuahua => "America/Chihuahua",
            Self::AmericaCoralHarbour => "America/Coral_Harbour",
            Self::AmericaCordoba => "America/Cordoba",
            Self::AmericaCostaRica => "America/Costa_Rica",
            Self::AmericaCreston => "America/Creston",
            Self::AmericaCuiaba => "America/Cuiaba",
            Self::AmericaCuracao => "America/Curacao",
            Self::AmericaDawson => "America/Dawson",
            Self::AmericaDawsonCreek => "America/Dawson_Creek",
            Self::AmericaDenver => "America/Denver",
            Self::AmericaDetroit => "America/Detroit",
            Self::AmericaDominica => "America/Dominica",
            Self::AmericaEdmonton => "America/Edmonton",
            Self::AmericaEirunepe => "America/Eirunepe",
            Self::AmericaElSalvador => "America/El_Salvador",
            Self::AmericaFortaleza => "America/Fortaleza",
            Self::AmericaGlaceBay => "America/Glace_Bay",
            Self::AmericaGodthab => "America/Godthab",
            Self::AmericaGrandTurk => "America/Grand_Turk",
            Self::AmericaGrenada => "America/Grenada",
            Self::AmericaGuadeloupe => "America/Guadeloupe",
            Self::AmericaGuatemala => "America/Guatemala",
            Self::AmericaGuayaquil => "America/Guayaquil",
            Self::AmericaGuyana => "America/Guyana",
            Self::AmericaHalifax => "America/Halifax",
            Self::AmericaHavana => "America/Havana",
            Self::AmericaHermosillo => "America/Hermosillo",
            Self::AmericaIndianaIndianapolis => "America/Indiana/Indianapolis",
            Self::AmericaIndianaVincennes => "America/Indiana/Vincennes",
            Self::AmericaInuvik => "America/Inuvik",
            Self::AmericaIqaluit => "America/Iqaluit",
            Self::AmericaJamaica => "America/Jamaica",
            Self::AmericaJuneau => "America/Juneau",
            Self::AmericaKentuckyLouisville => "America/Kentucky/Louisville",
            Self::AmericaKentuckyMonticello => "America/Kentucky/Monticello",
            Self::AmericaLaPaz => "America/La_Paz",
            Self::AmericaLima => "America/Lima",
            Self::AmericaLosAngeles => "America/Los_Angeles",
            Self::AmericaLowerPrinces => "America/Lower_Princes",
            Self::AmericaMaceio => "America/Maceio",
            Self::AmericaManagua => "America/Managua",
            Self::AmericaManaus => "America/Manaus",
            Self::AmericaMatamoros => "America/Matamoros",
            Self::AmericaMazatlan => "America/Mazatlan",
            Self::AmericaMenominee => "America/Menominee",
            Self::AmericaMerida => "America/Merida",
            Self::AmericaMexicoCity => "America/Mexico_City",
            Self::AmericaMiquelon => "America/Miquelon",
            Self::AmericaMoncton => "America/Moncton",
            Self::AmericaMonterrey => "America/Monterrey",
            Self::AmericaMontevideo => "America/Montevideo",
            Self::AmericaMontreal => "America/Montreal",
            Self::AmericaMontserrat => "America/Montserrat",
            Self::AmericaNassau => "America/Nassau",
            Self::AmericaNewYork => "America/New_York",
            Self::AmericaNipigon => "America/Nipigon",
            Self::AmericaNoronha => "America/Noronha",
            Self::AmericaNorthDakotaBeulah => "America/North_Dakota/Beulah",
            Self::AmericaNorthDakotaCenter => "America/North_Dakota/Center",
            Self::AmericaOjinaga => "America/Ojinaga",
            Self::AmericaPanama => "America/Panama",
            Self::AmericaParamaribo => "America/Paramaribo",
            Self::AmericaPhoenix => "America/Phoenix",
            Self::AmericaPortAuPrince => "America/Port-au-Prince",
            Self::AmericaPortOfSpain => "America/Port_of_Spain",
            Self::AmericaPortoVelho => "America/Porto_Velho",
            Self::AmericaPuertoRico => "America/Puerto_Rico",
            Self::AmericaRecife => "America/Recife",
            Self::AmericaRegina => "America/Regina",
            Self::AmericaResolute => "America/Resolute",
            Self::AmericaRioBranco => "America/Rio_Branco",
            Self::AmericaSantiago => "America/Santiago",
            Self::AmericaSantoDomingo => "America/Santo_Domingo",
            Self::AmericaSaoPaulo => "America/Sao_Paulo",
            Self::AmericaSitka => "America/Sitka",
            Self::AmericaStJohns => "America/St_Johns",
            Self::AmericaStKitts => "America/St_Kitts",
            Self::AmericaStLucia => "America/St_Lucia",
            Self::AmericaStThomas => "America/St_Thomas",
            Self::AmericaStVincent => "America/St_Vincent",
            Self::AmericaSwiftCurrent => "America/Swift_Current",
            Self::AmericaTegucigalpa => "America/Tegucigalpa",
            Self::AmericaThunderBay => "America/Thunder_Bay",
            Self::AmericaTijuana => "America/Tijuana",
            Self::AmericaToronto => "America/Toronto",
            Self::AmericaTortola => "America/Tortola",
            Self::AmericaVancouver => "America/Vancouver",
            Self::AmericaWinnipeg => "America/Winnipeg",
            Self::AntarcticaCasey => "Antarctica/Casey",
            Self::AntarcticaDavis => "Antarctica/Davis",
            Self::AntarcticaDumontDUrville => "Antarctica/DumontDUrville",
            Self::AntarcticaMawson => "Antarctica/Mawson",
            Self::AntarcticaPalmer => "Antarctica/Palmer",
            Self::AntarcticaSyowa => "Antarctica/Syowa",
            Self::AntarcticaVostok => "Antarctica/Vostok",
            Self::AsiaAden => "Asia/Aden",
            Self::AsiaAlmaty => "Asia/Almaty",
            Self::AsiaAmman => "Asia/Amman",
            Self::AsiaAnadyr => "Asia/Anadyr",
            Self::AsiaAqtau => "Asia/Aqtau",
            Self::AsiaAshgabat => "Asia/Ashgabat",
            Self::AsiaBaghdad => "Asia/Baghdad",
            Self::AsiaBahrain => "Asia/Bahrain",
            Self::AsiaBaku => "Asia/Baku",
            Self::AsiaBangkok => "Asia/Bangkok",
            Self::AsiaBeirut => "Asia/Beirut",
            Self::AsiaBishkek => "Asia/Bishkek",
            Self::AsiaBrunei => "Asia/Brunei",
            Self::AsiaChoibalsan => "Asia/Choibalsan",
            Self::AsiaChongqing => "Asia/Chongqing",
            Self::AsiaColombo => "Asia/Colombo",
            Self::AsiaDamascus => "Asia/Damascus",
            Self::AsiaDhaka => "Asia/Dhaka",
            Self::AsiaDili => "Asia/Dili",
            Self::AsiaDubai => "Asia/Dubai",
            Self::AsiaDushanbe => "Asia/Dushanbe",
            Self::AsiaGaza => "Asia/Gaza",
            Self::AsiaHoChiMinh => "Asia/Ho_Chi_Minh",
            Self::AsiaHongKong => "Asia/Hong_Kong",
            Self::AsiaIrkutsk => "Asia/Irkutsk",
            Self::AsiaIstanbul => "Asia/Istanbul",
            Self::AsiaJakarta => "Asia/Jakarta",
            Self::AsiaJayapura => "Asia/Jayapura",
            Self::AsiaJerusalem => "Asia/Jerusalem",
            Self::AsiaKabul => "Asia/Kabul",
            Self::AsiaKamchatka => "Asia/Kamchatka",
            Self::AsiaKarachi => "Asia/Karachi",
            Self::AsiaKathmandu => "Asia/Kathmandu",
            Self::AsiaKolkata => "Asia/Kolkata",
            Self::AsiaKrasnoyarsk => "Asia/Krasnoyarsk",
            Self::AsiaKualaLumpur => "Asia/Kuala_Lumpur",
            Self::AsiaKuching => "Asia/Kuching",
            Self::AsiaKuwait => "Asia/Kuwait",
            Self::AsiaMacao => "Asia/Macao",
            Self::AsiaMacau => "Asia/Macau",
            Self::AsiaMagadan => "Asia/Magadan",
            Self::AsiaMakassar => "Asia/Makassar",
            Self::AsiaManila => "Asia/Manila",
            Self::AsiaMuscat => "Asia/Muscat",
            Self::AsiaNicosia => "Asia/Nicosia",
            Self::AsiaNovosibirsk => "Asia/Novosibirsk",
            Self::AsiaOmsk => "Asia/Omsk",
            Self::AsiaOral => "Asia/Oral",
            Self::AsiaPhnomPenh => "Asia/Phnom_Penh",
            Self::AsiaPontianak => "Asia/Pontianak",
            Self::AsiaPyongyang => "Asia/Pyongyang",
            Self::AsiaQatar => "Asia/Qatar",
            Self::AsiaQyzylorda => "Asia/Qyzylorda",
            Self::AsiaRangoon => "Asia/Rangoon",
            Self::AsiaRiyadh => "Asia/Riyadh",
            Self::AsiaSakhalin => "Asia/Sakhalin",
            Self::AsiaSamarkand => "Asia/Samarkand",
            Self::AsiaSeoul => "Asia/Seoul",
            Self::AsiaShanghai => "Asia/Shanghai",
            Self::AsiaSingapore => "Asia/Singapore",
            Self::AsiaTaipei => "Asia/Taipei",
            Self::AsiaTashkent => "Asia/Tashkent",
            Self::AsiaTbilisi => "Asia/Tbilisi",
            Self::AsiaTehran => "Asia/Tehran",
            Self::AsiaThimphu => "Asia/Thimphu",
            Self::AsiaTokyo => "Asia/Tokyo",
            Self::AsiaUlaanbaatar => "Asia/Ulaanbaatar",
            Self::AsiaUrumqi => "Asia/Urumqi",
            Self::AsiaVientiane => "Asia/Vientiane",
            Self::AsiaVladivostok => "Asia/Vladivostok",
            Self::AsiaYakutsk => "Asia/Yakutsk",
            Self::AsiaYekaterinburg => "Asia/Yekaterinburg",
            Self::AsiaYerevan => "Asia/Yerevan",
            Self::AtlanticAzores => "Atlantic/Azores",
            Self::AtlanticBermuda => "Atlantic/Bermuda",
            Self::AtlanticCapeVerde => "Atlantic/Cape_Verde",
            Self::AtlanticCanary => "Atlantic/Canary",
            Self::AtlanticFaroe => "Atlantic/Faroe",
            Self::AtlanticMadeira => "Atlantic/Madeira",
            Self::AtlanticReykjavik => "Atlantic/Reykjavik",
            Self::AtlanticSouthGeorgia => "Atlantic/South_Georgia",
            Self::AtlanticStanley => "Atlantic/Stanley",
            Self::AustraliaAdelaide => "Australia/Adelaide",
            Self::AustraliaBrisbane => "Australia/Brisbane",
            Self::AustraliaCurrie => "Australia/Currie",
            Self::AustraliaDarwin => "Australia/Darwin",
            Self::AustraliaEucla => "Australia/Eucla",
            Self::AustraliaHobart => "Australia/Hobart",
            Self::AustraliaLordHowe => "Australia/Lord_Howe",
            Self::AustraliaMelbourne => "Australia/Melbourne",
            Self::AustraliaPerth => "Australia/Perth",
            Self::AustraliaSydney => "Australia/Sydney",
            Self::EtcGmt => "Etc/GMT",
            Self::EtcGmtPlus10 => "Etc/GMT+10",
            Self::EtcGmtPlus3 => "Etc/GMT+3",
            Self::EtcGmtPlus4 => "Etc/GMT+4",
            Self::EtcGmtPlus5 => "Etc/GMT+5",
            Self::EtcGmtPlus6 => "Etc/GMT+6",
            Self::EtcGmtMinus1 => "Etc/GMT-1",
            Self::EtcGmtMinus10 => "Etc/GMT-10",
            Self::EtcGmtMinus12 => "Etc/GMT-12",
            Self::EtcGmtMinus2 => "Etc/GMT-2",
            Self::EtcGmtMinus3 => "Etc/GMT-3",
            Self::EtcGmtMinus4 => "Etc/GMT-4",
            Self::EtcGmtMinus5 => "Etc/GMT-5",
            Self::EtcGmtMinus8 => "Etc/GMT-8",
            Self::EtcGmtMinus9 => "Etc/GMT-9",
            Self::EtcUtc => "Etc/UTC",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::EuropeAndorra => "Europe/Andorra",
            Self::EuropeAthens => "Europe/Athens",
            Self::EuropeBelfast => "Europe/Belfast",
            Self::EuropeBelgrade => "Europe/Belgrade",
            Self::EuropeBerlin => "Europe/Berlin",
            Self::EuropeBratislava => "Europe/Bratislava",
            Self::EuropeBrussels => "Europe/Brussels",
            Self::EuropeBucharest => "Europe/Bucharest",
            Self::EuropeBudapest => "Europe/Budapest",
            Self::EuropeChisinau => "Europe/Chisinau",
            Self::EuropeCopenhagen => "Europe/Copenhagen",
            Self::EuropeDublin => "Europe/Dublin",
            Self::EuropeGibraltar => "Europe/Gibraltar",
            Self::EuropeGuernsey => "Europe/Guernsey",
            Self::EuropeHelsinki => "Europe/Helsinki",
            Self::EuropeIsleOfMan => "Europe/Isle_of_Man",
            Self::EuropeIstanbul => "Europe/Istanbul",
            Self::EuropeJersey => "Europe/Jersey",
            Self::EuropeKaliningrad => "Europe/Kaliningrad",
            Self::EuropeKiev => "Europe/Kiev",
            Self::EuropeLisbon => "Europe/Lisbon",
            Self::EuropeLjubljana => "Europe/Ljubljana",
            Self::EuropeLondon => "Europe/London",
            Self::EuropeLuxembourg => "Europe/Luxembourg",
            Self::EuropeMadrid => "Europe/Madrid",
            Self::EuropeMalta => "Europe/Malta",
            Self::EuropeMariehamn => "Europe/Mariehamn",
            Self::EuropeMinsk => "Europe/Minsk",
            Self::EuropeMonaco => "Europe/Monaco",
            Self::EuropeMoscow => "Europe/Moscow",
            Self::EuropeOslo => "Europe/Oslo",
            Self::EuropeParis => "Europe/Paris",
            Self::EuropePodgorica => "Europe/Podgorica",
            Self::EuropePrague => "Europe/Prague",
            Self::EuropeRiga => "Europe/Riga",
            Self::EuropeRome => "Europe/Rome",
            Self::EuropeSanMarino => "Europe/San_Marino",
            Self::EuropeSamara => "Europe/Samara",
            Self::EuropeSimferopol => "Europe/Simferopol",
            Self::EuropeSarajevo => "Europe/Sarajevo",
            Self::EuropeSkopje => "Europe/Skopje",
            Self::EuropeSofia => "Europe/Sofia",
            Self::EuropeStockholm => "Europe/Stockholm",
            Self::EuropeTallinn => "Europe/Tallinn",
            Self::EuropeTirane => "Europe/Tirane",
            Self::EuropeUzhgorod => "Europe/Uzhgorod",
            Self::EuropeVaduz => "Europe/Vaduz",
            Self::EuropeVienna => "Europe/Vienna",
            Self::EuropeVilnius => "Europe/Vilnius",
            Self::EuropeVolgograd => "Europe/Volgograd",
            Self::EuropeWarsaw => "Europe/Warsaw",
            Self::EuropeZagreb => "Europe/Zagreb",
            Self::EuropeZurich => "Europe/Zurich",
            Self::IndianAntananarivo => "Indian/Antananarivo",
            Self::IndianChagos => "Indian/Chagos",
            Self::IndianChristmas => "Indian/Christmas",
            Self::IndianMahe => "Indian/Mahe",
            Self::IndianMaldives => "Indian/Maldives",
            Self::IndianMauritius => "Indian/Mauritius",
            Self::IndianMayotte => "Indian/Mayotte",
            Self::IndianReunion => "Indian/Reunion",
            Self::PacificApia => "Pacific/Apia",
            Self::PacificAuckland => "Pacific/Auckland",
            Self::PacificEaster => "Pacific/Easter",
            Self::PacificEfate => "Pacific/Efate",
            Self::PacificFiji => "Pacific/Fiji",
            Self::PacificGalapagos => "Pacific/Galapagos",
            Self::PacificGuadalcanal => "Pacific/Guadalcanal",
            Self::PacificGuam => "Pacific/Guam",
            Self::PacificHonolulu => "Pacific/Honolulu",
            Self::PacificMajuro => "Pacific/Majuro",
            Self::PacificMidway => "Pacific/Midway",
            Self::PacificNoumea => "Pacific/Noumea",
            Self::PacificPalau => "Pacific/Palau",
            Self::PacificPitcairn => "Pacific/Pitcairn",
            Self::PacificPortMoresby => "Pacific/Port_Moresby",
            Self::PacificRarotonga => "Pacific/Rarotonga",
            Self::PacificTahiti => "Pacific/Tahiti",
            Self::PacificTongatapu => "Pacific/Tongatapu",
        }
    }
}

/// A deprecated IANA time zone alias that appears in the data, e.g.
/// `Asia/Calcutta`. Round-trips as-is but [`canonical`](Self::canonical)
/// resolves it to the modern [`IanaTimeZone`].
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Deprecated {
    #[serde(rename = "America/Buenos_Aires")]
    AmericaBuenosAires,
    #[serde(rename = "Asia/Calcutta")]
    AsiaCalcutta,
    #[serde(rename = "Asia/Katmandu")]
    AsiaKatmandu,
    #[serde(rename = "Asia/Saigon")]
    AsiaSaigon,
    #[serde(rename = "Brazil/East")]
    BrazilEast,
    #[serde(rename = "Canada/Atlantic")]
    CanadaAtlantic,
    #[serde(rename = "Canada/Eastern")]
    CanadaEastern,
    #[serde(rename = "Canada/Mountain")]
    CanadaMountain,
    #[serde(rename = "Canada/Pacific")]
    CanadaPacific,
    #[serde(rename = "Canada/Saskatchewan")]
    CanadaSaskatchewan,
    #[serde(rename = "Chile/Continental")]
    ChileContinental,
    #[serde(rename = "Eire")]
    Eire,
    #[serde(rename = "Iceland")]
    Iceland,
    #[serde(rename = "Iran")]
    Iran,
    #[serde(rename = "Jamaica")]
    Jamaica,
    #[serde(rename = "Mexico/BajaSur")]
    MexicoBajaSur,
    #[serde(rename = "US/Central")]
    UsCentral,
    #[serde(rename = "US/Eastern")]
    UsEastern,
    #[serde(rename = "US/Indiana-Starke")]
    UsIndianaStarke,
    #[serde(rename = "US/Michigan")]
    UsMichigan,
    #[serde(rename = "US/Mountain")]
    UsMountain,
    #[serde(rename = "US/Pacific")]
    UsPacific,
    #[serde(rename = "W-SU")]
    WSu,
    #[serde(rename = "Zulu")]
    Zulu,
}

impl Deprecated {
    /// Every [`Deprecated`] variant, in declaration order.
    pub const ALL: [Self; 24] = [
        Self::AmericaBuenosAires,
        Self::AsiaCalcutta,
        Self::AsiaKatmandu,
        Self::AsiaSaigon,
        Self::BrazilEast,
        Self::CanadaAtlantic,
        Self::CanadaEastern,
        Self::CanadaMountain,
        Self::CanadaPacific,
        Self::CanadaSaskatchewan,
        Self::ChileContinental,
        Self::Eire,
        Self::Iceland,
        Self::Iran,
        Self::Jamaica,
        Self::MexicoBajaSur,
        Self::UsCentral,
        Self::UsEastern,
        Self::UsIndianaStarke,
        Self::UsMichigan,
        Self::UsMountain,
        Self::UsPacific,
        Self::WSu,
        Self::Zulu,
    ];

    /// The exact wire label for this variant.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmericaBuenosAires => "America/Buenos_Aires",
            Self::AsiaCalcutta => "Asia/Calcutta",
            Self::AsiaKatmandu => "Asia/Katmandu",
            Self::AsiaSaigon => "Asia/Saigon",
            Self::BrazilEast => "Brazil/East",
            Self::CanadaAtlantic => "Canada/Atlantic",
            Self::CanadaEastern => "Canada/Eastern",
            Self::CanadaMountain => "Canada/Mountain",
            Self::CanadaPacific => "Canada/Pacific",
            Self::CanadaSaskatchewan => "Canada/Saskatchewan",
            Self::ChileContinental => "Chile/Continental",
            Self::Eire => "Eire",
            Self::Iceland => "Iceland",
            Self::Iran => "Iran",
            Self::Jamaica => "Jamaica",
            Self::MexicoBajaSur => "Mexico/BajaSur",
            Self::UsCentral => "US/Central",
            Self::UsEastern => "US/Eastern",
            Self::UsIndianaStarke => "US/Indiana-Starke",
            Self::UsMichigan => "US/Michigan",
            Self::UsMountain => "US/Mountain",
            Self::UsPacific => "US/Pacific",
            Self::WSu => "W-SU",
            Self::Zulu => "Zulu",
        }
    }
}

impl Deprecated {
    /// Returns the modern [`IanaTimeZone`] this deprecated alias resolves to.
    #[must_use]
    pub const fn canonical(self) -> IanaTimeZone {
        match self {
            Self::AmericaBuenosAires => IanaTimeZone::AmericaArgentinaBuenosAires,
            Self::AsiaCalcutta => IanaTimeZone::AsiaKolkata,
            Self::AsiaKatmandu => IanaTimeZone::AsiaKathmandu,
            Self::AsiaSaigon => IanaTimeZone::AsiaHoChiMinh,
            Self::BrazilEast => IanaTimeZone::AmericaSaoPaulo,
            Self::CanadaAtlantic => IanaTimeZone::AmericaHalifax,
            Self::CanadaEastern => IanaTimeZone::AmericaToronto,
            Self::CanadaMountain => IanaTimeZone::AmericaEdmonton,
            Self::CanadaPacific => IanaTimeZone::AmericaVancouver,
            Self::CanadaSaskatchewan => IanaTimeZone::AmericaRegina,
            Self::ChileContinental => IanaTimeZone::AmericaSantiago,
            Self::Eire => IanaTimeZone::EuropeDublin,
            Self::Iceland => IanaTimeZone::AtlanticReykjavik,
            Self::Iran => IanaTimeZone::AsiaTehran,
            Self::Jamaica => IanaTimeZone::AmericaJamaica,
            Self::MexicoBajaSur => IanaTimeZone::AmericaMazatlan,
            Self::UsCentral | Self::UsIndianaStarke => IanaTimeZone::AmericaChicago,
            Self::UsEastern => IanaTimeZone::AmericaNewYork,
            Self::UsMichigan => IanaTimeZone::AmericaDetroit,
            Self::UsPacific => IanaTimeZone::AmericaLosAngeles,
            Self::UsMountain => IanaTimeZone::AmericaDenver,
            Self::WSu => IanaTimeZone::EuropeMoscow,
            Self::Zulu => IanaTimeZone::EtcUtc,
        }
    }
}

/// A time zone abbreviation or UTC offset label, e.g. `JST` or `GMT+9`.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Abbreviation {
    #[serde(rename = "ACT")]
    Act,
    #[serde(rename = "ART")]
    Art,
    #[serde(rename = "AST")]
    Ast,
    #[serde(rename = "BST")]
    Bst,
    #[serde(rename = "CAT")]
    Cat,
    #[serde(rename = "CDT")]
    Cdt,
    #[serde(rename = "CET")]
    Cet,
    #[serde(rename = "CST")]
    Cst,
    #[serde(rename = "EAT")]
    Eat,
    #[serde(rename = "ECT")]
    Ect,
    #[serde(rename = "EDT")]
    Edt,
    #[serde(rename = "EET")]
    Eet,
    #[serde(rename = "EST")]
    Est,
    #[serde(rename = "GMT")]
    Gmt,
    #[serde(rename = "GMT+0")]
    GmtPlus0,
    #[serde(rename = "GMT+1")]
    GmtPlus1,
    #[serde(rename = "GMT+2")]
    GmtPlus2,
    #[serde(rename = "GMT+3")]
    GmtPlus3,
    #[serde(rename = "GMT+4")]
    GmtPlus4,
    #[serde(rename = "GMT+5")]
    GmtPlus5,
    #[serde(rename = "GMT+6")]
    GmtPlus6,
    #[serde(rename = "GMT+7")]
    GmtPlus7,
    #[serde(rename = "GMT+8")]
    GmtPlus8,
    #[serde(rename = "GMT+9")]
    GmtPlus9,
    #[serde(rename = "GMT+10")]
    GmtPlus10,
    #[serde(rename = "GMT+11")]
    GmtPlus11,
    #[serde(rename = "GMT+12")]
    GmtPlus12,
    #[serde(rename = "GMT-2")]
    GmtMinus2,
    #[serde(rename = "GMT-3")]
    GmtMinus3,
    #[serde(rename = "GMT-4")]
    GmtMinus4,
    #[serde(rename = "GMT-5")]
    GmtMinus5,
    #[serde(rename = "GMT-6")]
    GmtMinus6,
    #[serde(rename = "GMT-7")]
    GmtMinus7,
    #[serde(rename = "GMT-8")]
    GmtMinus8,
    #[serde(rename = "GMT-10")]
    GmtMinus10,
    #[serde(rename = "GMT-11")]
    GmtMinus11,
    #[serde(rename = "HST")]
    Hst,
    #[serde(rename = "IST")]
    Ist,
    #[serde(rename = "JST")]
    Jst,
    #[serde(rename = "MDT")]
    Mdt,
    #[serde(rename = "MST")]
    Mst,
    #[serde(rename = "NST")]
    Nst,
    #[serde(rename = "PDT")]
    Pdt,
    #[serde(rename = "PST")]
    Pst,
    #[serde(rename = "PST8PDT")]
    Pst8Pdt,
    #[serde(rename = "ROK")]
    Rok,
    #[serde(rename = "UTC")]
    Utc,
    #[serde(rename = "WET")]
    Wet,
}

impl Abbreviation {
    /// Every [`Abbreviation`] variant, in declaration order.
    pub const ALL: [Self; 48] = [
        Self::Act,
        Self::Art,
        Self::Ast,
        Self::Bst,
        Self::Cat,
        Self::Cdt,
        Self::Cet,
        Self::Cst,
        Self::Eat,
        Self::Ect,
        Self::Edt,
        Self::Eet,
        Self::Est,
        Self::Gmt,
        Self::GmtPlus0,
        Self::GmtPlus1,
        Self::GmtPlus2,
        Self::GmtPlus3,
        Self::GmtPlus4,
        Self::GmtPlus5,
        Self::GmtPlus6,
        Self::GmtPlus7,
        Self::GmtPlus8,
        Self::GmtPlus9,
        Self::GmtPlus10,
        Self::GmtPlus11,
        Self::GmtPlus12,
        Self::GmtMinus2,
        Self::GmtMinus3,
        Self::GmtMinus4,
        Self::GmtMinus5,
        Self::GmtMinus6,
        Self::GmtMinus7,
        Self::GmtMinus8,
        Self::GmtMinus10,
        Self::GmtMinus11,
        Self::Hst,
        Self::Ist,
        Self::Jst,
        Self::Mdt,
        Self::Mst,
        Self::Nst,
        Self::Pdt,
        Self::Pst,
        Self::Pst8Pdt,
        Self::Rok,
        Self::Utc,
        Self::Wet,
    ];

    /// The exact wire label for this variant.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Act => "ACT",
            Self::Art => "ART",
            Self::Ast => "AST",
            Self::Bst => "BST",
            Self::Cat => "CAT",
            Self::Cdt => "CDT",
            Self::Cet => "CET",
            Self::Cst => "CST",
            Self::Eat => "EAT",
            Self::Ect => "ECT",
            Self::Edt => "EDT",
            Self::Eet => "EET",
            Self::Est => "EST",
            Self::Gmt => "GMT",
            Self::GmtPlus0 => "GMT+0",
            Self::GmtPlus1 => "GMT+1",
            Self::GmtPlus2 => "GMT+2",
            Self::GmtPlus3 => "GMT+3",
            Self::GmtPlus4 => "GMT+4",
            Self::GmtPlus5 => "GMT+5",
            Self::GmtPlus6 => "GMT+6",
            Self::GmtPlus7 => "GMT+7",
            Self::GmtPlus8 => "GMT+8",
            Self::GmtPlus9 => "GMT+9",
            Self::GmtPlus10 => "GMT+10",
            Self::GmtPlus11 => "GMT+11",
            Self::GmtPlus12 => "GMT+12",
            Self::GmtMinus2 => "GMT-2",
            Self::GmtMinus3 => "GMT-3",
            Self::GmtMinus4 => "GMT-4",
            Self::GmtMinus5 => "GMT-5",
            Self::GmtMinus6 => "GMT-6",
            Self::GmtMinus7 => "GMT-7",
            Self::GmtMinus8 => "GMT-8",
            Self::GmtMinus10 => "GMT-10",
            Self::GmtMinus11 => "GMT-11",
            Self::Hst => "HST",
            Self::Ist => "IST",
            Self::Jst => "JST",
            Self::Mdt => "MDT",
            Self::Mst => "MST",
            Self::Nst => "NST",
            Self::Pdt => "PDT",
            Self::Pst => "PST",
            Self::Pst8Pdt => "PST8PDT",
            Self::Rok => "ROK",
            Self::Utc => "UTC",
            Self::Wet => "WET",
        }
    }
}

impl Abbreviation {
    /// Returns the canonical [`IanaTimeZone`] this abbreviation most commonly denotes.
    ///
    /// Ambiguous abbreviations (e.g. `IST`, `CST`) resolve to their most common
    /// Twitter interpretation. Offset labels (`GMT+9`) resolve to a representative zone at that
    /// offset.
    #[must_use]
    pub const fn canonical(self) -> IanaTimeZone {
        match self {
            Self::Act => IanaTimeZone::AustraliaDarwin,
            Self::Art | Self::GmtMinus3 => IanaTimeZone::AmericaArgentinaBuenosAires,
            Self::Ast | Self::GmtMinus4 => IanaTimeZone::AmericaHalifax,
            Self::Bst => IanaTimeZone::EuropeLondon,
            Self::Cat => IanaTimeZone::AfricaMaputo,
            Self::Cdt | Self::Cst | Self::GmtMinus6 => IanaTimeZone::AmericaChicago,
            Self::Cet => IanaTimeZone::EuropeParis,
            Self::Eat | Self::GmtPlus3 => IanaTimeZone::AfricaNairobi,
            Self::Ect => IanaTimeZone::AmericaGuayaquil,
            Self::Edt | Self::Est | Self::GmtMinus5 => IanaTimeZone::AmericaNewYork,
            Self::Eet => IanaTimeZone::EuropeBucharest,
            Self::Gmt | Self::GmtPlus0 | Self::Utc => IanaTimeZone::EtcGmt,
            Self::GmtMinus10 | Self::Hst => IanaTimeZone::PacificHonolulu,
            Self::GmtMinus11 => IanaTimeZone::PacificMidway,
            Self::GmtMinus2 => IanaTimeZone::AtlanticSouthGeorgia,
            Self::GmtMinus7 | Self::Mdt | Self::Mst => IanaTimeZone::AmericaDenver,
            Self::GmtMinus8 | Self::Pdt | Self::Pst | Self::Pst8Pdt => {
                IanaTimeZone::AmericaLosAngeles
            }
            Self::GmtPlus1 => IanaTimeZone::AfricaLagos,
            Self::GmtPlus10 => IanaTimeZone::AustraliaBrisbane,
            Self::GmtPlus11 => IanaTimeZone::AsiaVladivostok,
            Self::GmtPlus12 => IanaTimeZone::PacificFiji,
            Self::GmtPlus2 => IanaTimeZone::AfricaCairo,
            Self::GmtPlus4 => IanaTimeZone::AsiaDubai,
            Self::GmtPlus5 => IanaTimeZone::AsiaKarachi,
            Self::GmtPlus6 => IanaTimeZone::AsiaDhaka,
            Self::GmtPlus7 => IanaTimeZone::AsiaBangkok,
            Self::GmtPlus8 => IanaTimeZone::AsiaShanghai,
            Self::GmtPlus9 | Self::Jst => IanaTimeZone::AsiaTokyo,
            Self::Ist => IanaTimeZone::AsiaKolkata,
            Self::Nst => IanaTimeZone::AmericaStJohns,
            Self::Rok => IanaTimeZone::AsiaSeoul,
            Self::Wet => IanaTimeZone::EuropeLisbon,
        }
    }
}

/// A Windows-style time zone display name, e.g. `Tokyo` or `Central Time (US & Canada)`.
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Named {
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
    Astana,
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
    Israel,
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
    Poland,
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
    Turkey,
    Urumqi,
    Vienna,
    Vilnius,
    Vladivostok,
    Volgograd,
    Warsaw,
    Wellington,
    Yakutsk,
    Yerevan,
    Zagreb,
}

impl Named {
    /// Every [`Named`] variant, in declaration order.
    pub const ALL: [Self; 145] = [
        Self::Central,
        Self::Mountain,
        Self::Pacific,
        Self::Eastern,
        Self::IndianaEast,
        Self::AtlanticCanada,
        Self::AbuDhabi,
        Self::BuenosAires,
        Self::CapeVerdeIs,
        Self::CentralAmerica,
        Self::InternationalDateLineWest,
        Self::HongKong,
        Self::KualaLumpur,
        Self::LaPaz,
        Self::MarshallIs,
        Self::MexicoCity,
        Self::MidAtlantic,
        Self::MidwayIsland,
        Self::NewCaledonia,
        Self::NewDelhi,
        Self::NukuAlofa,
        Self::PortMoresby,
        Self::SolomonIs,
        Self::SriJayawardenepura,
        Self::StPetersburg,
        Self::UlaanBataar,
        Self::WestCentralAfrica,
        Self::Adelaide,
        Self::Alaska,
        Self::Almaty,
        Self::Amsterdam,
        Self::Arizona,
        Self::Astana,
        Self::Athens,
        Self::Auckland,
        Self::Azores,
        Self::Baghdad,
        Self::Baku,
        Self::Bangkok,
        Self::Beijing,
        Self::Belgrade,
        Self::Berlin,
        Self::Bern,
        Self::Bogota,
        Self::Brasilia,
        Self::Bratislava,
        Self::Brisbane,
        Self::Brussels,
        Self::Bucharest,
        Self::Budapest,
        Self::Cairo,
        Self::Canberra,
        Self::Caracas,
        Self::Casablanca,
        Self::Chennai,
        Self::Chihuahua,
        Self::Chongqing,
        Self::Copenhagen,
        Self::Darwin,
        Self::Dhaka,
        Self::Dublin,
        Self::Edinburgh,
        Self::Ekaterinburg,
        Self::Fiji,
        Self::Georgetown,
        Self::Greenland,
        Self::Guadalajara,
        Self::Guam,
        Self::Hanoi,
        Self::Harare,
        Self::Hawaii,
        Self::Helsinki,
        Self::Hobart,
        Self::Irkutsk,
        Self::Israel,
        Self::Islamabad,
        Self::Istanbul,
        Self::Jakarta,
        Self::Jerusalem,
        Self::Kabul,
        Self::Kamchatka,
        Self::Karachi,
        Self::Kathmandu,
        Self::Kiev,
        Self::Kolkata,
        Self::Krasnoyarsk,
        Self::Kuwait,
        Self::Kyiv,
        Self::Lima,
        Self::Lisbon,
        Self::Ljubljana,
        Self::London,
        Self::Madrid,
        Self::Magadan,
        Self::Mazatlan,
        Self::Melbourne,
        Self::Minsk,
        Self::Monrovia,
        Self::Monterrey,
        Self::Moscow,
        Self::Mumbai,
        Self::Muscat,
        Self::Nairobi,
        Self::Newfoundland,
        Self::Novosibirsk,
        Self::Osaka,
        Self::Paris,
        Self::Perth,
        Self::Poland,
        Self::Prague,
        Self::Pretoria,
        Self::Quito,
        Self::Rangoon,
        Self::Riga,
        Self::Riyadh,
        Self::Rome,
        Self::Samoa,
        Self::Santiago,
        Self::Sapporo,
        Self::Sarajevo,
        Self::Saskatchewan,
        Self::Seoul,
        Self::Singapore,
        Self::Skopje,
        Self::Sofia,
        Self::Stockholm,
        Self::Sydney,
        Self::Taipei,
        Self::Tallinn,
        Self::Tashkent,
        Self::Tbilisi,
        Self::Tehran,
        Self::Tijuana,
        Self::Tokyo,
        Self::Turkey,
        Self::Urumqi,
        Self::Vienna,
        Self::Vilnius,
        Self::Vladivostok,
        Self::Volgograd,
        Self::Warsaw,
        Self::Wellington,
        Self::Yakutsk,
        Self::Yerevan,
        Self::Zagreb,
    ];

    /// The exact wire label for this variant.
    // A flat one-arm-per-variant lookup table.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Central => "Central Time (US & Canada)",
            Self::Mountain => "Mountain Time (US & Canada)",
            Self::Pacific => "Pacific Time (US & Canada)",
            Self::Eastern => "Eastern Time (US & Canada)",
            Self::IndianaEast => "Indiana (East)",
            Self::AtlanticCanada => "Atlantic Time (Canada)",
            Self::AbuDhabi => "Abu Dhabi",
            Self::BuenosAires => "Buenos Aires",
            Self::CapeVerdeIs => "Cape Verde Is.",
            Self::CentralAmerica => "Central America",
            Self::InternationalDateLineWest => "International Date Line West",
            Self::HongKong => "Hong Kong",
            Self::KualaLumpur => "Kuala Lumpur",
            Self::LaPaz => "La Paz",
            Self::MarshallIs => "Marshall Is.",
            Self::MexicoCity => "Mexico City",
            Self::MidAtlantic => "Mid-Atlantic",
            Self::MidwayIsland => "Midway Island",
            Self::NewCaledonia => "New Caledonia",
            Self::NewDelhi => "New Delhi",
            Self::NukuAlofa => "Nuku'alofa",
            Self::PortMoresby => "Port Moresby",
            Self::SolomonIs => "Solomon Is.",
            Self::SriJayawardenepura => "Sri Jayawardenepura",
            Self::StPetersburg => "St. Petersburg",
            Self::UlaanBataar => "Ulaan Bataar",
            Self::WestCentralAfrica => "West Central Africa",
            Self::Adelaide => "Adelaide",
            Self::Alaska => "Alaska",
            Self::Almaty => "Almaty",
            Self::Amsterdam => "Amsterdam",
            Self::Arizona => "Arizona",
            Self::Astana => "Astana",
            Self::Athens => "Athens",
            Self::Auckland => "Auckland",
            Self::Azores => "Azores",
            Self::Baghdad => "Baghdad",
            Self::Baku => "Baku",
            Self::Bangkok => "Bangkok",
            Self::Beijing => "Beijing",
            Self::Belgrade => "Belgrade",
            Self::Berlin => "Berlin",
            Self::Bern => "Bern",
            Self::Bogota => "Bogota",
            Self::Brasilia => "Brasilia",
            Self::Bratislava => "Bratislava",
            Self::Brisbane => "Brisbane",
            Self::Brussels => "Brussels",
            Self::Bucharest => "Bucharest",
            Self::Budapest => "Budapest",
            Self::Cairo => "Cairo",
            Self::Canberra => "Canberra",
            Self::Caracas => "Caracas",
            Self::Casablanca => "Casablanca",
            Self::Chennai => "Chennai",
            Self::Chihuahua => "Chihuahua",
            Self::Chongqing => "Chongqing",
            Self::Copenhagen => "Copenhagen",
            Self::Darwin => "Darwin",
            Self::Dhaka => "Dhaka",
            Self::Dublin => "Dublin",
            Self::Edinburgh => "Edinburgh",
            Self::Ekaterinburg => "Ekaterinburg",
            Self::Fiji => "Fiji",
            Self::Georgetown => "Georgetown",
            Self::Greenland => "Greenland",
            Self::Guadalajara => "Guadalajara",
            Self::Guam => "Guam",
            Self::Hanoi => "Hanoi",
            Self::Harare => "Harare",
            Self::Hawaii => "Hawaii",
            Self::Helsinki => "Helsinki",
            Self::Hobart => "Hobart",
            Self::Irkutsk => "Irkutsk",
            Self::Israel => "Israel",
            Self::Islamabad => "Islamabad",
            Self::Istanbul => "Istanbul",
            Self::Jakarta => "Jakarta",
            Self::Jerusalem => "Jerusalem",
            Self::Kabul => "Kabul",
            Self::Kamchatka => "Kamchatka",
            Self::Karachi => "Karachi",
            Self::Kathmandu => "Kathmandu",
            Self::Kiev => "Kiev",
            Self::Kolkata => "Kolkata",
            Self::Krasnoyarsk => "Krasnoyarsk",
            Self::Kuwait => "Kuwait",
            Self::Kyiv => "Kyiv",
            Self::Lima => "Lima",
            Self::Lisbon => "Lisbon",
            Self::Ljubljana => "Ljubljana",
            Self::London => "London",
            Self::Madrid => "Madrid",
            Self::Magadan => "Magadan",
            Self::Mazatlan => "Mazatlan",
            Self::Melbourne => "Melbourne",
            Self::Minsk => "Minsk",
            Self::Monrovia => "Monrovia",
            Self::Monterrey => "Monterrey",
            Self::Moscow => "Moscow",
            Self::Mumbai => "Mumbai",
            Self::Muscat => "Muscat",
            Self::Nairobi => "Nairobi",
            Self::Newfoundland => "Newfoundland",
            Self::Novosibirsk => "Novosibirsk",
            Self::Osaka => "Osaka",
            Self::Paris => "Paris",
            Self::Perth => "Perth",
            Self::Poland => "Poland",
            Self::Prague => "Prague",
            Self::Pretoria => "Pretoria",
            Self::Quito => "Quito",
            Self::Rangoon => "Rangoon",
            Self::Riga => "Riga",
            Self::Riyadh => "Riyadh",
            Self::Rome => "Rome",
            Self::Samoa => "Samoa",
            Self::Santiago => "Santiago",
            Self::Sapporo => "Sapporo",
            Self::Sarajevo => "Sarajevo",
            Self::Saskatchewan => "Saskatchewan",
            Self::Seoul => "Seoul",
            Self::Singapore => "Singapore",
            Self::Skopje => "Skopje",
            Self::Sofia => "Sofia",
            Self::Stockholm => "Stockholm",
            Self::Sydney => "Sydney",
            Self::Taipei => "Taipei",
            Self::Tallinn => "Tallinn",
            Self::Tashkent => "Tashkent",
            Self::Tbilisi => "Tbilisi",
            Self::Tehran => "Tehran",
            Self::Tijuana => "Tijuana",
            Self::Tokyo => "Tokyo",
            Self::Turkey => "Turkey",
            Self::Urumqi => "Urumqi",
            Self::Vienna => "Vienna",
            Self::Vilnius => "Vilnius",
            Self::Vladivostok => "Vladivostok",
            Self::Volgograd => "Volgograd",
            Self::Warsaw => "Warsaw",
            Self::Wellington => "Wellington",
            Self::Yakutsk => "Yakutsk",
            Self::Yerevan => "Yerevan",
            Self::Zagreb => "Zagreb",
        }
    }
}

impl Named {
    /// Returns the canonical [`IanaTimeZone`] this display name denotes.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn canonical(self) -> IanaTimeZone {
        match self {
            Self::AbuDhabi => IanaTimeZone::AsiaDubai,
            Self::Adelaide => IanaTimeZone::AustraliaAdelaide,
            Self::Alaska => IanaTimeZone::AmericaAnchorage,
            Self::Almaty | Self::Astana => IanaTimeZone::AsiaAlmaty,
            Self::Amsterdam => IanaTimeZone::EuropeAmsterdam,
            Self::Arizona => IanaTimeZone::AmericaPhoenix,
            Self::Athens => IanaTimeZone::EuropeAthens,
            Self::AtlanticCanada => IanaTimeZone::AmericaHalifax,
            Self::Auckland | Self::Wellington => IanaTimeZone::PacificAuckland,
            Self::Azores => IanaTimeZone::AtlanticAzores,
            Self::Baghdad => IanaTimeZone::AsiaBaghdad,
            Self::Baku => IanaTimeZone::AsiaBaku,
            Self::Bangkok => IanaTimeZone::AsiaBangkok,
            Self::Beijing | Self::Chongqing | Self::Urumqi => IanaTimeZone::AsiaShanghai,
            Self::Belgrade => IanaTimeZone::EuropeBelgrade,
            Self::Berlin => IanaTimeZone::EuropeBerlin,
            Self::Bern => IanaTimeZone::EuropeZurich,
            Self::Bogota => IanaTimeZone::AmericaBogota,
            Self::Brasilia => IanaTimeZone::AmericaSaoPaulo,
            Self::Bratislava => IanaTimeZone::EuropeBratislava,
            Self::Brisbane => IanaTimeZone::AustraliaBrisbane,
            Self::Brussels => IanaTimeZone::EuropeBrussels,
            Self::Bucharest => IanaTimeZone::EuropeBucharest,
            Self::Budapest => IanaTimeZone::EuropeBudapest,
            Self::BuenosAires => IanaTimeZone::AmericaArgentinaBuenosAires,
            Self::Cairo => IanaTimeZone::AfricaCairo,
            Self::Canberra | Self::Sydney => IanaTimeZone::AustraliaSydney,
            Self::CapeVerdeIs => IanaTimeZone::AtlanticCapeVerde,
            Self::Caracas => IanaTimeZone::AmericaCaracas,
            Self::Casablanca => IanaTimeZone::AfricaCasablanca,
            Self::Central => IanaTimeZone::AmericaChicago,
            Self::CentralAmerica => IanaTimeZone::AmericaCostaRica,
            Self::Chennai | Self::Kolkata | Self::Mumbai | Self::NewDelhi => {
                IanaTimeZone::AsiaKolkata
            }
            Self::Chihuahua => IanaTimeZone::AmericaChihuahua,
            Self::Copenhagen => IanaTimeZone::EuropeCopenhagen,
            Self::Darwin => IanaTimeZone::AustraliaDarwin,
            Self::Dhaka => IanaTimeZone::AsiaDhaka,
            Self::Dublin => IanaTimeZone::EuropeDublin,
            Self::Eastern => IanaTimeZone::AmericaNewYork,
            Self::Edinburgh | Self::London => IanaTimeZone::EuropeLondon,
            Self::Ekaterinburg => IanaTimeZone::AsiaYekaterinburg,
            Self::Fiji | Self::PortMoresby | Self::SolomonIs => IanaTimeZone::PacificFiji,
            Self::Georgetown => IanaTimeZone::AmericaGuyana,
            Self::Greenland => IanaTimeZone::AmericaGodthab,
            Self::Guadalajara | Self::MexicoCity => IanaTimeZone::AmericaMexicoCity,
            Self::Guam | Self::NukuAlofa | Self::Samoa => IanaTimeZone::PacificTongatapu,
            Self::Hanoi => IanaTimeZone::AsiaHoChiMinh,
            Self::Harare => IanaTimeZone::AfricaHarare,
            Self::Hawaii => IanaTimeZone::PacificHonolulu,
            Self::Helsinki => IanaTimeZone::EuropeHelsinki,
            Self::Hobart => IanaTimeZone::AustraliaHobart,
            Self::HongKong => IanaTimeZone::AsiaHongKong,
            Self::IndianaEast => IanaTimeZone::AmericaIndianaIndianapolis,
            Self::InternationalDateLineWest | Self::MidwayIsland => IanaTimeZone::PacificMidway,
            Self::Irkutsk => IanaTimeZone::AsiaIrkutsk,
            Self::Islamabad | Self::Karachi => IanaTimeZone::AsiaKarachi,
            Self::Israel | Self::Jerusalem => IanaTimeZone::AsiaJerusalem,
            Self::Istanbul | Self::Turkey => IanaTimeZone::EuropeIstanbul,
            Self::Jakarta => IanaTimeZone::AsiaJakarta,
            Self::Kabul => IanaTimeZone::AsiaKabul,
            Self::Kamchatka => IanaTimeZone::AsiaKamchatka,
            Self::Kathmandu => IanaTimeZone::AsiaKathmandu,
            Self::Kiev | Self::Kyiv => IanaTimeZone::EuropeKiev,
            Self::Krasnoyarsk => IanaTimeZone::AsiaKrasnoyarsk,
            Self::KualaLumpur => IanaTimeZone::AsiaKualaLumpur,
            Self::Kuwait => IanaTimeZone::AsiaKuwait,
            Self::LaPaz => IanaTimeZone::AmericaLaPaz,
            Self::Lima => IanaTimeZone::AmericaLima,
            Self::Lisbon => IanaTimeZone::EuropeLisbon,
            Self::Ljubljana => IanaTimeZone::EuropeLjubljana,
            Self::Madrid => IanaTimeZone::EuropeMadrid,
            Self::Magadan | Self::NewCaledonia | Self::Vladivostok => IanaTimeZone::AsiaVladivostok,
            Self::MarshallIs => IanaTimeZone::PacificMajuro,
            Self::Mazatlan => IanaTimeZone::AmericaMazatlan,
            Self::Melbourne => IanaTimeZone::AustraliaMelbourne,
            Self::MidAtlantic => IanaTimeZone::AtlanticSouthGeorgia,
            Self::Minsk => IanaTimeZone::EuropeMinsk,
            Self::Monrovia | Self::WestCentralAfrica => IanaTimeZone::AfricaLagos,
            Self::Monterrey => IanaTimeZone::AmericaMonterrey,
            Self::Moscow | Self::StPetersburg => IanaTimeZone::EuropeMoscow,
            Self::Mountain => IanaTimeZone::AmericaDenver,
            Self::Muscat => IanaTimeZone::AsiaMuscat,
            Self::Nairobi => IanaTimeZone::AfricaNairobi,
            Self::Newfoundland => IanaTimeZone::AmericaStJohns,
            Self::Novosibirsk => IanaTimeZone::AsiaNovosibirsk,
            Self::Osaka | Self::Sapporo | Self::Tokyo => IanaTimeZone::AsiaTokyo,
            Self::Pacific => IanaTimeZone::AmericaLosAngeles,
            Self::Paris => IanaTimeZone::EuropeParis,
            Self::Perth => IanaTimeZone::AustraliaPerth,
            Self::Poland | Self::Warsaw => IanaTimeZone::EuropeWarsaw,
            Self::Prague => IanaTimeZone::EuropePrague,
            Self::Pretoria => IanaTimeZone::AfricaJohannesburg,
            Self::Quito => IanaTimeZone::AmericaGuayaquil,
            Self::Rangoon => IanaTimeZone::AsiaRangoon,
            Self::Riga => IanaTimeZone::EuropeRiga,
            Self::Riyadh => IanaTimeZone::AsiaRiyadh,
            Self::Rome => IanaTimeZone::EuropeRome,
            Self::Santiago => IanaTimeZone::AmericaSantiago,
            Self::Sarajevo => IanaTimeZone::EuropeSarajevo,
            Self::Saskatchewan => IanaTimeZone::AmericaRegina,
            Self::Seoul => IanaTimeZone::AsiaSeoul,
            Self::Singapore => IanaTimeZone::AsiaSingapore,
            Self::Skopje => IanaTimeZone::EuropeSkopje,
            Self::Sofia => IanaTimeZone::EuropeSofia,
            Self::SriJayawardenepura => IanaTimeZone::AsiaColombo,
            Self::Stockholm => IanaTimeZone::EuropeStockholm,
            Self::Taipei => IanaTimeZone::AsiaTaipei,
            Self::Tallinn => IanaTimeZone::EuropeTallinn,
            Self::Tashkent => IanaTimeZone::AsiaTashkent,
            Self::Tbilisi => IanaTimeZone::AsiaTbilisi,
            Self::Tehran => IanaTimeZone::AsiaTehran,
            Self::Tijuana => IanaTimeZone::AmericaTijuana,
            Self::UlaanBataar => IanaTimeZone::AsiaUlaanbaatar,
            Self::Vienna => IanaTimeZone::EuropeVienna,
            Self::Vilnius => IanaTimeZone::EuropeVilnius,
            Self::Volgograd => IanaTimeZone::EuropeVolgograd,
            Self::Yakutsk => IanaTimeZone::AsiaYakutsk,
            Self::Yerevan => IanaTimeZone::AsiaYerevan,
            Self::Zagreb => IanaTimeZone::EuropeZagreb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Abbreviation, Deprecated, IanaTimeZone, Named, TIME_ZONE_VALUES, TimeZone};

    fn parse(label: &str) -> TimeZone {
        serde_json::from_str(&format!("{label:?}")).expect("label should parse")
    }

    #[test]
    fn unknown_label_error_surfaces_value() {
        let error = serde_json::from_str::<TimeZone>(r#""Mars/Olympus_Mons""#)
            .expect_err("an unknown label should fail to deserialize");

        assert!(
            error.to_string().contains("Mars/Olympus_Mons"),
            "error should name the unknown label, got: {error}"
        );
    }

    #[test]
    fn deserializes_each_family() {
        assert_eq!(parse("Asia/Tokyo"), TimeZone::Iana(IanaTimeZone::AsiaTokyo));
        assert_eq!(
            parse("Asia/Calcutta"),
            TimeZone::Deprecated(Deprecated::AsiaCalcutta)
        );
        assert_eq!(parse("JST"), TimeZone::Abbreviation(Abbreviation::Jst));
        assert_eq!(parse("Tokyo"), TimeZone::Named(Named::Tokyo));
    }

    #[test]
    fn every_value_round_trips() {
        for &value in TIME_ZONE_VALUES.iter() {
            let json = serde_json::to_string(&value).expect("should serialize");
            assert_eq!(json, format!("{:?}", value.as_str()), "label for {value:?}");
            let back: TimeZone = serde_json::from_str(&json).expect("should deserialize");
            assert_eq!(back, value, "round-trip for {value:?}");
        }
    }

    #[test]
    fn values_are_unique_and_sorted_by_label() {
        let labels: Vec<&str> = TIME_ZONE_VALUES.iter().map(|tz| tz.as_str()).collect();
        let mut sorted = labels.clone();
        sorted.sort_unstable();
        assert_eq!(labels, sorted, "values must be sorted by label");
        sorted.dedup();
        assert_eq!(sorted.len(), labels.len(), "labels must be unique");
    }

    #[test]
    fn ordered_by_string_representation() {
        assert!(parse("Asia/Tokyo") < parse("Europe/London"));
        assert!(parse("JST") < parse("Tokyo"));
        assert!(parse("GMT+9") < parse("JST"));
    }

    #[test]
    fn distinct_labels_share_one_zone() {
        assert!(parse("Asia/Tokyo").same_zone(parse("JST")));
        assert!(parse("Asia/Tokyo").same_zone(parse("Tokyo")));
        assert!(parse("JST").same_zone(parse("GMT+9")));
        assert!(parse("Tokyo").same_zone(parse("Osaka")));
    }

    #[test]
    fn collapses_deprecated_iana_alias() {
        assert!(parse("Asia/Calcutta").same_zone(parse("Asia/Kolkata")));
        assert_eq!(
            parse("Asia/Calcutta").canonical(),
            IanaTimeZone::AsiaKolkata
        );
        assert!(parse("Asia/Saigon").same_zone(parse("Hanoi")));
    }

    #[test]
    fn unrelated_zones_differ() {
        assert!(!parse("Asia/Tokyo").same_zone(parse("Europe/London")));
        assert!(!parse("PST").same_zone(parse("EST")));
    }
}

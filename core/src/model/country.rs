use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid country code")]
    Invalid(String),
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum Country {
    #[serde(rename = "AD")]
    Andorra,
    #[serde(rename = "AE")]
    UnitedArabEmirates,
    #[serde(rename = "AF")]
    Afghanistan,
    #[serde(rename = "AG")]
    AntiguaAndBarbuda,
    #[serde(rename = "AI")]
    Anguilla,
    #[serde(rename = "AL")]
    Albania,
    #[serde(rename = "AM")]
    Armenia,
    #[serde(rename = "AO")]
    Angola,
    #[serde(rename = "AQ")]
    Antarctica,
    #[serde(rename = "AR")]
    Argentina,
    #[serde(rename = "AS")]
    AmericanSamoa,
    #[serde(rename = "AT")]
    Austria,
    #[serde(rename = "AU")]
    Australia,
    #[serde(rename = "AW")]
    Aruba,
    #[serde(rename = "AZ")]
    Azerbaijan,
    #[serde(rename = "BA")]
    BosniaAndHerzegovina,
    #[serde(rename = "BB")]
    Barbados,
    #[serde(rename = "BD")]
    Bangladesh,
    #[serde(rename = "BE")]
    Belgium,
    #[serde(rename = "BF")]
    BurkinaFaso,
    #[serde(rename = "BG")]
    Bulgaria,
    #[serde(rename = "BH")]
    Bahrain,
    #[serde(rename = "BI")]
    Burundi,
    #[serde(rename = "BJ")]
    Benin,
    #[serde(rename = "BL")]
    SaintBarthelemy,
    #[serde(rename = "BM")]
    Bermuda,
    #[serde(rename = "BN")]
    Brunei,
    #[serde(rename = "BO")]
    Bolivia,
    #[serde(rename = "BQ")]
    CaribbeanNetherlands,
    #[serde(rename = "BR")]
    Brazil,
    #[serde(rename = "BS")]
    Bahamas,
    #[serde(rename = "BT")]
    Bhutan,
    #[serde(rename = "BV")]
    BouvetIsland,
    #[serde(rename = "BW")]
    Botswana,
    #[serde(rename = "BY")]
    Belarus,
    #[serde(rename = "BZ")]
    Belize,
    #[serde(rename = "CA")]
    Canada,
    #[serde(rename = "CD")]
    DemocraticRepublicOfTheCongo,
    #[serde(rename = "CF")]
    CentralAfricanRepublic,
    #[serde(rename = "CG")]
    Congo,
    #[serde(rename = "CH")]
    Switzerland,
    #[serde(rename = "CI")]
    IvoryCoast,
    #[serde(rename = "CK")]
    CookIslands,
    #[serde(rename = "CL")]
    Chile,
    #[serde(rename = "CM")]
    Cameroon,
    #[serde(rename = "CN")]
    China,
    #[serde(rename = "CO")]
    Colombia,
    #[serde(rename = "CR")]
    CostaRica,
    #[serde(rename = "CU")]
    Cuba,
    #[serde(rename = "CV")]
    CapeVerde,
    #[serde(rename = "CW")]
    Curacao,
    #[serde(rename = "CX")]
    ChristmasIsland,
    #[serde(rename = "CY")]
    Cyprus,
    #[serde(rename = "CZ")]
    Czechia,
    #[serde(rename = "DE")]
    Germany,
    #[serde(rename = "DJ")]
    Djibouti,
    #[serde(rename = "DK")]
    Denmark,
    #[serde(rename = "DM")]
    Dominica,
    #[serde(rename = "DO")]
    DominicanRepublic,
    #[serde(rename = "DZ")]
    Algeria,
    #[serde(rename = "EC")]
    Ecuador,
    #[serde(rename = "EE")]
    Estonia,
    #[serde(rename = "EG")]
    Egypt,
    #[serde(rename = "EH")]
    WesternSahara,
    #[serde(rename = "ER")]
    Eritrea,
    #[serde(rename = "ES")]
    Spain,
    #[serde(rename = "ET")]
    Ethiopia,
    #[serde(rename = "FI")]
    Finland,
    #[serde(rename = "FJ")]
    Fiji,
    #[serde(rename = "FK")]
    FalklandIslands,
    #[serde(rename = "FM")]
    Micronesia,
    #[serde(rename = "FO")]
    FaroeIslands,
    #[serde(rename = "FR")]
    France,
    #[serde(rename = "GA")]
    Gabon,
    #[serde(rename = "GB")]
    UnitedKingdom,
    #[serde(rename = "GD")]
    Grenada,
    #[serde(rename = "GE")]
    Georgia,
    #[serde(rename = "GF")]
    FrenchGuiana,
    #[serde(rename = "GG")]
    Guernsey,
    #[serde(rename = "GH")]
    Ghana,
    #[serde(rename = "GI")]
    Gibraltar,
    #[serde(rename = "GL")]
    Greenland,
    #[serde(rename = "GM")]
    Gambia,
    #[serde(rename = "GN")]
    Guinea,
    #[serde(rename = "GP")]
    Guadeloupe,
    #[serde(rename = "GQ")]
    EquatorialGuinea,
    #[serde(rename = "GR")]
    Greece,
    #[serde(rename = "GS")]
    SouthGeorgiaAndTheSouthSandwichIslands,
    #[serde(rename = "GT")]
    Guatemala,
    #[serde(rename = "GU")]
    Guam,
    #[serde(rename = "GW")]
    GuineaBissau,
    #[serde(rename = "GY")]
    Guyana,
    #[serde(rename = "HK")]
    HongKong,
    #[serde(rename = "HN")]
    Honduras,
    #[serde(rename = "HR")]
    Croatia,
    #[serde(rename = "HT")]
    Haiti,
    #[serde(rename = "HU")]
    Hungary,
    #[serde(rename = "ID")]
    Indonesia,
    #[serde(rename = "IE")]
    Ireland,
    #[serde(rename = "IL")]
    Israel,
    #[serde(rename = "IM")]
    IsleOfMan,
    #[serde(rename = "IN")]
    India,
    #[serde(rename = "IO")]
    BritishIndianOceanTerritory,
    #[serde(rename = "IQ")]
    Iraq,
    #[serde(rename = "IR")]
    Iran,
    #[serde(rename = "IS")]
    Iceland,
    #[serde(rename = "IT")]
    Italy,
    #[serde(rename = "JE")]
    Jersey,
    #[serde(rename = "JM")]
    Jamaica,
    #[serde(rename = "JO")]
    Jordan,
    #[serde(rename = "JP")]
    Japan,
    #[serde(rename = "KE")]
    Kenya,
    #[serde(rename = "KG")]
    Kyrgyzstan,
    #[serde(rename = "KH")]
    Cambodia,
    #[serde(rename = "KI")]
    Kiribati,
    #[serde(rename = "KM")]
    Comoros,
    #[serde(rename = "KN")]
    SaintKittsAndNevis,
    #[serde(rename = "KP")]
    NorthKorea,
    #[serde(rename = "KR")]
    Korea,
    #[serde(rename = "KW")]
    Kuwait,
    #[serde(rename = "KY")]
    CaymanIslands,
    #[serde(rename = "KZ")]
    Kazakhstan,
    #[serde(rename = "LA")]
    Laos,
    #[serde(rename = "LB")]
    Lebanon,
    #[serde(rename = "LC")]
    SaintLucia,
    #[serde(rename = "LI")]
    Liechtenstein,
    #[serde(rename = "LK")]
    SriLanka,
    #[serde(rename = "LR")]
    Liberia,
    #[serde(rename = "LS")]
    Lesotho,
    #[serde(rename = "LT")]
    Lithuania,
    #[serde(rename = "LU")]
    Luxembourg,
    #[serde(rename = "LV")]
    Latvia,
    #[serde(rename = "LY")]
    Libya,
    #[serde(rename = "MA")]
    Morocco,
    #[serde(rename = "MC")]
    Monaco,
    #[serde(rename = "MD")]
    Moldova,
    #[serde(rename = "ME")]
    Montenegro,
    #[serde(rename = "MF")]
    SaintMartin,
    #[serde(rename = "MG")]
    Madagascar,
    #[serde(rename = "MH")]
    MarshallIslands,
    #[serde(rename = "MK")]
    Macedonia,
    #[serde(rename = "ML")]
    Mali,
    #[serde(rename = "MM")]
    Myanmar,
    #[serde(rename = "MN")]
    Mongolia,
    #[serde(rename = "MO")]
    Macau,
    #[serde(rename = "MP")]
    NorthernMarianaIslands,
    #[serde(rename = "MQ")]
    Martinique,
    #[serde(rename = "MR")]
    Mauritania,
    #[serde(rename = "MS")]
    Montserrat,
    #[serde(rename = "MT")]
    Malta,
    #[serde(rename = "MU")]
    Mauritius,
    #[serde(rename = "MV")]
    Maldives,
    #[serde(rename = "MW")]
    Malawi,
    #[serde(rename = "MX")]
    Mexico,
    #[serde(rename = "MY")]
    Malaysia,
    #[serde(rename = "MZ")]
    Mozambique,
    #[serde(rename = "NA")]
    Namibia,
    #[serde(rename = "NC")]
    NewCaledonia,
    #[serde(rename = "NE")]
    Niger,
    #[serde(rename = "NG")]
    Nigeria,
    #[serde(rename = "NI")]
    Nicaragua,
    #[serde(rename = "NL")]
    Netherlands,
    #[serde(rename = "NO")]
    Norway,
    #[serde(rename = "NP")]
    Nepal,
    #[serde(rename = "NR")]
    Nauru,
    #[serde(rename = "NU")]
    Niue,
    #[serde(rename = "NZ")]
    NewZealand,
    #[serde(rename = "OM")]
    Oman,
    #[serde(rename = "PA")]
    Panama,
    #[serde(rename = "PE")]
    Peru,
    #[serde(rename = "PF")]
    FrenchPolynesia,
    #[serde(rename = "PG")]
    PapuaNewGuinea,
    #[serde(rename = "PH")]
    Philippines,
    #[serde(rename = "PK")]
    Pakistan,
    #[serde(rename = "PL")]
    Poland,
    #[serde(rename = "PR")]
    PuertoRico,
    #[serde(rename = "PT")]
    Portugal,
    #[serde(rename = "PW")]
    Palau,
    #[serde(rename = "PY")]
    Paraguay,
    #[serde(rename = "QA")]
    Qatar,
    #[serde(rename = "RE")]
    Reunion,
    #[serde(rename = "RO")]
    Romania,
    #[serde(rename = "RS")]
    Serbia,
    #[serde(rename = "RU")]
    Russia,
    #[serde(rename = "RW")]
    Rwanda,
    #[serde(rename = "SA")]
    SaudiArabia,
    #[serde(rename = "SB")]
    SolomonIslands,
    #[serde(rename = "SC")]
    Seychelles,
    #[serde(rename = "SD")]
    Sudan,
    #[serde(rename = "SE")]
    Sweden,
    #[serde(rename = "SG")]
    Singapore,
    #[serde(rename = "SH")]
    SaintHelena,
    #[serde(rename = "SI")]
    Slovenia,
    #[serde(rename = "SK")]
    Slovakia,
    #[serde(rename = "SL")]
    SierraLeone,
    #[serde(rename = "SM")]
    SanMarino,
    #[serde(rename = "SN")]
    Senegal,
    #[serde(rename = "SO")]
    Somalia,
    #[serde(rename = "SR")]
    Suriname,
    #[serde(rename = "ST")]
    SaoTomeAndPrincipe,
    #[serde(rename = "SV")]
    ElSalvador,
    #[serde(rename = "SX")]
    SintMaarten,
    #[serde(rename = "SY")]
    Syria,
    #[serde(rename = "SZ")]
    Eswatini,
    #[serde(rename = "TC")]
    TurksAndCaicosIslands,
    #[serde(rename = "TD")]
    Chad,
    #[serde(rename = "TF")]
    FrenchSouthernTerritories,
    #[serde(rename = "TG")]
    Togo,
    #[serde(rename = "TH")]
    Thailand,
    #[serde(rename = "TJ")]
    Tajikistan,
    #[serde(rename = "TK")]
    Tokelau,
    #[serde(rename = "TL")]
    EastTimor,
    #[serde(rename = "TM")]
    Turkmenistan,
    #[serde(rename = "TN")]
    Tunisia,
    #[serde(rename = "TO")]
    Tonga,
    #[serde(rename = "TR")]
    Turkey,
    #[serde(rename = "TT")]
    TrinidadAndTobago,
    #[serde(rename = "TV")]
    Tuvalu,
    #[serde(rename = "TW")]
    Taiwan,
    #[serde(rename = "TZ")]
    Tanzania,
    #[serde(rename = "UA")]
    Ukraine,
    #[serde(rename = "UG")]
    Uganda,
    #[serde(rename = "UM")]
    UnitedStatesMinorOutlyingIslands,
    #[serde(rename = "US")]
    UnitedStates,
    #[serde(rename = "UY")]
    Uruguay,
    #[serde(rename = "UZ")]
    Uzbekistan,
    #[serde(rename = "VA")]
    VaticanCity,
    #[serde(rename = "VC")]
    SaintVincentAndTheGrenadines,
    #[serde(rename = "VE")]
    Venezuela,
    #[serde(rename = "VG")]
    BritishVirginIslands,
    #[serde(rename = "VI")]
    VirginIslands,
    #[serde(rename = "VN")]
    Vietnam,
    #[serde(rename = "VU")]
    Vanuatu,
    #[serde(rename = "WS")]
    Samoa,
    #[serde(rename = "XK")]
    Kosovo,
    #[serde(rename = "XX")]
    All,
    #[serde(rename = "XY")]
    Copyright,
    #[serde(rename = "YE")]
    Yemen,
    #[serde(rename = "YT")]
    Mayotte,
    #[serde(rename = "ZA")]
    SouthAfrica,
    #[serde(rename = "ZM")]
    Zambia,
    #[serde(rename = "ZW")]
    Zimbabwe,
}

pub const COUNTRY_VALUES: [Country; 242] = [
    Country::Andorra,
    Country::UnitedArabEmirates,
    Country::Afghanistan,
    Country::AntiguaAndBarbuda,
    Country::Anguilla,
    Country::Albania,
    Country::Armenia,
    Country::Angola,
    Country::Antarctica,
    Country::Argentina,
    Country::AmericanSamoa,
    Country::Austria,
    Country::Australia,
    Country::Aruba,
    Country::Azerbaijan,
    Country::BosniaAndHerzegovina,
    Country::Barbados,
    Country::Bangladesh,
    Country::Belgium,
    Country::BurkinaFaso,
    Country::Bulgaria,
    Country::Bahrain,
    Country::Burundi,
    Country::Benin,
    Country::SaintBarthelemy,
    Country::Bermuda,
    Country::Brunei,
    Country::Bolivia,
    Country::CaribbeanNetherlands,
    Country::Brazil,
    Country::Bahamas,
    Country::Bhutan,
    Country::BouvetIsland,
    Country::Botswana,
    Country::Belarus,
    Country::Belize,
    Country::Canada,
    Country::DemocraticRepublicOfTheCongo,
    Country::CentralAfricanRepublic,
    Country::Congo,
    Country::Switzerland,
    Country::IvoryCoast,
    Country::CookIslands,
    Country::Chile,
    Country::Cameroon,
    Country::China,
    Country::Colombia,
    Country::CostaRica,
    Country::Cuba,
    Country::CapeVerde,
    Country::Curacao,
    Country::ChristmasIsland,
    Country::Cyprus,
    Country::Czechia,
    Country::Germany,
    Country::Djibouti,
    Country::Denmark,
    Country::Dominica,
    Country::DominicanRepublic,
    Country::Algeria,
    Country::Ecuador,
    Country::Estonia,
    Country::Egypt,
    Country::WesternSahara,
    Country::Eritrea,
    Country::Spain,
    Country::Ethiopia,
    Country::Finland,
    Country::Fiji,
    Country::FalklandIslands,
    Country::Micronesia,
    Country::FaroeIslands,
    Country::France,
    Country::Gabon,
    Country::UnitedKingdom,
    Country::Grenada,
    Country::Georgia,
    Country::FrenchGuiana,
    Country::Guernsey,
    Country::Ghana,
    Country::Gibraltar,
    Country::Greenland,
    Country::Gambia,
    Country::Guinea,
    Country::Guadeloupe,
    Country::EquatorialGuinea,
    Country::Greece,
    Country::SouthGeorgiaAndTheSouthSandwichIslands,
    Country::Guatemala,
    Country::Guam,
    Country::GuineaBissau,
    Country::Guyana,
    Country::HongKong,
    Country::Honduras,
    Country::Croatia,
    Country::Haiti,
    Country::Hungary,
    Country::Indonesia,
    Country::Ireland,
    Country::Israel,
    Country::IsleOfMan,
    Country::India,
    Country::BritishIndianOceanTerritory,
    Country::Iraq,
    Country::Iran,
    Country::Iceland,
    Country::Italy,
    Country::Jersey,
    Country::Jamaica,
    Country::Jordan,
    Country::Japan,
    Country::Kenya,
    Country::Kyrgyzstan,
    Country::Cambodia,
    Country::Kiribati,
    Country::Comoros,
    Country::SaintKittsAndNevis,
    Country::NorthKorea,
    Country::Korea,
    Country::Kuwait,
    Country::CaymanIslands,
    Country::Kazakhstan,
    Country::Laos,
    Country::Lebanon,
    Country::SaintLucia,
    Country::Liechtenstein,
    Country::SriLanka,
    Country::Liberia,
    Country::Lesotho,
    Country::Lithuania,
    Country::Luxembourg,
    Country::Latvia,
    Country::Libya,
    Country::Morocco,
    Country::Monaco,
    Country::Moldova,
    Country::Montenegro,
    Country::SaintMartin,
    Country::Madagascar,
    Country::MarshallIslands,
    Country::Macedonia,
    Country::Mali,
    Country::Myanmar,
    Country::Mongolia,
    Country::Macau,
    Country::NorthernMarianaIslands,
    Country::Martinique,
    Country::Mauritania,
    Country::Montserrat,
    Country::Malta,
    Country::Mauritius,
    Country::Maldives,
    Country::Malawi,
    Country::Mexico,
    Country::Malaysia,
    Country::Mozambique,
    Country::Namibia,
    Country::NewCaledonia,
    Country::Niger,
    Country::Nigeria,
    Country::Nicaragua,
    Country::Netherlands,
    Country::Norway,
    Country::Nepal,
    Country::Nauru,
    Country::Niue,
    Country::NewZealand,
    Country::Oman,
    Country::Panama,
    Country::Peru,
    Country::FrenchPolynesia,
    Country::PapuaNewGuinea,
    Country::Philippines,
    Country::Pakistan,
    Country::Poland,
    Country::PuertoRico,
    Country::Portugal,
    Country::Palau,
    Country::Paraguay,
    Country::Qatar,
    Country::Reunion,
    Country::Romania,
    Country::Serbia,
    Country::Russia,
    Country::Rwanda,
    Country::SaudiArabia,
    Country::SolomonIslands,
    Country::Seychelles,
    Country::Sudan,
    Country::Sweden,
    Country::Singapore,
    Country::SaintHelena,
    Country::Slovenia,
    Country::Slovakia,
    Country::SierraLeone,
    Country::SanMarino,
    Country::Senegal,
    Country::Somalia,
    Country::Suriname,
    Country::SaoTomeAndPrincipe,
    Country::ElSalvador,
    Country::SintMaarten,
    Country::Syria,
    Country::Eswatini,
    Country::TurksAndCaicosIslands,
    Country::Chad,
    Country::FrenchSouthernTerritories,
    Country::Togo,
    Country::Thailand,
    Country::Tajikistan,
    Country::Tokelau,
    Country::EastTimor,
    Country::Turkmenistan,
    Country::Tunisia,
    Country::Tonga,
    Country::Turkey,
    Country::TrinidadAndTobago,
    Country::Tuvalu,
    Country::Taiwan,
    Country::Tanzania,
    Country::Ukraine,
    Country::Uganda,
    Country::UnitedStatesMinorOutlyingIslands,
    Country::UnitedStates,
    Country::Uruguay,
    Country::Uzbekistan,
    Country::VaticanCity,
    Country::SaintVincentAndTheGrenadines,
    Country::Venezuela,
    Country::BritishVirginIslands,
    Country::VirginIslands,
    Country::Vietnam,
    Country::Vanuatu,
    Country::Samoa,
    Country::Kosovo,
    Country::All,
    Country::Copyright,
    Country::Yemen,
    Country::Mayotte,
    Country::SouthAfrica,
    Country::Zambia,
    Country::Zimbabwe,
];

static COUNTRY_CODE_MAP: LazyLock<BTreeMap<String, Country>> = LazyLock::new(|| {
    COUNTRY_VALUES
        .iter()
        .map(|value| (value.to_string(), *value))
        .collect()
});

impl FromStr for Country {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        COUNTRY_CODE_MAP
            .get(s)
            .ok_or_else(|| Error::Invalid(s.to_string()))
            .copied()
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde::ser::Serialize::serialize(self, f)
    }
}

/// A possible country code that may be an empty string to represent an absent value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PossibleCountry(pub Option<Country>);

impl Display for PossibleCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde::ser::Serialize::serialize(self, f)
    }
}

mod internal {
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    pub(super) enum PossibleCountry {
        Empty(PossibleCountryEmpty),
        Country(super::Country),
    }

    #[derive(serde::Deserialize)]
    pub(super) enum PossibleCountryEmpty {
        #[serde(rename = "")]
        Empty,
    }
}

impl<'de> serde::Deserialize<'de> for PossibleCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        internal::PossibleCountry::deserialize(deserializer).map(|country| match country {
            internal::PossibleCountry::Empty(_) => Self(None),
            internal::PossibleCountry::Country(country) => Self(Some(country)),
        })
    }
}

impl serde::ser::Serialize for PossibleCountry {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Some(country) => country.serialize(serializer),
            None => serializer.serialize_str(""),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn variants_sorted() {
        let mut sorted = super::COUNTRY_VALUES.to_vec();

        sorted.sort_by_key(|country| serde_json::json!(country).to_string());

        assert_eq!(sorted, super::COUNTRY_VALUES);
    }

    #[test]
    fn values_sorted() {
        let mut sorted = super::COUNTRY_VALUES.to_vec();

        sorted.sort();

        assert_eq!(sorted, super::COUNTRY_VALUES);
    }

    #[test]
    fn round_trip_country() {
        for value in super::COUNTRY_VALUES {
            let as_string = value.to_string();
            let parsed = as_string.parse().unwrap();

            assert_eq!(value, parsed);
        }
    }
}

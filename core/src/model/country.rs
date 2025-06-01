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
    #[serde(rename = "AQ")]
    Antarctica,
    #[serde(rename = "AR")]
    Argentina,
    #[serde(rename = "AT")]
    Austria,
    #[serde(rename = "AU")]
    Australia,
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
    #[serde(rename = "BG")]
    Bulgaria,
    #[serde(rename = "BH")]
    Bahrain,
    #[serde(rename = "BL")]
    SaintBarthelemy,
    #[serde(rename = "BN")]
    Brunei,
    #[serde(rename = "BO")]
    Bolivia,
    #[serde(rename = "BR")]
    Brazil,
    #[serde(rename = "BS")]
    Bahamas,
    #[serde(rename = "BW")]
    Botswana,
    #[serde(rename = "BY")]
    Belarus,
    #[serde(rename = "BZ")]
    Belize,
    #[serde(rename = "CA")]
    Canada,
    #[serde(rename = "CD")]
    Congo,
    #[serde(rename = "CH")]
    Switzerland,
    #[serde(rename = "CI")]
    IvoryCoast,
    #[serde(rename = "CM")]
    Cameroon,
    #[serde(rename = "CN")]
    China,
    #[serde(rename = "CL")]
    Chile,
    #[serde(rename = "CO")]
    Colombia,
    #[serde(rename = "CR")]
    CostaRica,
    #[serde(rename = "CU")]
    Cuba,
    #[serde(rename = "CY")]
    Cyprus,
    #[serde(rename = "CZ")]
    Czechia,
    #[serde(rename = "DE")]
    Germany,
    #[serde(rename = "DK")]
    Denmark,
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
    #[serde(rename = "ES")]
    Spain,
    #[serde(rename = "ET")]
    Ethiopia,
    #[serde(rename = "FI")]
    Finland,
    #[serde(rename = "FJ")]
    Fiji,
    #[serde(rename = "FR")]
    France,
    #[serde(rename = "GB")]
    UnitedKingdom,
    #[serde(rename = "GE")]
    Georgia,
    #[serde(rename = "GH")]
    Ghana,
    #[serde(rename = "GI")]
    Gibraltar,
    #[serde(rename = "GM")]
    Gambia,
    #[serde(rename = "GR")]
    Greece,
    #[serde(rename = "GT")]
    Guatemala,
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
    #[serde(rename = "IQ")]
    Iraq,
    #[serde(rename = "IR")]
    Iran,
    #[serde(rename = "IS")]
    Iceland,
    #[serde(rename = "IT")]
    Italy,
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
    #[serde(rename = "LB")]
    Lebanon,
    #[serde(rename = "LC")]
    SaintLucia,
    #[serde(rename = "LK")]
    SriLanka,
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
    #[serde(rename = "ME")]
    Montenegro,
    #[serde(rename = "MG")]
    Madagascar,
    #[serde(rename = "MK")]
    Macedonia,
    #[serde(rename = "MM")]
    Myanmar,
    #[serde(rename = "MO")]
    Macau,
    #[serde(rename = "MR")]
    Mauritania,
    #[serde(rename = "MT")]
    Malta,
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
    #[serde(rename = "NZ")]
    NewZealand,
    #[serde(rename = "OM")]
    Oman,
    #[serde(rename = "PA")]
    Panama,
    #[serde(rename = "PE")]
    Peru,
    #[serde(rename = "PG")]
    PapuaNewGuinea,
    #[serde(rename = "PH")]
    Philippines,
    #[serde(rename = "PK")]
    Pakistan,
    #[serde(rename = "PL")]
    Poland,
    #[serde(rename = "PT")]
    Portugal,
    #[serde(rename = "PY")]
    Paraguay,
    #[serde(rename = "QA")]
    Qatar,
    #[serde(rename = "RU")]
    Russia,
    #[serde(rename = "RO")]
    Romania,
    #[serde(rename = "RS")]
    Serbia,
    #[serde(rename = "RW")]
    Rwanda,
    #[serde(rename = "SA")]
    SaudiArabia,
    #[serde(rename = "SD")]
    Sudan,
    #[serde(rename = "SE")]
    Sweden,
    #[serde(rename = "SG")]
    Singapore,
    #[serde(rename = "SI")]
    Slovenia,
    #[serde(rename = "SK")]
    Slovakia,
    #[serde(rename = "SN")]
    Senegal,
    #[serde(rename = "SO")]
    Somalia,
    #[serde(rename = "SV")]
    ElSalvador,
    #[serde(rename = "SY")]
    Syria,
    #[serde(rename = "SX")]
    SintMaarten,
    #[serde(rename = "TC")]
    TurksAndCaicosIslands,
    #[serde(rename = "TH")]
    Thailand,
    #[serde(rename = "TN")]
    Tunisia,
    #[serde(rename = "TR")]
    Turkey,
    #[serde(rename = "TT")]
    TrinidadAndTobago,
    #[serde(rename = "TW")]
    Taiwan,
    #[serde(rename = "TZ")]
    Tanzania,
    #[serde(rename = "UA")]
    Ukraine,
    #[serde(rename = "UG")]
    Uganda,
    #[serde(rename = "US")]
    UnitedStates,
    #[serde(rename = "UY")]
    Uruguay,
    #[serde(rename = "VA")]
    VaticanCity,
    #[serde(rename = "VE")]
    Venezuela,
    #[serde(rename = "VI")]
    VirginIslands,
    #[serde(rename = "VN")]
    Vietnam,
    #[serde(rename = "VU")]
    Vanuatu,
    #[serde(rename = "ZA")]
    SouthAfrica,
    #[serde(rename = "XK")]
    Kosovo,
    #[serde(rename = "XX")]
    Xx,
    #[serde(rename = "ZW")]
    Zimbabwe,
}

pub const COUNTRY_VALUES: [Country; 144] = [
    Country::UnitedArabEmirates,
    Country::Afghanistan,
    Country::AntiguaAndBarbuda,
    Country::Anguilla,
    Country::Albania,
    Country::Antarctica,
    Country::Argentina,
    Country::Austria,
    Country::Australia,
    Country::Azerbaijan,
    Country::BosniaAndHerzegovina,
    Country::Barbados,
    Country::Bangladesh,
    Country::Belgium,
    Country::Bulgaria,
    Country::Bahrain,
    Country::SaintBarthelemy,
    Country::Brunei,
    Country::Bolivia,
    Country::Brazil,
    Country::Bahamas,
    Country::Botswana,
    Country::Belarus,
    Country::Belize,
    Country::Canada,
    Country::Congo,
    Country::Switzerland,
    Country::IvoryCoast,
    Country::Cameroon,
    Country::China,
    Country::Chile,
    Country::Colombia,
    Country::CostaRica,
    Country::Cuba,
    Country::Cyprus,
    Country::Czechia,
    Country::Germany,
    Country::Denmark,
    Country::DominicanRepublic,
    Country::Algeria,
    Country::Ecuador,
    Country::Estonia,
    Country::Egypt,
    Country::Spain,
    Country::Ethiopia,
    Country::Finland,
    Country::France,
    Country::UnitedKingdom,
    Country::Georgia,
    Country::Ghana,
    Country::Gibraltar,
    Country::Gambia,
    Country::Greece,
    Country::Guatemala,
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
    Country::Iraq,
    Country::Iran,
    Country::Iceland,
    Country::Italy,
    Country::Jamaica,
    Country::Jordan,
    Country::Japan,
    Country::Kenya,
    Country::Cambodia,
    Country::NorthKorea,
    Country::Korea,
    Country::Kuwait,
    Country::CaymanIslands,
    Country::Kazakhstan,
    Country::Lebanon,
    Country::SaintLucia,
    Country::SriLanka,
    Country::Lithuania,
    Country::Luxembourg,
    Country::Latvia,
    Country::Morocco,
    Country::Monaco,
    Country::Montenegro,
    Country::Madagascar,
    Country::Macedonia,
    Country::Myanmar,
    Country::Macau,
    Country::Malta,
    Country::Mexico,
    Country::Malaysia,
    Country::Mozambique,
    Country::Namibia,
    Country::Nigeria,
    Country::Nicaragua,
    Country::Netherlands,
    Country::Norway,
    Country::Nepal,
    Country::NewZealand,
    Country::Oman,
    Country::Panama,
    Country::Peru,
    Country::PapuaNewGuinea,
    Country::Philippines,
    Country::Pakistan,
    Country::Poland,
    Country::Portugal,
    Country::Paraguay,
    Country::Qatar,
    Country::Russia,
    Country::Romania,
    Country::Serbia,
    Country::Rwanda,
    Country::SaudiArabia,
    Country::Sudan,
    Country::Sweden,
    Country::Singapore,
    Country::Slovenia,
    Country::Slovakia,
    Country::Senegal,
    Country::ElSalvador,
    Country::Syria,
    Country::TurksAndCaicosIslands,
    Country::Thailand,
    Country::Tunisia,
    Country::Turkey,
    Country::TrinidadAndTobago,
    Country::Taiwan,
    Country::Tanzania,
    Country::Ukraine,
    Country::Uganda,
    Country::UnitedStates,
    Country::Uruguay,
    Country::VaticanCity,
    Country::Venezuela,
    Country::Vietnam,
    Country::Vanuatu,
    Country::SouthAfrica,
    Country::Kosovo,
    Country::Xx,
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

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum PossibleCountryInternal {
    Empty(PossibleCountryEmptyInternal),
    Country(Country),
}

#[derive(serde::Deserialize)]
enum PossibleCountryEmptyInternal {
    #[serde(rename = "")]
    Empty,
}

impl<'de> serde::Deserialize<'de> for PossibleCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        PossibleCountryInternal::deserialize(deserializer).map(|country| match country {
            PossibleCountryInternal::Empty(_) => Self(None),
            PossibleCountryInternal::Country(country) => Self(Some(country)),
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
    fn sorted() {
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

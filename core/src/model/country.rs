#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Country {
    #[serde(rename = "AE")]
    UnitedArabEmirates,
    #[serde(rename = "AF")]
    Afghanistan,
    #[serde(rename = "AR")]
    Argentina,
    #[serde(rename = "AQ")]
    Antarctica,
    #[serde(rename = "AT")]
    Austria,
    #[serde(rename = "AU")]
    Australia,
    #[serde(rename = "BE")]
    Belgium,
    #[serde(rename = "BG")]
    Bulgaria,
    #[serde(rename = "BR")]
    Brazil,
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
    #[serde(rename = "EC")]
    Ecuador,
    #[serde(rename = "EE")]
    Estonia,
    #[serde(rename = "ES")]
    Spain,
    #[serde(rename = "FI")]
    Finland,
    #[serde(rename = "FR")]
    France,
    #[serde(rename = "GB")]
    UnitedKingdom,
    #[serde(rename = "GR")]
    Greece,
    #[serde(rename = "HK")]
    HongKong,
    #[serde(rename = "HR")]
    Croatia,
    #[serde(rename = "HU")]
    Hungary,
    #[serde(rename = "ID")]
    Indonesia,
    #[serde(rename = "IE")]
    Ireland,
    #[serde(rename = "IL")]
    Israel,
    #[serde(rename = "IN")]
    India,
    #[serde(rename = "IS")]
    Iceland,
    #[serde(rename = "IT")]
    Italy,
    #[serde(rename = "JM")]
    Jamaica,
    #[serde(rename = "JP")]
    Japan,
    #[serde(rename = "KE")]
    Kenya,
    #[serde(rename = "KH")]
    Cambodia,
    #[serde(rename = "KP")]
    NorthKorea,
    #[serde(rename = "KR")]
    Korea,
    #[serde(rename = "KW")]
    Kuwait,
    #[serde(rename = "LB")]
    Lebanon,
    #[serde(rename = "LT")]
    Lithuania,
    #[serde(rename = "LU")]
    Luxembourg,
    #[serde(rename = "LV")]
    Latvia,
    #[serde(rename = "MA")]
    Morocco,
    #[serde(rename = "MM")]
    Myanmar,
    #[serde(rename = "MT")]
    Malta,
    #[serde(rename = "MX")]
    Mexico,
    #[serde(rename = "MY")]
    Malaysia,
    #[serde(rename = "NG")]
    Nigeria,
    #[serde(rename = "NL")]
    Netherlands,
    #[serde(rename = "NO")]
    Norway,
    #[serde(rename = "NZ")]
    NewZealand,
    #[serde(rename = "PK")]
    Pakistan,
    #[serde(rename = "PH")]
    Philippines,
    #[serde(rename = "PL")]
    Poland,
    #[serde(rename = "PT")]
    Portugal,
    #[serde(rename = "PY")]
    Paraguay,
    #[serde(rename = "RU")]
    Russia,
    #[serde(rename = "RO")]
    Romania,
    #[serde(rename = "RS")]
    Serbia,
    #[serde(rename = "SA")]
    SaudiArabia,
    #[serde(rename = "SE")]
    Sweden,
    #[serde(rename = "SG")]
    Singapore,
    #[serde(rename = "SI")]
    Slovenia,
    #[serde(rename = "SK")]
    Slovakia,
    #[serde(rename = "SY")]
    Syria,
    #[serde(rename = "TC")]
    TurksAndCaicosIslands,
    #[serde(rename = "TH")]
    Thailand,
    #[serde(rename = "TR")]
    Turkey,
    #[serde(rename = "UA")]
    Ukraine,
    #[serde(rename = "UG")]
    Uganda,
    #[serde(rename = "US")]
    UnitedStates,
    #[serde(rename = "UY")]
    Uruguay,
    #[serde(rename = "VN")]
    Vietnam,
    #[serde(rename = "ZA")]
    SouthAfrica,
    #[serde(rename = "XX")]
    Xx,
}

/// A possible country code that may be an empty string to represent an absent value.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PossibleCountry(pub Option<Country>);

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

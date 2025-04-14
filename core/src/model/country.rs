#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Country {
    #[serde(rename = "AE")]
    UnitedArabEmirates,
    #[serde(rename = "AF")]
    Afghanistan,
    #[serde(rename = "AR")]
    Argentina,
    #[serde(rename = "AL")]
    Albania,
    #[serde(rename = "AQ")]
    Antarctica,
    #[serde(rename = "AT")]
    Austria,
    #[serde(rename = "AU")]
    Australia,
    #[serde(rename = "BB")]
    Barbados,
    #[serde(rename = "BE")]
    Belgium,
    #[serde(rename = "BG")]
    Bulgaria,
    #[serde(rename = "BL")]
    SaintBarthelemy,
    #[serde(rename = "BO")]
    Bolivia,
    #[serde(rename = "BR")]
    Brazil,
    #[serde(rename = "BS")]
    Bahamas,
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
    #[serde(rename = "FI")]
    Finland,
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
    #[serde(rename = "LB")]
    Lebanon,
    #[serde(rename = "LC")]
    SaintLucia,
    #[serde(rename = "LT")]
    Lithuania,
    #[serde(rename = "LU")]
    Luxembourg,
    #[serde(rename = "LV")]
    Latvia,
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
    #[serde(rename = "NG")]
    Nigeria,
    #[serde(rename = "NI")]
    Nicaragua,
    #[serde(rename = "NL")]
    Netherlands,
    #[serde(rename = "NO")]
    Norway,
    #[serde(rename = "NZ")]
    NewZealand,
    #[serde(rename = "PA")]
    Panama,
    #[serde(rename = "PE")]
    Peru,
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
    #[serde(rename = "SV")]
    ElSalvador,
    #[serde(rename = "SY")]
    Syria,
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
    #[serde(rename = "VN")]
    Vietnam,
    #[serde(rename = "ZA")]
    SouthAfrica,
    #[serde(rename = "XK")]
    Kosovo,
    #[serde(rename = "XX")]
    Xx,
    #[serde(rename = "ZW")]
    Zimbabwe,
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

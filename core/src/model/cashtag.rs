use serde::de::{Deserializer, Visitor};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

/// May be a stock symbol, a crypto abbreviation, or an arbitrary sequence of letters.
///
/// In the case of non-standard casing (i.e. not fully capitalized) we preserve the original form
/// in order to be able to round-trip source JSON.
#[derive(Clone, Debug)]
pub enum Cashtag {
    Symbol(CashtagSymbol),
    NonStandardSymbol { symbol: CashtagSymbol, form: String },
    Other(String),
}

impl Cashtag {
    #[must_use]
    pub const fn symbol(&self) -> Option<CashtagSymbol> {
        match self {
            Self::Symbol(symbol) | Self::NonStandardSymbol { symbol, .. } => Some(*symbol),
            Self::Other(_) => None,
        }
    }

    #[must_use]
    fn form(&self) -> &str {
        match self {
            Self::Symbol(symbol) => symbol.as_str(),
            Self::NonStandardSymbol { form, .. } | Self::Other(form) => form,
        }
    }
}

impl PartialEq for Cashtag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Symbol(symbol), Self::Symbol(other_symbol)) => symbol.eq(other_symbol),
            _ => self.form().eq(other.form()),
        }
    }
}

impl Eq for Cashtag {}

impl PartialOrd for Cashtag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cashtag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.form().cmp(other.form())
    }
}

impl Display for Cashtag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.form().fmt(f)
    }
}

impl<'de> serde::de::Deserialize<'de> for Cashtag {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CashtagVisitor;

        impl Visitor<'_> for CashtagVisitor {
            type Value = Cashtag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct Cashtag")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v.chars()
                    .any(|char| !char.is_ascii_uppercase() && char != '.')
                {
                    let value = v.to_ascii_uppercase();

                    Ok(CashtagSymbol::from_uppercase_str(&value).map_or_else(
                        || Self::Value::Other(v.to_string()),
                        |symbol| Self::Value::NonStandardSymbol {
                            symbol,
                            form: v.to_string(),
                        },
                    ))
                } else {
                    Ok(CashtagSymbol::from_uppercase_str(v)
                        .map_or_else(|| Self::Value::Other(v.to_string()), Self::Value::Symbol))
                }
            }
        }

        deserializer.deserialize_str(CashtagVisitor)
    }
}

impl serde::ser::Serialize for Cashtag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid case")]
    InvalidCase,
    #[error("Unknown symbol")]
    UnknownSymbol,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CashtagSymbol {
    Stock(StockSymbol),
    Crypto(CryptoSymbol),
}

static CASHTAG_SYMBOL_VALUES: LazyLock<Vec<CashtagSymbol>> = LazyLock::new(|| {
    let mut values = Vec::with_capacity(STOCK_SYMBOL_MAPPINGS.len() + CRYPTO_SYMBOL_MAPPINGS.len());

    values.extend(
        STOCK_SYMBOL_MAPPINGS
            .iter()
            .map(|(symbol, _)| CashtagSymbol::Stock(*symbol)),
    );

    values.extend(
        CRYPTO_SYMBOL_MAPPINGS
            .iter()
            .map(|(symbol, _)| CashtagSymbol::Crypto(*symbol)),
    );

    values.sort_unstable();

    values
});

impl CashtagSymbol {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Stock(symbol) => symbol.as_str(),
            Self::Crypto(symbol) => symbol.as_str(),
        }
    }

    #[must_use]
    pub fn values() -> &'static [Self] {
        &CASHTAG_SYMBOL_VALUES
    }

    // Assumes we've already checked the case.
    fn from_uppercase_str(s: &str) -> Option<Self> {
        s.parse::<StockSymbol>()
            .map(Self::Stock)
            .or_else(|_| s.parse::<CryptoSymbol>().map(Self::Crypto))
            .ok()
    }
}

impl PartialOrd for CashtagSymbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CashtagSymbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl FromStr for CashtagSymbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars()
            .any(|char| !char.is_ascii_uppercase() && char != '.')
        {
            Err(Error::InvalidCase)
        } else {
            Self::from_uppercase_str(s).ok_or(Error::UnknownSymbol)
        }
    }
}

impl Display for CashtagSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stock(symbol) => symbol.fmt(f),
            Self::Crypto(symbol) => symbol.fmt(f),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StockSymbol {
    Apple,
    Amc,
    Amazon,
    Boeing,
    BankOfAmerica,
    Baidu,
    Citi,
    Costco,
    CrowdStrike,
    TrumpMedia,
    Facebook,
    GeneralMotors,
    GameStop,
    Google,
    Intel,
    JpMorgan,
    McDonalds,
    Meta,
    MorganStanley,
    Microsoft,
    Nvidia,
    Palantir,
    InvescoQqq,
    Rivian,
    SAndP500,
    Tesla,
    Twitter,
    WellsFargo,
    Walmart,
}

const STOCK_SYMBOL_MAPPINGS: [(StockSymbol, &str); 29] = [
    (StockSymbol::Apple, "AAPL"),
    (StockSymbol::Amc, "AMC"),
    (StockSymbol::Amazon, "AMZN"),
    (StockSymbol::Boeing, "BA"),
    (StockSymbol::BankOfAmerica, "BAC"),
    (StockSymbol::Baidu, "BIDU"),
    (StockSymbol::Citi, "C"),
    (StockSymbol::Costco, "COST"),
    (StockSymbol::CrowdStrike, "CRWD"),
    (StockSymbol::TrumpMedia, "DJT"),
    (StockSymbol::Facebook, "FB"),
    (StockSymbol::GeneralMotors, "GM"),
    (StockSymbol::GameStop, "GME"),
    (StockSymbol::Google, "GOOGL"),
    (StockSymbol::Intel, "INTC"),
    (StockSymbol::JpMorgan, "JPM"),
    (StockSymbol::McDonalds, "MCD"),
    (StockSymbol::Meta, "META"),
    (StockSymbol::MorganStanley, "MS"),
    (StockSymbol::Microsoft, "MSFT"),
    (StockSymbol::Nvidia, "NVDA"),
    (StockSymbol::Palantir, "PLTR"),
    (StockSymbol::InvescoQqq, "QQQ"),
    (StockSymbol::Rivian, "RIVN"),
    (StockSymbol::SAndP500, "SPX"),
    (StockSymbol::Tesla, "TSLA"),
    (StockSymbol::Twitter, "TWTR"),
    (StockSymbol::WellsFargo, "WFC"),
    (StockSymbol::Walmart, "WMT"),
];

impl StockSymbol {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        STOCK_SYMBOL_MAPPINGS[*self as usize].1
    }

    pub fn values() -> impl Iterator<Item = Self> {
        STOCK_SYMBOL_MAPPINGS.iter().map(|(symbol, _)| *symbol)
    }
}

static STOCK_SYMBOL_FROM: LazyLock<BTreeMap<String, StockSymbol>> = LazyLock::new(|| {
    STOCK_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, str_repr)| ((*str_repr).to_string(), *symbol))
        .collect()
});

impl FromStr for StockSymbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        STOCK_SYMBOL_FROM
            .get(s)
            .copied()
            .ok_or(Error::UnknownSymbol)
    }
}

impl Display for StockSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CryptoSymbol {
    Cardano,
    Bitcoin,
    DaddyTate,
    Degen,
    Doge,
    Eth,
    Litecoin,
    Terra,
    Pepe,
    ShibaInu,
    Solana,
    UsdCoin,
    Tether,
    Dogwifhat,
    Monero,
    Ripple,
}

const CRYPTO_SYMBOL_MAPPINGS: [(CryptoSymbol, &str); 16] = [
    (CryptoSymbol::Cardano, "ADA"),
    (CryptoSymbol::Bitcoin, "BTC"),
    (CryptoSymbol::DaddyTate, "DADDY"),
    (CryptoSymbol::Degen, "DEGEN"),
    (CryptoSymbol::Doge, "DOGE"),
    (CryptoSymbol::Eth, "ETH"),
    (CryptoSymbol::Litecoin, "LTC"),
    (CryptoSymbol::Terra, "LUNA"),
    (CryptoSymbol::Pepe, "PEPE"),
    (CryptoSymbol::ShibaInu, "SHIB"),
    (CryptoSymbol::Solana, "SOL"),
    (CryptoSymbol::UsdCoin, "USDC"),
    (CryptoSymbol::Tether, "USDT"),
    (CryptoSymbol::Dogwifhat, "WIF"),
    (CryptoSymbol::Monero, "XMR"),
    (CryptoSymbol::Ripple, "XRP"),
];

impl CryptoSymbol {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        CRYPTO_SYMBOL_MAPPINGS[*self as usize].1
    }

    pub fn values() -> impl Iterator<Item = Self> {
        CRYPTO_SYMBOL_MAPPINGS.iter().map(|(symbol, _)| *symbol)
    }
}

static CRYPTO_SYMBOL_FROM: LazyLock<BTreeMap<String, CryptoSymbol>> = LazyLock::new(|| {
    CRYPTO_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, str_repr)| ((*str_repr).to_string(), *symbol))
        .collect()
});

impl FromStr for CryptoSymbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CRYPTO_SYMBOL_FROM
            .get(s)
            .copied()
            .ok_or(Error::UnknownSymbol)
    }
}

impl Display for CryptoSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_symbol_mappings_order_matches_enum_order() {
        let from_values: Vec<StockSymbol> = StockSymbol::values().collect();
        let from_mappings: Vec<StockSymbol> = STOCK_SYMBOL_MAPPINGS
            .iter()
            .map(|(symbol, _)| *symbol)
            .collect();

        assert_eq!(
            from_values, from_mappings,
            "StockSymbol::values() order must match STOCK_SYMBOL_MAPPINGS order"
        );
    }

    #[test]
    fn test_crypto_symbol_mappings_order_matches_enum_order() {
        let from_values: Vec<CryptoSymbol> = CryptoSymbol::values().collect();
        let from_mappings: Vec<CryptoSymbol> = CRYPTO_SYMBOL_MAPPINGS
            .iter()
            .map(|(symbol, _)| *symbol)
            .collect();

        assert_eq!(
            from_values, from_mappings,
            "CryptoSymbol::values() order must match CRYPTO_SYMBOL_MAPPINGS order"
        );
    }

    #[test]
    fn test_stock_symbol_ordering_matches_string_ordering() {
        let symbols: Vec<StockSymbol> = StockSymbol::values().collect();

        let mut sorted = symbols.clone();
        sorted.sort();

        let mut sorted_by_string = symbols.clone();
        sorted_by_string.sort_by_key(|s| s.to_string());

        assert_eq!(sorted, sorted_by_string);
    }

    #[test]
    fn test_crypto_symbol_ordering_matches_string_ordering() {
        let symbols: Vec<CryptoSymbol> = CryptoSymbol::values().collect();

        let mut sorted = symbols.clone();
        sorted.sort();

        let mut sorted_by_string = symbols.clone();
        sorted_by_string.sort_by_key(|s| s.to_string());

        assert_eq!(sorted, sorted_by_string);
    }

    #[test]
    fn test_cashtag_symbol_ordering_matches_string_ordering() {
        let symbols: Vec<CashtagSymbol> = CashtagSymbol::values().to_vec();

        let mut sorted = symbols.clone();
        sorted.sort();

        let mut sorted_by_string = symbols.clone();
        sorted_by_string.sort_by_key(|s| s.to_string());

        assert_eq!(sorted, sorted_by_string);
    }
}

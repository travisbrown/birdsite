use serde::{
    Serialize,
    de::{Deserialize, Deserializer, Visitor},
};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid case")]
    InvalidCase,
    #[error("Unknown stock symbol")]
    UnknownStockSymbol,
    #[error("Unknown crypto abbreviation")]
    UnknownCryptoSymbol,
    #[error("Unknown other symbol")]
    UnknownOtherSymbol,
}

/// May be a stock symbol, a crypto abbreviation, or an arbitrary sequence of letters.
///
/// In the case of non-standard casing (i.e. not fully capitalized) we preserve the original form
/// in order to be able to round-trip source JSON.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cashtag {
    pub symbol: CashtagSymbol,
    pub form: Option<String>,
}

impl<'de> Deserialize<'de> for Cashtag {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CashtagVisitor;

        impl Visitor<'_> for CashtagVisitor {
            type Value = Cashtag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Cashtag")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v.chars()
                    .any(|char| !char.is_ascii_uppercase() && char != '.')
                {
                    let value = v.to_ascii_uppercase();
                    let symbol = CashtagSymbol::from_uppercase_str(&value);

                    Ok(Self::Value {
                        symbol,
                        form: Some(value),
                    })
                } else {
                    let symbol = CashtagSymbol::from_uppercase_str(v);

                    Ok(Self::Value { symbol, form: None })
                }
            }
        }

        deserializer.deserialize_str(CashtagVisitor)
    }
}

impl Serialize for Cashtag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.form {
            Some(form) => form.serialize(serializer),
            None => self.symbol.to_string().serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CashtagSymbol {
    Stock(StockSymbol),
    Crypto(CryptoSymbol),
    Other(OtherSymbol),
    Unknown(String),
}

impl CashtagSymbol {
    // Assumes we've already checked the case.
    fn from_uppercase_str(s: &str) -> Self {
        s.parse::<StockSymbol>()
            .map(Self::Stock)
            .or_else(|_| s.parse::<CryptoSymbol>().map(Self::Crypto))
            .or_else(|_| s.parse::<OtherSymbol>().map(Self::Other))
            .unwrap_or_else(|_| Self::Unknown(s.to_string()))
    }
}

impl Display for CashtagSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stock(symbol) => symbol.fmt(f),
            Self::Crypto(symbol) => symbol.fmt(f),
            Self::Other(symbol) => symbol.fmt(f),
            Self::Unknown(symbol) => symbol.fmt(f),
        }
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
            Ok(Self::from_uppercase_str(s))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StockSymbol {
    Amc,
    Amazon,
    Apple,
    Baidu,
    BankOfAmerica,
    Boeing,
    Citi,
    Costco,
    CrowdStrike,
    GeneralMotors,
    GameStop,
    Google,
    Intel,
    JpMorgan,
    McDonalds,
    Meta,
    Microsoft,
    MorganStanley,
    Nvidia,
    Rivian,
    Tesla,
    TrumpMedia,
    Walmart,
    WellsFargo,
}

const STOCK_SYMBOL_MAPPINGS: &[(&str, StockSymbol)] = &[
    ("AMC", StockSymbol::Amc),
    ("AMZN", StockSymbol::Amazon),
    ("AAPL", StockSymbol::Apple),
    ("BIDU", StockSymbol::Baidu),
    ("BAC", StockSymbol::BankOfAmerica),
    ("BA", StockSymbol::Boeing),
    ("C", StockSymbol::Citi),
    ("COST", StockSymbol::Costco),
    ("CRWD", StockSymbol::CrowdStrike),
    ("GM", StockSymbol::GeneralMotors),
    ("GME", StockSymbol::GameStop),
    ("GOOGL", StockSymbol::Google),
    ("INTC", StockSymbol::Intel),
    ("JPM", StockSymbol::JpMorgan),
    ("MCD", StockSymbol::McDonalds),
    ("META", StockSymbol::Meta),
    ("MSFT", StockSymbol::Microsoft),
    ("MS", StockSymbol::MorganStanley),
    ("NVDA", StockSymbol::Nvidia),
    ("RIVN", StockSymbol::Rivian),
    ("TSLA", StockSymbol::Tesla),
    ("DJT", StockSymbol::TrumpMedia),
    ("WMT", StockSymbol::Walmart),
    ("WFC", StockSymbol::WellsFargo),
];

static STOCK_SYMBOL_TO: LazyLock<BTreeMap<StockSymbol, &'static str>> = LazyLock::new(|| {
    STOCK_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, value)| (*value, *symbol))
        .collect()
});

static STOCK_SYMBOL_FROM: LazyLock<BTreeMap<String, StockSymbol>> = LazyLock::new(|| {
    STOCK_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, value)| (symbol.to_string(), *value))
        .collect()
});

impl Display for StockSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            STOCK_SYMBOL_TO
                .get(self)
                .expect("Stock symbol missing (this is a bug)"),
        )
    }
}

impl FromStr for StockSymbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        STOCK_SYMBOL_FROM
            .get(s)
            .copied()
            .ok_or(Error::UnknownStockSymbol)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CryptoSymbol {
    Bitcoin,
    DaddyTate,
    Degen,
    Doge,
    Dogwifhat,
    Eth,
    Monero,
    Pepe,
    Ripple,
    UsdCoin,
}

const CRYPTO_SYMBOL_MAPPINGS: &[(&str, CryptoSymbol)] = &[
    ("BTC", CryptoSymbol::Bitcoin),
    ("DADDY", CryptoSymbol::DaddyTate),
    ("DEGEN", CryptoSymbol::Degen),
    ("DOGE", CryptoSymbol::Doge),
    ("WIF", CryptoSymbol::Dogwifhat),
    ("ETH", CryptoSymbol::Eth),
    ("XMR", CryptoSymbol::Monero),
    ("PEPE", CryptoSymbol::Pepe),
    ("XRP", CryptoSymbol::Ripple),
    ("USDC", CryptoSymbol::UsdCoin),
];

static CRYPTO_SYMBOL_TO: LazyLock<BTreeMap<CryptoSymbol, &'static str>> = LazyLock::new(|| {
    CRYPTO_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, value)| (*value, *symbol))
        .collect()
});

static CRYPTO_SYMBOL_FROM: LazyLock<BTreeMap<String, CryptoSymbol>> = LazyLock::new(|| {
    CRYPTO_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, value)| (symbol.to_string(), *value))
        .collect()
});

impl Display for CryptoSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            CRYPTO_SYMBOL_TO
                .get(self)
                .expect("Crypto abbreviation missing (this is a bug)"),
        )
    }
}

impl FromStr for CryptoSymbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CRYPTO_SYMBOL_FROM
            .get(s)
            .copied()
            .ok_or(Error::UnknownCryptoSymbol)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OtherSymbol {
    Ada,
    Agix,
    Aids,
    Aisnek,
    Algo,
    Alph,
    Amp,
    Andy,
    Anon,
    Apad,
    Apate,
    Ape,
    Apu,
    Arb,
    Army,
    Arrr,
    Asi,
    Atehun,
    Atlas,
    Atom,
    Aud,
    Audio,
    Avax,
    Axl,
    Ayin,
    Azero,
    Babb,
    Baby,
    Baer,
    Bags,
    Balz,
    Barron,
    Base,
    Bbig,
    Bch,
    Beam,
    Bells,
    Bendog,
    Bet,
    Bigsb,
    Billy,
    Blast,
    Blif,
    Blk,
    Block,
    Blox,
    Bmo,
    Bnb,
    Boden,
    Bold,
    Bon,
    Bones,
    Bonk,
    Boppy,
    Bot,
    Bozo,
    Brett,
    BrkA,
    Bruv,
    Bs,
    Bt,
    BtcX,
    Bub,
    Byte,
    Cake,
    Cate,
    Cats,
    Cbdc,
    Cboh,
    Celo,
    Cheng,
    Chex,
    Chud,
    Cience,
    Ckb,
    Cobra,
    Concho,
    Cony,
    Coq,
    Coreum,
    Cot,
    Crown,
    Cro,
    Crypto,
    Csc,
    Cspr,
    Cura,
    Cz,
    Dag,
    Dec,
    Deport,
    Dgb,
    Dis,
    Dmaga,
    Dobo,
    Dog,
    Donjr,
    Dot,
    Dpunks,
    Drip,
    Dsync,
    Dtjr,
    Duk,
    Duko,
    Earn,
    Ebr,
    Egirl,
    Egld,
    Elsie,
    Emory,
    Enqai,
    Ens,
    Epic,
    Epik,
    Etf,
    EthX,
    Evai,
    Ewt,
    Fakeai,
    Far,
    Fartdg,
    Father,
    Fet,
    Femcel,
    Ffie,
    Fi,
    Floki,
    Flr,
    Fnma,
    Ft,
    Ftm,
    Ftn,
    G,
    Game,
    Gary,
    Gates,
    Giga,
    Gob,
    Gold,
    Gpu,
    Grin,
    Gses,
    Gtbif,
    Gtii,
    H,
    Hawk,
    Hbar,
    Hdnw,
    Hero,
    Hex,
    Hims,
    Hit,
    Hiti,
    Hive,
    Hokk,
    Honor,
    Hoot,
    Hould,
    Hnt,
    Htr,
    Hulk,
    Hungry,
    Hymc,
    Hype,
    Iag,
    Icp,
    Inc,
    Inj,
    Ionq,
    Iota,
    Jenner,
    Jesus,
    Jhh,
    Jownes,
    Jup,
    Karate,
    Kas,
    Kda,
    Kkr,
    Kibl,
    Koin,
    Koko,
    Kol,
    Kolin,
    Kscp,
    Kween,
    Laptop,
    Lcx,
    Lgcy,
    Link,
    Linu,
    Lobo,
    Lotto,
    Low,
    Lrc,
    Ltc,
    Lucky,
    Luna,
    Lung,
    Macho,
    Maga,
    Magaa,
    Mastr,
    Matic,
    Max,
    Melek,
    Memefi,
    Mfer,
    Michi,
    Mlg,
    Mmat,
    Mmtlp,
    Mog,
    Mohat,
    Monai,
    Mother,
    Motoko,
    Mpc,
    Mryen,
    Msos,
    Msox,
    Mstr,
    Msty,
    Muln,
    Mumu,
    Muva,
    Mxi,
    Myro,
    Nati,
    Near,
    Net,
    Nexa,
    Ngu,
    Ninja,
    Nmt,
    Nne,
    Nomnom,
    Nos,
    Nokb,
    Npc,
    Nub,
    Nvidia,
    Nwa,
    Obema,
    Ondo,
    One,
    Open,
    Orai,
    Ordi,
    Ox,
    Paal,
    Peed,
    Peezy,
    Penn,
    Pep,
    Phteve,
    Phun,
    Pi,
    Piggy,
    Pixl,
    Pizza,
    Pls,
    Plsx,
    Pltr,
    Pndc,
    Pongo,
    Ponke,
    Popcat,
    Popdog,
    Pork,
    Psqh,
    Ptrump,
    Pucca,
    Pups,
    Puush,
    Qanx,
    Qckdep,
    Qnt,
    Qubic,
    Racist,
    Rapr,
    Rddt,
    Render,
    Retain,
    Rfk,
    Rio,
    Riot,
    Rizz,
    Rklb,
    Rndr,
    Rnt,
    Rose,
    Rpls,
    Rsic,
    Rum,
    Runes,
    Saito,
    Sats,
    Sc,
    Schd,
    Schizo,
    Seedz,
    Sei,
    Seing,
    Sgb,
    Shib,
    Shido,
    Shield,
    Sigma,
    Silj,
    Silly,
    Silver,
    Slerf,
    Smlr,
    Snek,
    Sntvt,
    Sol,
    Solama,
    Sold,
    Somo,
    Spx,
    Spy,
    Squid,
    Srp,
    Sss,
    Stan,
    Steve,
    Strk,
    Sui,
    Syd,
    Sys,
    Taddy,
    Tao,
    Tate,
    Tcash,
    Td,
    Tel,
    Tfuel,
    Thc,
    Theta,
    Thnd,
    Tiktok,
    Tking,
    Toby,
    Ton,
    Topg,
    Trch,
    Tremp,
    Tren,
    Tron,
    Trs,
    Trul,
    Trump,
    Trx,
    Tsly,
    Tsnd,
    Tsndf,
    Tsuka,
    Turbo,
    Twb,
    Twt,
    U,
    Uni,
    Uos,
    Upup,
    Usa,
    Usd,
    Usdt,
    Velo,
    Verse,
    Vet,
    Vine,
    Vra,
    Vxv,
    Waxp,
    Wbs,
    Wife,
    Wolf,
    Wtk,
    Wzrd,
    X,
    Xdc,
    Xep,
    Xlm,
    XrpX,
    Xrph,
    Younes,
    Zack,
    Zbit,
    Zuzu,
}

const OTHER_SYMBOL_MAPPINGS: &[(&str, OtherSymbol)] = &[
    ("ADA", OtherSymbol::Ada),
    ("AGIX", OtherSymbol::Agix),
    ("AIDS", OtherSymbol::Aids),
    ("AISNEK", OtherSymbol::Aisnek),
    ("ALGO", OtherSymbol::Algo),
    ("ALPH", OtherSymbol::Alph),
    ("AMP", OtherSymbol::Amp),
    ("ANDY", OtherSymbol::Andy),
    ("ANON", OtherSymbol::Anon),
    ("APAD", OtherSymbol::Apad),
    ("APATE", OtherSymbol::Apate),
    ("APE", OtherSymbol::Ape),
    ("APU", OtherSymbol::Apu),
    ("ARB", OtherSymbol::Arb),
    ("ARMY", OtherSymbol::Army),
    ("ARRR", OtherSymbol::Arrr),
    ("ASI", OtherSymbol::Asi),
    ("ATEHUN", OtherSymbol::Atehun),
    ("ATLAS", OtherSymbol::Atlas),
    ("ATOM", OtherSymbol::Atom),
    ("AUD", OtherSymbol::Aud),
    ("AUDIO", OtherSymbol::Audio),
    ("AVAX", OtherSymbol::Avax),
    ("AXL", OtherSymbol::Axl),
    ("AYIN", OtherSymbol::Ayin),
    ("AZERO", OtherSymbol::Azero),
    ("BABB", OtherSymbol::Babb),
    ("BABY", OtherSymbol::Baby),
    ("BAER", OtherSymbol::Baer),
    ("BAGS", OtherSymbol::Bags),
    ("BALZ", OtherSymbol::Balz),
    ("BARRON", OtherSymbol::Barron),
    ("BASE", OtherSymbol::Base),
    ("BBIG", OtherSymbol::Bbig),
    ("BCH", OtherSymbol::Bch),
    ("BEAM", OtherSymbol::Beam),
    ("BELLS", OtherSymbol::Bells),
    ("BENDOG", OtherSymbol::Bendog),
    ("BET", OtherSymbol::Bet),
    ("BIGSB", OtherSymbol::Bigsb),
    ("BILLY", OtherSymbol::Billy),
    ("BLAST", OtherSymbol::Blast),
    ("BLIF", OtherSymbol::Blif),
    ("BLK", OtherSymbol::Blk),
    ("BLOCK", OtherSymbol::Block),
    ("BLOX", OtherSymbol::Blox),
    ("BMO", OtherSymbol::Bmo),
    ("BNB", OtherSymbol::Bnb),
    ("BODEN", OtherSymbol::Boden),
    ("BOLD", OtherSymbol::Bold),
    ("BON", OtherSymbol::Bon),
    ("BONES", OtherSymbol::Bones),
    ("BONK", OtherSymbol::Bonk),
    ("BOPPY", OtherSymbol::Boppy),
    ("BOT", OtherSymbol::Bot),
    ("BOZO", OtherSymbol::Bozo),
    ("BRETT", OtherSymbol::Brett),
    ("BRK.A", OtherSymbol::BrkA),
    ("BRUV", OtherSymbol::Bruv),
    ("BS", OtherSymbol::Bs),
    ("BT", OtherSymbol::Bt),
    ("BTC.X", OtherSymbol::BtcX),
    ("BUB", OtherSymbol::Bub),
    ("BYTE", OtherSymbol::Byte),
    ("CAKE", OtherSymbol::Cake),
    ("CATE", OtherSymbol::Cate),
    ("CATS", OtherSymbol::Cats),
    ("CBDC", OtherSymbol::Cbdc),
    ("CBOH", OtherSymbol::Cboh),
    ("CELO", OtherSymbol::Celo),
    ("CHENG", OtherSymbol::Cheng),
    ("CHEX", OtherSymbol::Chex),
    ("CHUD", OtherSymbol::Chud),
    ("CIENCE", OtherSymbol::Cience),
    ("CKB", OtherSymbol::Ckb),
    ("COBRA", OtherSymbol::Cobra),
    ("CONCHO", OtherSymbol::Concho),
    ("CONY", OtherSymbol::Cony),
    ("COQ", OtherSymbol::Coq),
    ("COREUM", OtherSymbol::Coreum),
    ("COT", OtherSymbol::Cot),
    ("CRO", OtherSymbol::Cro),
    ("CROWN", OtherSymbol::Crown),
    ("CRYPTO", OtherSymbol::Crypto),
    ("CSC", OtherSymbol::Csc),
    ("CSPR", OtherSymbol::Cspr),
    ("CURA", OtherSymbol::Cura),
    ("CZ", OtherSymbol::Cz),
    ("DAG", OtherSymbol::Dag),
    ("DEC", OtherSymbol::Dec),
    ("DEPORT", OtherSymbol::Deport),
    ("DGB", OtherSymbol::Dgb),
    ("DIS", OtherSymbol::Dis),
    ("DMAGA", OtherSymbol::Dmaga),
    ("DOBO", OtherSymbol::Dobo),
    ("DOG", OtherSymbol::Dog),
    ("DONJR", OtherSymbol::Donjr),
    ("DOT", OtherSymbol::Dot),
    ("DPUNKS", OtherSymbol::Dpunks),
    ("DRIP", OtherSymbol::Drip),
    ("DSYNC", OtherSymbol::Dsync),
    ("DTJR", OtherSymbol::Dtjr),
    ("DUK", OtherSymbol::Duk),
    ("DUKO", OtherSymbol::Duko),
    ("EARN", OtherSymbol::Earn),
    ("EBR", OtherSymbol::Ebr),
    ("EGIRL", OtherSymbol::Egirl),
    ("EGLD", OtherSymbol::Egld),
    ("ELSIE", OtherSymbol::Elsie),
    ("EMORY", OtherSymbol::Emory),
    ("ENQAI", OtherSymbol::Enqai),
    ("ENS", OtherSymbol::Ens),
    ("EPIC", OtherSymbol::Epic),
    ("EPIK", OtherSymbol::Epik),
    ("ETF", OtherSymbol::Etf),
    ("ETH.X", OtherSymbol::EthX),
    ("EVAI", OtherSymbol::Evai),
    ("EWT", OtherSymbol::Ewt),
    ("FAKEAI", OtherSymbol::Fakeai),
    ("FAR", OtherSymbol::Far),
    ("FARTDG", OtherSymbol::Fartdg),
    ("FATHER", OtherSymbol::Father),
    ("FEMCEL", OtherSymbol::Femcel),
    ("FET", OtherSymbol::Fet),
    ("FFIE", OtherSymbol::Ffie),
    ("FI", OtherSymbol::Fi),
    ("FLOKI", OtherSymbol::Floki),
    ("FLR", OtherSymbol::Flr),
    ("FNMA", OtherSymbol::Fnma),
    ("FT", OtherSymbol::Ft),
    ("FTM", OtherSymbol::Ftm),
    ("FTN", OtherSymbol::Ftn),
    ("G", OtherSymbol::G),
    ("GAME", OtherSymbol::Game),
    ("GARY", OtherSymbol::Gary),
    ("GATES", OtherSymbol::Gates),
    ("GIGA", OtherSymbol::Giga),
    ("GOB", OtherSymbol::Gob),
    ("GOLD", OtherSymbol::Gold),
    ("GPU", OtherSymbol::Gpu),
    ("GRIN", OtherSymbol::Grin),
    ("GSES", OtherSymbol::Gses),
    ("GTBIF", OtherSymbol::Gtbif),
    ("GTII", OtherSymbol::Gtii),
    ("H", OtherSymbol::H),
    ("HAWK", OtherSymbol::Hawk),
    ("HBAR", OtherSymbol::Hbar),
    ("HDNW", OtherSymbol::Hdnw),
    ("HERO", OtherSymbol::Hero),
    ("HEX", OtherSymbol::Hex),
    ("HIMS", OtherSymbol::Hims),
    ("HIT", OtherSymbol::Hit),
    ("HITI", OtherSymbol::Hiti),
    ("HIVE", OtherSymbol::Hive),
    ("HOKK", OtherSymbol::Hokk),
    ("HONOR", OtherSymbol::Honor),
    ("HOOT", OtherSymbol::Hoot),
    ("HOULD", OtherSymbol::Hould),
    ("HNT", OtherSymbol::Hnt),
    ("HTR", OtherSymbol::Htr),
    ("HULK", OtherSymbol::Hulk),
    ("HUNGRY", OtherSymbol::Hungry),
    ("HYMC", OtherSymbol::Hymc),
    ("HYPE", OtherSymbol::Hype),
    ("IAG", OtherSymbol::Iag),
    ("ICP", OtherSymbol::Icp),
    ("INC", OtherSymbol::Inc),
    ("INJ", OtherSymbol::Inj),
    ("IONQ", OtherSymbol::Ionq),
    ("IOTA", OtherSymbol::Iota),
    ("JENNER", OtherSymbol::Jenner),
    ("JESUS", OtherSymbol::Jesus),
    ("JHH", OtherSymbol::Jhh),
    ("JOWNES", OtherSymbol::Jownes),
    ("JUP", OtherSymbol::Jup),
    ("KARATE", OtherSymbol::Karate),
    ("KAS", OtherSymbol::Kas),
    ("KDA", OtherSymbol::Kda),
    ("KIBL", OtherSymbol::Kibl),
    ("KKR", OtherSymbol::Kkr),
    ("KOIN", OtherSymbol::Koin),
    ("KOKO", OtherSymbol::Koko),
    ("KOL", OtherSymbol::Kol),
    ("KOLIN", OtherSymbol::Kolin),
    ("KSCP", OtherSymbol::Kscp),
    ("KWEEN", OtherSymbol::Kween),
    ("LAPTOP", OtherSymbol::Laptop),
    ("LCX", OtherSymbol::Lcx),
    ("LGCY", OtherSymbol::Lgcy),
    ("LINK", OtherSymbol::Link),
    ("LINU", OtherSymbol::Linu),
    ("LOBO", OtherSymbol::Lobo),
    ("LOTTO", OtherSymbol::Lotto),
    ("LOW", OtherSymbol::Low),
    ("LRC", OtherSymbol::Lrc),
    ("LTC", OtherSymbol::Ltc),
    ("LUCKY", OtherSymbol::Lucky),
    ("LUNA", OtherSymbol::Luna),
    ("LUNG", OtherSymbol::Lung),
    ("MACHO", OtherSymbol::Macho),
    ("MAGA", OtherSymbol::Maga),
    ("MAGAA", OtherSymbol::Magaa),
    ("MASTR", OtherSymbol::Mastr),
    ("MATIC", OtherSymbol::Matic),
    ("MAX", OtherSymbol::Max),
    ("MELEK", OtherSymbol::Melek),
    ("MEMEFI", OtherSymbol::Memefi),
    ("MICHI", OtherSymbol::Michi),
    ("MFER", OtherSymbol::Mfer),
    ("MLG", OtherSymbol::Mlg),
    ("MMAT", OtherSymbol::Mmat),
    ("MMTLP", OtherSymbol::Mmtlp),
    ("MOG", OtherSymbol::Mog),
    ("MOHAT", OtherSymbol::Mohat),
    ("MONAI", OtherSymbol::Monai),
    ("MOTHER", OtherSymbol::Mother),
    ("MOTOKO", OtherSymbol::Motoko),
    ("MPC", OtherSymbol::Mpc),
    ("MRYEN", OtherSymbol::Mryen),
    ("MSOS", OtherSymbol::Msos),
    ("MSOX", OtherSymbol::Msox),
    ("MSTR", OtherSymbol::Mstr),
    ("MSTY", OtherSymbol::Msty),
    ("MUMU", OtherSymbol::Mumu),
    ("MUVA", OtherSymbol::Muva),
    ("MULN", OtherSymbol::Muln),
    ("MXI", OtherSymbol::Mxi),
    ("MYRO", OtherSymbol::Myro),
    ("NATI", OtherSymbol::Nati),
    ("NEAR", OtherSymbol::Near),
    ("NET", OtherSymbol::Net),
    ("NEXA", OtherSymbol::Nexa),
    ("NGU", OtherSymbol::Ngu),
    ("NINJA", OtherSymbol::Ninja),
    ("NMT", OtherSymbol::Nmt),
    ("NNE", OtherSymbol::Nne),
    ("NOKB", OtherSymbol::Nokb),
    ("NOMNOM", OtherSymbol::Nomnom),
    ("NOS", OtherSymbol::Nos),
    ("NPC", OtherSymbol::Npc),
    ("NUB", OtherSymbol::Nub),
    ("NVIDIA", OtherSymbol::Nvidia),
    ("NWA", OtherSymbol::Nwa),
    ("OBEMA", OtherSymbol::Obema),
    ("ONDO", OtherSymbol::Ondo),
    ("ONE", OtherSymbol::One),
    ("OPEN", OtherSymbol::Open),
    ("ORAI", OtherSymbol::Orai),
    ("ORDI", OtherSymbol::Ordi),
    ("OX", OtherSymbol::Ox),
    ("PAAL", OtherSymbol::Paal),
    ("PEED", OtherSymbol::Peed),
    ("PEEZY", OtherSymbol::Peezy),
    ("PENN", OtherSymbol::Penn),
    ("PEP", OtherSymbol::Pep),
    ("PHTEVE", OtherSymbol::Phteve),
    ("PHUN", OtherSymbol::Phun),
    ("PI", OtherSymbol::Pi),
    ("PIGGY", OtherSymbol::Piggy),
    ("PIXL", OtherSymbol::Pixl),
    ("PIZZA", OtherSymbol::Pizza),
    ("PLS", OtherSymbol::Pls),
    ("PLSX", OtherSymbol::Plsx),
    ("PLTR", OtherSymbol::Pltr),
    ("PNDC", OtherSymbol::Pndc),
    ("PONGO", OtherSymbol::Pongo),
    ("PONKE", OtherSymbol::Ponke),
    ("POPCAT", OtherSymbol::Popcat),
    ("POPDOG", OtherSymbol::Popdog),
    ("PORK", OtherSymbol::Pork),
    ("PSQH", OtherSymbol::Psqh),
    ("PTRUMP", OtherSymbol::Ptrump),
    ("PUCCA", OtherSymbol::Pucca),
    ("PUPS", OtherSymbol::Pups),
    ("PUUSH", OtherSymbol::Puush),
    ("QANX", OtherSymbol::Qanx),
    ("QCKDEP", OtherSymbol::Qckdep),
    ("QNT", OtherSymbol::Qnt),
    ("QUBIC", OtherSymbol::Qubic),
    ("RACIST", OtherSymbol::Racist),
    ("RAPR", OtherSymbol::Rapr),
    ("RDDT", OtherSymbol::Rddt),
    ("RENDER", OtherSymbol::Render),
    ("RETAIN", OtherSymbol::Retain),
    ("RFK", OtherSymbol::Rfk),
    ("RIO", OtherSymbol::Rio),
    ("RIOT", OtherSymbol::Riot),
    ("RIZZ", OtherSymbol::Rizz),
    ("RKLB", OtherSymbol::Rklb),
    ("RNDR", OtherSymbol::Rndr),
    ("RNT", OtherSymbol::Rnt),
    ("ROSE", OtherSymbol::Rose),
    ("RPLS", OtherSymbol::Rpls),
    ("RSIC", OtherSymbol::Rsic),
    ("RUM", OtherSymbol::Rum),
    ("RUNES", OtherSymbol::Runes),
    ("SAITO", OtherSymbol::Saito),
    ("SATS", OtherSymbol::Sats),
    ("SC", OtherSymbol::Sc),
    ("SCHD", OtherSymbol::Schd),
    ("SCHIZO", OtherSymbol::Schizo),
    ("SEEDZ", OtherSymbol::Seedz),
    ("SEI", OtherSymbol::Sei),
    ("SEING", OtherSymbol::Seing),
    ("SGB", OtherSymbol::Sgb),
    ("SHIB", OtherSymbol::Shib),
    ("SHIDO", OtherSymbol::Shido),
    ("SHIELD", OtherSymbol::Shield),
    ("SIGMA", OtherSymbol::Sigma),
    ("SILJ", OtherSymbol::Silj),
    ("SILLY", OtherSymbol::Silly),
    ("SILVER", OtherSymbol::Silver),
    ("SLERF", OtherSymbol::Slerf),
    ("SMLR", OtherSymbol::Smlr),
    ("SNEK", OtherSymbol::Snek),
    ("SNTVT", OtherSymbol::Sntvt),
    ("SOL", OtherSymbol::Sol),
    ("SOLAMA", OtherSymbol::Solama),
    ("SOLD", OtherSymbol::Sold),
    ("SOMO", OtherSymbol::Somo),
    ("SPX", OtherSymbol::Spx),
    ("SPY", OtherSymbol::Spy),
    ("SQUID", OtherSymbol::Squid),
    ("SRP", OtherSymbol::Srp),
    ("SSS", OtherSymbol::Sss),
    ("STAN", OtherSymbol::Stan),
    ("STEVE", OtherSymbol::Steve),
    ("STRK", OtherSymbol::Strk),
    ("SUI", OtherSymbol::Sui),
    ("SYD", OtherSymbol::Syd),
    ("SYS", OtherSymbol::Sys),
    ("TADDY", OtherSymbol::Taddy),
    ("TAO", OtherSymbol::Tao),
    ("TATE", OtherSymbol::Tate),
    ("TCASH", OtherSymbol::Tcash),
    ("TD", OtherSymbol::Td),
    ("TEL", OtherSymbol::Tel),
    ("TFUEL", OtherSymbol::Tfuel),
    ("THETA", OtherSymbol::Theta),
    ("THC", OtherSymbol::Thc),
    ("THND", OtherSymbol::Thnd),
    ("TIKTOK", OtherSymbol::Tiktok),
    ("TKING", OtherSymbol::Tking),
    ("TOBY", OtherSymbol::Toby),
    ("TON", OtherSymbol::Ton),
    ("TOPG", OtherSymbol::Topg),
    ("TRCH", OtherSymbol::Trch),
    ("TREMP", OtherSymbol::Tremp),
    ("TREN", OtherSymbol::Tren),
    ("TRON", OtherSymbol::Tron),
    ("TRS", OtherSymbol::Trs),
    ("TRX", OtherSymbol::Trx),
    ("TRUL", OtherSymbol::Trul),
    ("TRUMP", OtherSymbol::Trump),
    ("TSLY", OtherSymbol::Tsly),
    ("TSND", OtherSymbol::Tsnd),
    ("TSNDF", OtherSymbol::Tsndf),
    ("TSUKA", OtherSymbol::Tsuka),
    ("TURBO", OtherSymbol::Turbo),
    ("TWB", OtherSymbol::Twb),
    ("TWT", OtherSymbol::Twt),
    ("U", OtherSymbol::U),
    ("UNI", OtherSymbol::Uni),
    ("UOS", OtherSymbol::Uos),
    ("UPUP", OtherSymbol::Upup),
    ("USA", OtherSymbol::Usa),
    ("USD", OtherSymbol::Usd),
    ("USDT", OtherSymbol::Usdt),
    ("VELO", OtherSymbol::Velo),
    ("VERSE", OtherSymbol::Verse),
    ("VET", OtherSymbol::Vet),
    ("VINE", OtherSymbol::Vine),
    ("VRA", OtherSymbol::Vra),
    ("VXV", OtherSymbol::Vxv),
    ("WAXP", OtherSymbol::Waxp),
    ("WBS", OtherSymbol::Wbs),
    ("WIFE", OtherSymbol::Wife),
    ("WOLF", OtherSymbol::Wolf),
    ("WTK", OtherSymbol::Wtk),
    ("WZRD", OtherSymbol::Wzrd),
    ("X", OtherSymbol::X),
    ("XDC", OtherSymbol::Xdc),
    ("XEP", OtherSymbol::Xep),
    ("XLM", OtherSymbol::Xlm),
    ("XRP.X", OtherSymbol::XrpX),
    ("XRPH", OtherSymbol::Xrph),
    ("YOUNES", OtherSymbol::Younes),
    ("ZACK", OtherSymbol::Zack),
    ("ZBIT", OtherSymbol::Zbit),
    ("ZUZU", OtherSymbol::Zuzu),
];

static OTHER_SYMBOL_TO: LazyLock<BTreeMap<OtherSymbol, &'static str>> = LazyLock::new(|| {
    OTHER_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, value)| (*value, *symbol))
        .collect()
});

static OTHER_SYMBOL_FROM: LazyLock<BTreeMap<String, OtherSymbol>> = LazyLock::new(|| {
    OTHER_SYMBOL_MAPPINGS
        .iter()
        .map(|(symbol, value)| (symbol.to_string(), *value))
        .collect()
});

impl Display for OtherSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            OTHER_SYMBOL_TO
                .get(self)
                .expect("Other symbol missing (this is a bug)"),
        )
    }
}

impl FromStr for OtherSymbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OTHER_SYMBOL_FROM
            .get(s)
            .copied()
            .ok_or(Error::UnknownOtherSymbol)
    }
}

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Unexpected, Visitor},
};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid color code")]
    Invalid(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 6 {
            u16::from_str_radix(&s[0..2], 16)
                .and_then(|red| {
                    u16::from_str_radix(&s[2..4], 16).and_then(|green| {
                        u16::from_str_radix(&s[4..6], 16).map(|blue| Self { red, green, blue })
                    })
                })
                .map_err(|_| Self::Err::Invalid(s.to_string()))
        } else {
            Err(Self::Err::Invalid(s.to_string()))
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}{:X}{:X}", self.red, self.green, self.blue)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ColorVisitor;

        impl Visitor<'_> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct Color")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<Self::Value>().map_err(|_| {
                    serde::de::Error::invalid_value(Unexpected::Str(v), &"color hex string")
                })
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use serde_json::json;

    const EXAMPLE_TEXT_TIMESTAMP: &str = "6D5C18";

    #[test]
    fn round_trip_color() {
        let expected = Color {
            red: 109,
            green: 92,
            blue: 24,
        };

        let json = json!(EXAMPLE_TEXT_TIMESTAMP);
        let deserialized: Color = serde_json::from_str(&json.to_string()).unwrap();

        assert_eq!(deserialized, expected);

        let reserialized = json!(deserialized);

        assert_eq!(reserialized, json);
    }
}

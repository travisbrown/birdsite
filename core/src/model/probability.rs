use num_rational::Ratio;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Ratio<u64>, D::Error> {
    let value = f64::deserialize(deserializer)?;
    let mut text = value.to_string();
    let decimal_point_index = text.find('.');
    let decimal_places = decimal_point_index
        .map(|index| text.len() - index - 1)
        .unwrap_or(0);

    if let Some(index) = decimal_point_index {
        text.remove(index);
    }

    let numerator = text.parse::<u64>().map_err(|_| {
        serde::de::Error::invalid_value(serde::de::Unexpected::Float(value), &"u64 ratio")
    })?;

    let denominator = 10u64.pow(decimal_places as u32);

    Ok(Ratio::new(numerator, denominator))
}

pub fn serialize<S: Serializer>(value: &Ratio<u64>, serializer: S) -> Result<S::Ok, S::Error> {
    f64::serialize(&(*value.numer() as f64 / *value.denom() as f64), serializer)
}

fn _deserialize_deprecated<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Ratio<u64>, D::Error> {
    f64::deserialize(deserializer).and_then(|value| {
        Ratio::from_float(value)
            .and_then(|ratio| {
                let (numerator_sign, numerator_digits) = ratio.numer().to_u64_digits();
                let (denominator_sign, denominator_digits) = ratio.denom().to_u64_digits();

                if numerator_sign == num_bigint::Sign::NoSign
                    && denominator_sign == num_bigint::Sign::NoSign
                    && numerator_digits.len() == 1
                    && denominator_digits.len() == 1
                    && denominator_digits[0] != 0
                {
                    Some(Ratio::new(numerator_digits[0], denominator_digits[0]))
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Float(value), &"u64 ratio")
            })
    })
}

#[cfg(test)]
mod tests {
    use crate::model::probability;
    use num_rational::Ratio;

    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Test {
        #[serde(with = "probability")]
        foo: Ratio<u64>,
        #[serde(with = "probability")]
        bar: Ratio<u64>,
    }

    #[test]
    fn parse_json() {
        let test =
            serde_json::from_str::<Test>(r#"{ "foo": 0.1372897, "bar": 132791823 }"#).unwrap();

        assert_eq!(test.foo, Ratio::new(1372897, 10000000));
        assert_eq!(test.bar, Ratio::new(132791823, 1));
    }

    #[test]
    fn round_trip_json() {
        let value = Test {
            foo: Ratio::new(1372897, 10000000),
            bar: Ratio::new(132791823, 1),
        };

        let serialized = serde_json::json!(value).to_string();
        let deserialized = serde_json::from_str::<Test>(&serialized).unwrap();

        assert_eq!(deserialized, value);
    }
}

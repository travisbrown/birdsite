use crate::{Endpoint, SiteInfo, TransactionId};
use base64::prelude::*;
use chrono::Utc;
use rand::RngExt;
use sha2::Digest;

mod color;
mod cubic;

const DEFAULT_METHOD: &str = "GET";
const DEFAULT_KEYWORD: &str = "obfiowerehiring";
const DEFAULT_NUMBER: u8 = 3;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Generator {
    keyword: String,
    number: u8,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(DEFAULT_KEYWORD, DEFAULT_NUMBER)
    }
}

impl Generator {
    #[must_use]
    pub fn new(keyword: &str, number: u8) -> Self {
        Self {
            keyword: keyword.to_string(),
            number,
        }
    }

    /// Generate an ID for a given endpoint using current site information.
    #[must_use]
    pub fn compute(
        &self,
        site_info: &SiteInfo,
        endpoint: &Endpoint<'_>,
        random_byte: Option<u8>,
        timestamp_s: Option<i64>,
    ) -> TransactionId {
        self.compute_for_path(
            site_info,
            &Self::path(&endpoint.name, &endpoint.version),
            random_byte,
            timestamp_s,
        )
    }

    /// Generate an ID for a given endpoint using current site information.
    #[must_use]
    pub fn compute_for_path(
        &self,
        site_info: &SiteInfo,
        path: &str,
        random_byte: Option<u8>,
        timestamp_s: Option<i64>,
    ) -> TransactionId {
        let key_bytes_indices = &site_info.indices[1..];

        let frame_time = key_bytes_indices
            .iter()
            .map(|i| i32::from(site_info.verification_key[*i] % 16))
            .product::<i32>();

        let frame_time = ((f64::from(frame_time) / 10.0).round() * 10.0) as usize;
        let total_time: f64 = 4096.0;
        let target_time = frame_time as f64 / total_time;

        let animation_key = animation_key(&site_info.frame, target_time);

        let timestamp_s = timestamp_s.unwrap_or_else(|| {
            let ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Invalid system clock")
                .as_millis() as i128;

            // `2023-05-01T00:00:00Z`
            let base_ms = 1_682_924_400i128 * 1000;
            ((ms - base_ms) / 1000) as i64
        });

        let timestamp_bytes = (timestamp_s as u32).to_le_bytes();

        let digest_input = format!(
            "{}!{}!{}{}{}",
            DEFAULT_METHOD, path, timestamp_s, self.keyword, animation_key
        );

        let digest = sha2::Sha256::digest(digest_input.as_bytes());

        let mut bytes = Vec::with_capacity(site_info.verification_key.len() + 4 + 16 + 1);
        bytes.extend_from_slice(&site_info.verification_key);
        bytes.extend_from_slice(&timestamp_bytes);
        bytes.extend_from_slice(&digest[..16]);
        bytes.push(self.number);

        let random_byte = random_byte.unwrap_or_else(|| rand::rng().random());

        // We write the random byte and xor everything else with that byte.
        let mut output = Vec::with_capacity(1 + bytes.len());
        output.push(random_byte);
        output.extend(bytes.iter().map(|value| value ^ random_byte));

        TransactionId {
            value: base64::engine::general_purpose::STANDARD_NO_PAD.encode(output),
            animation_key: Some(animation_key),
            timestamp: Utc::now(),
        }
    }

    fn path(name: &str, version: &str) -> String {
        format!("/i/api/graphql/{version}/{name}")
    }
}

fn solve(value: f64, min_value: f64, max_value: f64, rounding: bool) -> f64 {
    let result = value * (max_value - min_value) / 255.0 + min_value;

    if rounding {
        result.floor()
    } else {
        (result * 100.0).round() / 100.0
    }
}

fn animation_key(frames: &[i32], target_time: f64) -> String {
    let from_color = color::Color::new(frames[0] as u8, frames[1] as u8, frames[2] as u8);
    let to_color = color::Color::new(frames[3] as u8, frames[4] as u8, frames[5] as u8);

    let from_rotation = 0.0f64;
    let to_rotation = solve(f64::from(frames[6]), 60.0, 360.0, true);

    let curve_values = frames
        .iter()
        .skip(7)
        .take(4)
        .enumerate()
        .map(|(i, &v)| {
            solve(
                f64::from(v),
                if i % 2 == 1 { -1.0 } else { 0.0 },
                1.0,
                false,
            )
        })
        .collect::<Vec<_>>();

    // Safe because we checked the length of the frame in the client.
    let cubic = cubic::Cubic::new([
        curve_values[0],
        curve_values[1],
        curve_values[2],
        curve_values[3],
    ]);

    let curve_value = cubic.value(target_time);

    // Interpolate color and rotation.
    let color = from_color.interpolate(to_color, curve_value);
    let rotation = interpolate(from_rotation, to_rotation, curve_value);

    let mut pieces: Vec<String> = vec![color.to_string()];

    let matrix = convert_rotation_to_matrix(rotation);

    for value in matrix {
        let rounded = math::round::half_to_even(value, 2).abs();

        let hex_value = float_to_hex(rounded);
        if hex_value.is_empty() {
            pieces.push("0".to_string());
        } else if hex_value.starts_with('.') {
            pieces.push(format!("0{hex_value}").to_lowercase());
        } else {
            pieces.push(hex_value.to_lowercase());
        }
    }

    pieces.push("00".to_string());

    // Concatenate and remove `.` and `-`.
    pieces
        .join("")
        .chars()
        .filter(|&c| c != '.' && c != '-')
        .collect()
}

fn convert_rotation_to_matrix(rotation_degrees: f64) -> [f64; 4] {
    let radians = rotation_degrees.to_radians();
    [radians.cos(), -radians.sin(), radians.sin(), radians.cos()]
}

// Uppercase hex string, matches Python implementation.
fn float_to_hex(x: f64) -> String {
    let int_part = x.trunc() as u128;
    let frac = x - int_part as f64;

    let int_str = if int_part == 0 {
        String::new()
    } else {
        let mut n = int_part;
        let mut tmp = Vec::<char>::new();
        while n > 0 {
            let rem = (n % 16) as u32;
            n /= 16;
            tmp.push(std::char::from_digit(rem, 16).unwrap().to_ascii_uppercase());
        }
        tmp.into_iter().rev().collect()
    };

    if frac == 0.0 {
        int_str
    } else {
        // Fractional digits.
        let mut frac_str = String::new();
        let mut f = frac;
        let mut steps = 0usize;
        while f > 0.0 && steps < 32 {
            f *= 16.0;
            let digit = f.trunc() as u32;
            f -= f64::from(digit);
            frac_str.push(
                std::char::from_digit(digit, 16)
                    .unwrap()
                    .to_ascii_uppercase(),
            );
            steps += 1;
        }

        if int_str.is_empty() {
            format!(".{frac_str}")
        } else {
            format!("{int_str}.{frac_str}")
        }
    }
}

fn interpolate(a: f64, b: f64, f: f64) -> f64 {
    a.mul_add(1.0 - f, b * f)
}

use base64::prelude::*;
use rand::Rng;

pub mod client;
mod cubic;

const DEFAULT_METHOD: &str = "GET";
const DEFAULT_SEED_KEYWORD: &str = "obfiowerehiring";
const DEFAULT_SEED_NUMBER: u8 = 3;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP client error")]
    Client(#[from] client::Error),
}

pub struct Generator {
    seed_keyword: String,
    seed_number: u8,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(DEFAULT_SEED_KEYWORD, DEFAULT_SEED_NUMBER)
    }
}

impl Generator {
    pub fn new(seed_keyword: &str, seed_number: u8) -> Self {
        Self {
            seed_keyword: seed_keyword.to_string(),
            seed_number,
        }
    }

    pub fn compute(
        &self,
        site_info: &client::SiteInfo,
        name: &str,
        version: &str,
    ) -> Result<String, Error> {
        let row_index = site_info.indices[0];
        let key_bytes_indices = &site_info.indices[1..];

        let row_index_from_key = site_info.verification_key[row_index] % 16;
        let frame_row = &site_info.array[row_index_from_key as usize];
        eprintln!("{:?}", frame_row);

        let frame_time = key_bytes_indices
            .iter()
            .map(|i| (site_info.verification_key[*i] % 16) as i32)
            .product::<i32>();

        let frame_time = ((frame_time as f64 / 10.0).round() * 10.0) as usize;
        let total_time = 4096;
        let target_time = frame_time as f64 / total_time as f64;

        let animation_key = animation_key(frame_row, target_time);

        eprintln!("Animation key: {}", animation_key);

        let transaction_id = generate_transaction_id(
            DEFAULT_METHOD,
            &Self::path(name, version),
            &self.seed_keyword,
            self.seed_number,
            &site_info.verification_key,
            &animation_key,
            None,
        );

        Ok(transaction_id)
    }

    fn path(name: &str, version: &str) -> String {
        format!("/i/api/graphql/{version}/{name}")
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn my_test() -> Result<(), super::Error> {
        let client = super::client::Client::default();
        let site_info = client.get_site_info().await?;

        let generator = super::Generator::default();

        let transaction_id =
            generator.compute(&site_info, "UsersByRestIds", "1hjT2eXW1Zcw-2xk8EbvoA")?;

        println!("{}", transaction_id);

        Ok(())
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

pub fn animation_key(frames: &[i32], target_time: f64) -> String {
    assert!(frames.len() >= 11, "frames must have at least 11 elements");

    // from_color / to_color with alpha=1.0 (ignored later)
    //let from_color = vec![frames[0] as f64, frames[1] as f64, frames[2] as f64, 1.0];
    //let to_color = vec![frames[3] as f64, frames[4] as f64, frames[5] as f64, 1.0];
    let from_color = Color::new(frames[0] as u8, frames[1] as u8, frames[2] as u8);
    let to_color = Color::new(frames[3] as u8, frames[4] as u8, frames[5] as u8);

    // rotations: [0] -> to
    let from_rotation = 0.0f64;
    let to_rotation = solve(frames[6] as f64, 60.0, 360.0, true);

    // cubic-bezier control points from frames[7..]
    let tail = &frames[7..];
    assert!(
        tail.len() >= 4,
        "expected at least 4 control values for cubic"
    );

    let curves_vec: Vec<f64> = tail
        .iter()
        .take(4)
        .enumerate()
        .map(|(i, &v)| {
            // Python: -1 if odd index else 0
            let a = if i % 2 == 1 { -1.0 } else { 0.0 };
            solve(v as f64, a, 1.0, false)
        })
        .collect();

    let curves: [f64; 4] = [curves_vec[0], curves_vec[1], curves_vec[2], curves_vec[3]];
    let cubic = cubic::Cubic::new(curves);
    let val = cubic.get_value(target_time);

    // Interpolate color and rotation
    let color = from_color.interpolate(to_color, val);
    let rotation = interpolate(from_rotation, to_rotation, val);

    let matrix = convert_rotation_to_matrix(rotation);

    // Colors → hex of rounded ints (Python round, not padded, lowercase)
    let mut str_arr: Vec<String> = vec![color.to_string()];

    // Matrix → round to 2 decimals, abs, hex-float string-ish, prefixed with "0" if starts with '.'
    for &v in &matrix {
        let rounded = math::round::half_to_even(v, 2).abs();

        let hex_value = float_to_hex(rounded);
        if hex_value.is_empty() {
            str_arr.push("0".to_string());
        } else if hex_value.starts_with('.') {
            str_arr.push(format!("0{}", hex_value).to_lowercase());
        } else {
            str_arr.push(hex_value.to_lowercase());
        }
    }

    // Append trailing zeros as strings
    str_arr.push("0".into());
    str_arr.push("0".into());

    // Concatenate and strip '.' and '-' like re.sub(r"[.-]", "", ...)
    let concatenated = str_arr.join("");
    concatenated
        .chars()
        .filter(|&c| c != '.' && c != '-')
        .collect()
}

#[inline]
fn convert_rotation_to_matrix(rotation_degrees: f64) -> [f64; 4] {
    let r = rotation_degrees.to_radians();
    [r.cos(), -r.sin(), r.sin(), r.cos()]
}

// Faithful float→hex (uppercase A-F), with a small cap on fractional digits
// to avoid infinite loops on repeating hex fractions (e.g., 0.1).
fn float_to_hex(x: f64) -> String {
    if x.is_sign_negative() {
        return format!("-{}", float_to_hex(-x));
    }
    let int_part = x.trunc() as u128;
    let frac = x - int_part as f64;

    // Integer digits
    let mut int_str = String::new();
    if int_part == 0 {
        // Matches Python behavior: if integer=0 and fraction>0, we start with ""
        // (Python later returns "" if fraction==0, we keep that behavior below)
    } else {
        let mut n = int_part;
        let mut tmp = Vec::<char>::new();
        while n > 0 {
            let rem = (n % 16) as u32;
            n /= 16;
            tmp.push(std::char::from_digit(rem, 16).unwrap().to_ascii_uppercase());
        }
        tmp.reverse();
        int_str.extend(tmp);
    }

    if frac == 0.0 {
        return int_str; // could be "" if x==0.0, like Python
    }

    // Fractional digits (capped)
    let mut frac_str = String::new();
    let mut f = frac;
    let mut steps = 0usize;
    while f > 0.0 && steps < 32 {
        f *= 16.0;
        let digit = f.trunc() as u32;
        f -= digit as f64;
        frac_str.push(
            std::char::from_digit(digit, 16)
                .unwrap()
                .to_ascii_uppercase(),
        );
        steps += 1;
    }

    if int_str.is_empty() {
        format!(".{}", frac_str)
    } else {
        format!("{}.{}", int_str, frac_str)
    }
}

pub fn generate_transaction_id(
    method: &str,
    request_path: &str,
    random_keyword: &str,
    random_number: u8,
    key: &[u8],
    animation_key: &str,
    time_now: Option<i64>,
) -> String {
    use sha2::Digest;

    // Python: floor((time.time()*1000 - 1682924400*1000)/1000)
    let now_secs = match time_now {
        Some(t) => t,
        None => {
            let ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock went backwards")
                .as_millis() as i128;
            let base_ms = 1_682_924_400i128 * 1000; // 2023-05-01T00:00:00Z
            ((ms - base_ms) / 1000) as i64
        }
    };

    // 4 little-endian bytes like [(t >> (i*8)) & 0xFF for i in range(4)]
    let time_now_bytes = (now_secs as u32).to_le_bytes(); // [u8;4]

    // SHA-256 of "{method}!{path}!{time_now}{random_keyword}{animation_key}"
    let to_hash = format!(
        "{}!{}!{}{}{}",
        method, request_path, now_secs, random_keyword, animation_key
    );
    let hash = sha2::Sha256::digest(to_hash.as_bytes());
    let hash_first_16 = &hash[..16];

    // Build bytes: key_bytes + time_now_bytes + hash[:16] + random_number
    let mut bytes_arr = Vec::with_capacity(key.len() + 4 + 16 + 1);
    bytes_arr.extend_from_slice(key);
    bytes_arr.extend_from_slice(&time_now_bytes);
    bytes_arr.extend_from_slice(hash_first_16);
    bytes_arr.push(random_number);

    // random_num in [0,255]
    let random_num: u8 = rand::rng().random();

    // out = [random_num] + XOR(each byte with random_num)
    let mut out = Vec::with_capacity(1 + bytes_arr.len());
    out.push(random_num);
    out.extend(bytes_arr.iter().map(|b| b ^ random_num));

    // Base64 without padding (Python: .strip("="))
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(out)
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn interpolate(self, other: Self, f: f64) -> Self {
        let r = Self::interpolate_value(self.r, other.r, f);
        let g = Self::interpolate_value(self.g, other.g, f);
        let b = Self::interpolate_value(self.b, other.b, f);

        Self::new(r, g, b)
    }

    fn interpolate_value(a: u8, b: u8, f: f64) -> u8 {
        math::round::half_to_even(interpolate(a as f64, b as f64, f).clamp(0.0, 255.0), 0) as u8
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}

fn interpolate(from_val: f64, to_val: f64, f: f64) -> f64 {
    from_val * (1.0 - f) + to_val * f
}

#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]
#![forbid(unsafe_code)]
//! Encoding and decoding of the `x-xp-forwarded-for` header used by the X (Twitter) GraphQL API.

use aes_gcm::{
    Aes256Gcm, KeyInit,
    aead::{AeadInOut, Generate, Nonce},
};
use chrono::{DateTime, Utc};
use sha2::Digest;
use std::borrow::Cow;

const BASE_KEY: &str = "0e6be1f1e21ffc33590b888fd4dc81b19713e570e805d4e5df80a493c9571a05";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("AES-GCM error")]
    AesGcm(#[from] aes_gcm::Error),
    #[error("Decoding input too short: {0} bytes")]
    InputTooShort(usize),
    #[error("Invalid hex string")]
    Hex(#[from] hex::FromHexError),
    #[error("Invalid UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("JSON decoding error")]
    Json(#[from] serde_json::Error),
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct Payload<'a> {
    navigator_properties: NavigatorProperties<'a>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct NavigatorProperties<'a> {
    #[serde(rename = "hasBeenActive")]
    has_been_active: BooleanString,
    #[serde(rename = "userAgent")]
    user_agent: Cow<'a, str>,
    webdriver: BooleanString,
}

#[derive(serde::Deserialize, serde::Serialize)]
enum BooleanString {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

/// Encoder and decoder for the `x-xp-forwarded-for` header, parameterized by a base key.
pub struct Generator {
    base_key: Cow<'static, str>,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(BASE_KEY)
    }
}

impl Generator {
    /// Creates a generator from a base key (borrowed or owned).
    pub fn new(base_key: impl Into<Cow<'static, str>>) -> Self {
        Self {
            base_key: base_key.into(),
        }
    }

    /// Encodes the navigator payload for the given guest ID into the hex-encoded header value.
    pub fn encode(
        &self,
        guest_id: &str,
        user_agent: &str,
        created_at: DateTime<Utc>,
    ) -> Result<String, Error> {
        let payload = Payload {
            navigator_properties: NavigatorProperties {
                has_been_active: BooleanString::False,
                user_agent: user_agent.into(),
                webdriver: BooleanString::False,
            },
            created_at,
        };

        let json = serde_json::to_vec(&payload)?;
        let bytes = self.encode_raw(guest_id, &json)?;

        Ok(hex::encode(bytes))
    }

    /// Decodes a hex-encoded header value for the given guest ID into its user agent and timestamp.
    pub fn decode(&self, guest_id: &str, value: &str) -> Result<(String, DateTime<Utc>), Error> {
        let bytes = hex::decode(value)?;
        let decoded_bytes = self.decode_raw(guest_id, &bytes)?;
        let decoded_str = std::str::from_utf8(&decoded_bytes)?;
        let payload: Payload<'_> = serde_json::from_str(decoded_str)?;

        Ok((
            payload.navigator_properties.user_agent.to_string(),
            payload.created_at,
        ))
    }

    fn derive_key(&self, guest_id: &str) -> [u8; 32] {
        // Feed the hasher incrementally to avoid allocating a joined `String` per call.
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.base_key.as_bytes());
        hasher.update(guest_id.as_bytes());
        hasher.finalize().into()
    }

    fn cipher(&self, guest_id: &str) -> Aes256Gcm {
        let key_bytes = self.derive_key(guest_id);
        Aes256Gcm::new(&key_bytes.into())
    }

    /// Encrypts `text` for the given guest ID, returning the nonce-prefixed ciphertext.
    pub fn encode_raw(&self, guest_id: &str, text: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
        let nonce = Nonce::<Aes256Gcm>::generate();

        let mut buffer = Vec::with_capacity(nonce.len() + text.len() + 16);
        buffer.extend_from_slice(text);

        let cipher = self.cipher(guest_id);

        cipher.encrypt_in_place(&nonce, &[], &mut buffer)?;

        buffer.splice(0..0, nonce);

        Ok(buffer)
    }

    /// Decrypts a nonce-prefixed ciphertext for the given guest ID, returning the plaintext.
    pub fn decode_raw(&self, guest_id: &str, bytes: &[u8]) -> Result<Vec<u8>, Error> {
        // `first_chunk` splits off the fixed-size nonce prefix without any fallible slicing.
        let nonce_bytes = *bytes
            .first_chunk::<12>()
            .ok_or(Error::InputTooShort(bytes.len()))?;
        let nonce = Nonce::<Aes256Gcm>::from(nonce_bytes);

        let mut buffer = Vec::with_capacity(bytes.len() - nonce_bytes.len());
        buffer.extend_from_slice(&bytes[nonce_bytes.len()..]);

        let cipher = self.cipher(guest_id);

        cipher.decrypt_in_place(&nonce, &[], &mut buffer)?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{SubsecRound, Utc};

    #[test]
    fn round_trip() {
        let guest_id = "v1%3A176468125390477869";
        let created_at = Utc::now().round_subsecs(3);
        let user_agent = "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36";

        let generator = super::Generator::default();

        let encoded = generator.encode(guest_id, user_agent, created_at).unwrap();

        let (decoded_user_agent, decoded_created_at) =
            generator.decode(guest_id, &encoded).unwrap();

        assert_eq!(user_agent, decoded_user_agent);
        assert_eq!(created_at, decoded_created_at);
    }
}

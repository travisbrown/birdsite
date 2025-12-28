use aes_gcm::{AeadCore, Aes256Gcm, KeyInit, aead::AeadMutInPlace};
use chrono::{DateTime, Utc};
use sha2::Digest;
use std::borrow::Cow;

const BASE_KEY: &str = "0e6be1f1e21ffc33590b888fd4dc81b19713e570e805d4e5df80a493c9571a05";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("AES-GCM error")]
    AesGcm(aes_gcm::Error),
    #[error("Decoding input too short")]
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

pub struct Generator {
    base_key: &'static str,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(BASE_KEY)
    }
}

impl Generator {
    pub fn new(base_key: &'static str) -> Self {
        Self { base_key }
    }

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

        let bytes = self
            .encode_raw(guest_id, serde_json::json!(payload).to_string().as_bytes())
            .map_err(Error::AesGcm)?;

        Ok(hex::encode(bytes))
    }

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
        sha2::Sha256::digest(format!("{}{}", self.base_key, guest_id).as_bytes()).into()
    }

    fn cipher(&self, guest_id: &str) -> Aes256Gcm {
        let key_bytes = self.derive_key(guest_id);
        Aes256Gcm::new(&key_bytes.into())
    }

    pub fn encode_raw(&self, guest_id: &str, text: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
        let nonce = Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);

        let mut buffer = Vec::with_capacity(128);
        buffer.extend_from_slice(text);

        let mut cipher = self.cipher(guest_id);

        cipher.encrypt_in_place(&nonce, &[], &mut buffer)?;

        buffer.splice(0..0, nonce);

        Ok(buffer)
    }

    pub fn decode_raw(&self, guest_id: &str, bytes: &[u8]) -> Result<Vec<u8>, Error> {
        if bytes.len() < 12 {
            Err(Error::InputTooShort(bytes.len()))
        } else {
            let nonce = bytes[0..12].into();

            let mut buffer = Vec::with_capacity(128);
            buffer.extend_from_slice(&bytes[12..]);

            let mut cipher = self.cipher(guest_id);

            cipher
                .decrypt_in_place(nonce, &[], &mut buffer)
                .map_err(Error::AesGcm)?;

            Ok(buffer)
        }
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
        assert_eq!(decoded_created_at, decoded_created_at);
    }
}

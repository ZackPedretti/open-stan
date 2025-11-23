use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use pbkdf2::hmac::digest::KeyInit;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use serde::Serialize;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const ORIGIN: &[u8] = b"https://nws-main.hove.io";
const SALT: &[u8] = b"mon-sel-fixe-ou-dynamique";
const ITERATIONS: u32 = 100_000;
const CLIENT_NAME: &str = "stan";

#[derive(Serialize)]
struct Payload {
    #[serde(rename = "clientName")]
    client_name: String,
    #[serde(rename = "tokenId")]
    token_id: String,
    iat: u64,
}

impl Payload {
    fn new() -> Self {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self { client_name: CLIENT_NAME.to_string(), token_id: Uuid::new_v4().to_string(), iat }
    }
}

fn generate_crypto_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(ORIGIN, SALT, ITERATIONS, &mut key);
    key
}

pub(crate) fn create_token() -> String {
    let mut iv = [0u8; 12];
    rand::rng().fill_bytes(&mut iv);
    let key = generate_crypto_key();
    let payload = Payload::new();
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Aes256Gcm::new_from_slice failed");
    let plaintext = serde_json::to_vec(&payload).unwrap();
    let ciphertext = cipher.encrypt(Nonce::from_slice(&iv), plaintext.as_ref()).unwrap();

    let iv = BASE64_STANDARD.encode(iv);
    let ciphertext = BASE64_STANDARD.encode(ciphertext);

    format!("{iv}.{ciphertext}")
}

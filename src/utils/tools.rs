use argon2::{ Config, ThreadMode, Variant, Version };
use rand::rngs::OsRng;
use rand::RngCore;
use regex::Regex;
use ttaw::metaphone::encoding;
use jsonwebtoken::{
    encode,
    EncodingKey,
    Header
};
use chrono::{ Utc, Duration };

use crate::types::RefreshClaims;

pub fn hash_password(password: &String) -> String {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536, // Memory cost (in Kibibytes)
        time_cost: 10, // Number of iterations
        lanes: 4, // Number of lanes (parallel computation units)
        thread_mode: ThreadMode::Sequential,
        secret: &[],
        ad: &[],
        hash_length: 32, // Output hash length in bytes
    };

    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();

    hash
}

lazy_static::lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
}

pub fn input_validator(input: &str, max_length: usize) -> bool {
    if input.len() > max_length {
        return false;
    }

    USERNAME_REGEX.is_match(input)
}

lazy_static::lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
}
pub fn is_email_valid(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

pub fn metaphone_encoding(input: &str) -> String {
    encoding(input).primary
}

pub async fn generate_token(id: i32, secret: String, duration: Duration) -> String {
    let exp: usize = (Utc::now() + duration).timestamp() as usize;
    let claims: RefreshClaims = RefreshClaims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    ).unwrap();
    token
}

use std::io::{Error, ErrorKind};

use sodiumoxide::crypto::pwhash::argon2id13;

pub async fn hash(password: &str) -> Result<(String, argon2id13::HashedPassword), Error> {
    sodiumoxide::init().unwrap();

    let hash = argon2id13::pwhash(
        password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .map_err(|_| ErrorKind::Other)?;

    let texthash = std::str::from_utf8(&hash.0).map_err(|_| ErrorKind::Other)?;
    Ok((texthash.to_string(), hash))
}

pub fn verify(hash: [u8; 128], passwd: &str) -> bool {
    sodiumoxide::init().unwrap();
    match argon2id13::HashedPassword::from_slice(&hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}

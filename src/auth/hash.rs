use pbkdf2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Pbkdf2,
};

pub async fn hash(password: &str) -> Result<String, Error> {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let hash = Pbkdf2.hash_password(password, &salt)?;
    Ok(hash.to_string())
}

pub fn verify(password: &str, hash: &str) -> bool {
    let password = password.as_bytes();
    let parsed_hash = PasswordHash::new(&hash);
    match parsed_hash {
        Err(_) => false,
        Ok(val) => Pbkdf2.verify_password(password, &val).is_ok(),
    }
}

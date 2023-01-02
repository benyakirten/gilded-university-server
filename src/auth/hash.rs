use pbkdf2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Pbkdf2,
};

pub fn hash(password: &str) -> Result<String, Error> {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let hash = Pbkdf2.hash_password(password, &salt)?;
    Ok(hash.to_string())
}

pub fn verify(password: &str, hash: &str) -> Result<(), Error> {
    let password = password.as_bytes();
    let parsed_hash = PasswordHash::new(hash)?;
    Pbkdf2.verify_password(password, &parsed_hash)
}

#[cfg(test)]
mod test {
    use pbkdf2::{
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Pbkdf2,
    };
    use rand_core::OsRng;

    use super::{hash, verify};

    #[test]
    fn create_password_hash() {
        let result = hash("testpassword").unwrap();
        let parsed_hash = PasswordHash::new(&result).unwrap();
        let got = Pbkdf2.verify_password("testpassword".as_bytes(), &parsed_hash);
        assert!(got.is_ok());
    }

    #[test]
    fn verify_match_returns_ok() {
        let password = "testpassword".as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password, &salt).unwrap().to_string();
        let got = verify("testpassword", &password_hash);

        assert!(got.is_ok());
    }

    #[test]
    fn verify_mismatch_returns_error() {
        let password = "testpassword".as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password, &salt).unwrap().to_string();
        let got = verify("differentpassword", &password_hash);

        assert!(got.is_err());
    }
}

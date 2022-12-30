use std::{
    env,
    io::{Error, ErrorKind},
};

use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JSONError, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

use ::entity::sea_orm_active_enums::Role;

pub async fn create_jwt(uid: &str, role: &Role) -> Result<String, JSONError> {
    let binding = get_jwt_secret();
    let secret = binding.as_bytes();
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims::new(uid, role, expiration);
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret))
}

pub fn authorize(role: &Role, token: &str) -> Result<String, Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| ErrorKind::Other)?;

    let decoded_role = Role::from_str(&decoded.claims.role);
    match decoded_role.meets_requirements(role) {
        true => Ok(decoded.claims.sub),
        false => Err(ErrorKind::PermissionDenied.into()),
    }
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET environment variable not found")
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: i64,
}

impl Claims {
    pub fn new(sub: &str, role: &Role, exp: i64) -> Claims {
        Claims {
            sub: sub.to_owned(),
            role: role.to_string(),
            exp: exp.to_owned(),
        }
    }
}

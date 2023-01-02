use std::str::FromStr;

use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JSONError, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::graphql::errors::AuthorizationError;
use entity::sea_orm_active_enums::Role;
use gilded_university_server::get_env;

pub fn create_jwt(uid: &Uuid, role: &Role) -> Result<String, JSONError> {
    let binding = get_env("JWT_SECRET");
    let secret = binding.as_bytes();
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims::new(uid, role, expiration);
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret))
}

#[allow(dead_code)]
pub fn authorize(role: &Role, token: &str) -> Result<Uuid, AuthorizationError> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_env("JWT_SECRET").as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|e| AuthorizationError::DecodingError(e.to_string()))?;

    // I'm not sure why, but I think it's because of serialization/deserialization
    // But the string will be "'{Role}'" rather than "{Role}"
    let decoded_role = &decoded.claims.role;
    let decoded_role = &decoded_role[1..decoded_role.len() - 1];
    let decoded_role = Role::from_str(decoded_role).unwrap_or(Role::Guest);
    match decoded_role.meets_requirements(role) {
        true => Ok(decoded.claims.sub),
        false => Err(AuthorizationError::InsufficientPermission {
            required: role.to_str(),
            permission: decoded_role.to_str(),
        }),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: i64,
}

impl Claims {
    pub fn new(sub: &Uuid, role: &Role, exp: i64) -> Claims {
        Claims {
            sub: sub.to_owned(),
            role: role.to_string(),
            exp: exp.to_owned(),
        }
    }
}

#[cfg(test)]
mod test_create_jwt {
    use std::env;

    use chrono::{Duration, Utc};
    use dotenvy::dotenv;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use sea_orm::prelude::Uuid;

    use super::{create_jwt, Claims};
    use entity::sea_orm_active_enums::Role;

    #[test]
    fn create_jwt_success() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let exp = Utc::now()
            .checked_add_signed(Duration::seconds(60))
            .unwrap()
            .timestamp();
        let res = create_jwt(&id, &Role::Student).unwrap();

        let claim = decode::<Claims>(
            &res,
            &DecodingKey::from_secret("jwtsecret".as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .unwrap()
        .claims;

        assert_eq!(claim.sub, id);
        assert_eq!(claim.role, "'Student'");
        assert_eq!(claim.exp, exp);
    }
}

#[cfg(test)]
mod test_authorize {
    use std::env;

    use chrono::{Duration, Utc};
    use dotenvy::dotenv;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use sea_orm::prelude::Uuid;

    use super::{authorize, Claims};
    use entity::sea_orm_active_enums::Role;

    fn create_test_jwt(id: &Uuid, role: &Role) -> String {
        let secret = "jwtsecret".as_bytes();
        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(60))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims::new(id, role, expiration);
        let header = Header::new(Algorithm::HS512);
        encode(&header, &claims, &EncodingKey::from_secret(secret)).unwrap()
    }

    #[test]
    fn authorization_success() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let token = create_test_jwt(&id, &Role::Admin);
        let got = authorize(&Role::Guest, &token).unwrap();

        assert_eq!(got, id);
    }

    #[test]
    fn authorization_success_on_tie() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let token = create_test_jwt(&id, &Role::Student);
        let got = authorize(&Role::Student, &token).unwrap();

        assert_eq!(got, id);
    }

    #[test]
    fn authorization_failure() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let token = create_test_jwt(&id, &Role::Guest);
        let got = authorize(&Role::Admin, &token);

        assert!(got.is_err());

        let err = got.err().unwrap().to_string();
        assert_eq!(
            err,
            "Required permission is Admin but user has permission Guest"
        );
    }
}

#[cfg(test)]
mod test_claims {}

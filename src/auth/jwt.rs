use std::str::FromStr;
use std::time::Duration;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::{
    errors::{AuthorizationError, TimeError},
    get_env,
    time::{Time, HOUR_IN_SECONDS},
};
use entity::sea_orm_active_enums::Role;

pub fn create_jwt(uid: &Uuid, role: &Role) -> Result<String, AuthorizationError> {
    let binding = get_env("JWT_SECRET");
    let secret = binding.as_bytes();

    let claims = Claims::new(uid, role, Duration::from_secs(HOUR_IN_SECONDS.into()))
        .map_err(|e| AuthorizationError::EncodingError(e.to_string()))?;
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret))
        .map_err(|e| AuthorizationError::EncodingError(e.to_string()))
}

#[allow(dead_code)]
pub fn authorize(role: &Role, token: &str) -> Result<Uuid, AuthorizationError> {
    let claims = get_claims_from_token(token)?;

    let decoded_role = Role::from_str(&claims.role).unwrap_or(Role::Guest);
    match decoded_role.meets_requirements(role) {
        true => Ok(claims.sub),
        false => Err(AuthorizationError::InsufficientPermission {
            required: role.to_str(),
            permission: decoded_role.to_str(),
        }),
    }
}

pub fn get_claims_from_token(token: &str) -> Result<Claims, AuthorizationError> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_env("JWT_SECRET").as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|e| AuthorizationError::DecodingError(e.to_string()))
    .map(|decoded| decoded.claims)?;

    Ok(claims)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: u64,
}

impl Claims {
    pub fn new(sub: &Uuid, role: &Role, exp: Duration) -> Result<Claims, TimeError> {
        let expiration = Time::now_plus_duration(exp)?;
        let claim = Claims {
            sub: sub.to_owned(),
            role: role.to_str(),
            exp: expiration.as_secs(),
        };
        Ok(claim)
    }
}

#[cfg(test)]
mod test_create_jwt {
    use std::env;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use dotenvy::dotenv;
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use sea_orm::prelude::Uuid;

    use crate::time::HOUR_IN_SECONDS;

    use super::{create_jwt, Claims};
    use entity::sea_orm_active_enums::Role;

    #[test]
    fn create_jwt_success() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(HOUR_IN_SECONDS.into()))
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let res = create_jwt(&id, &Role::Student).unwrap();

        let claim = decode::<Claims>(
            &res,
            &DecodingKey::from_secret("jwtsecret".as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .unwrap()
        .claims;

        assert_eq!(claim.sub, id);
        assert_eq!(claim.role, "Student");
        assert_eq!(claim.exp, exp);
    }
}

#[cfg(test)]
mod test_authorize {
    use std::env;
    use std::time::Duration;

    use dotenvy::dotenv;
    use sea_orm::prelude::Uuid;

    use super::authorize;
    use crate::{
        testutils::create_test_jwt,
        time::{Time, HOUR_IN_SECONDS},
    };
    use entity::sea_orm_active_enums::Role;

    #[test]
    fn authorization_success() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let token = create_test_jwt(
            &id,
            &Role::Admin,
            Time::now_plus_duration(Duration::from_secs(HOUR_IN_SECONDS.into()))
                .unwrap()
                .as_secs(),
        );
        let got = authorize(&Role::Guest, &token).unwrap();

        assert_eq!(got, id);
    }

    #[test]
    fn authorization_success_on_role_tie() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let token = create_test_jwt(
            &id,
            &Role::Student,
            Time::now_plus_duration(Duration::from_secs(HOUR_IN_SECONDS.into()))
                .unwrap()
                .as_secs(),
        );
        let got = authorize(&Role::Student, &token).unwrap();

        assert_eq!(got, id);
    }

    #[test]
    fn authorization_failure_on_role_too_low() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let token = create_test_jwt(
            &id,
            &Role::Guest,
            Time::now_plus_duration(Duration::from_secs(HOUR_IN_SECONDS.into()))
                .unwrap()
                .as_secs(),
        );
        let got = authorize(&Role::Admin, &token);

        assert!(got.is_err());

        let err = got.err().unwrap().to_string();
        assert_eq!(
            err,
            "Required permission is Admin but user has permission Guest"
        );
    }

    #[test]
    fn authorization_faiure_on_token_expired() {
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let token = create_test_jwt(&id, &Role::Guest, 100);
        let got = authorize(&Role::Admin, &token);

        assert!(got.is_err());

        let err = got.err().unwrap().to_string();
        assert_eq!(err, "Unable to decode JWT: ExpiredSignature");
    }
}

#[cfg(test)]
mod test_claims_from_token {
    use std::env;

    use dotenvy::dotenv;
    use entity::sea_orm_active_enums::Role;
    use sea_orm::prelude::Uuid;

    use crate::{testutils::create_test_jwt, time::Time};

    use super::get_claims_from_token;

    #[test]
    fn fail_on_false_token() {
        let res = get_claims_from_token("abcabcabc");
        assert!(res.is_err());

        let err = res.err().unwrap();
        assert_eq!(err.to_string(), "Unable to decode JWT: InvalidToken")
    }

    #[test]
    fn succeed_on_valid_token() {
        dotenv().ok();
        env::set_var("JWT_SECRET", "jwtsecret");
        let id = Uuid::new_v4();
        let now_plus_hour = Time::hour_hence().unwrap().as_secs();
        let token = create_test_jwt(&id, &Role::Admin, now_plus_hour);

        let res = get_claims_from_token(&token);
        assert!(res.is_ok());

        let claims = res.ok().unwrap();
        assert_eq!(claims.role, "Admin");
        assert_eq!(claims.sub, id);
        assert_eq!(claims.exp, now_plus_hour);
    }
}

#[cfg(test)]
mod test_claims {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use entity::sea_orm_active_enums::Role;
    use sea_orm::prelude::Uuid;

    use crate::time::HOUR_IN_SECONDS;

    use super::Claims;

    #[test]
    fn create_new_claim() {
        let id = Uuid::new_v4();
        let got = Claims::new(
            &id,
            &Role::Teacher,
            Duration::from_secs(HOUR_IN_SECONDS.into()),
        )
        .unwrap();

        assert_eq!(
            got.exp,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600
        );
        assert_eq!(got.role, "Teacher");
        assert_eq!(got.sub, id);
    }
}

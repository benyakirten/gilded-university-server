use std::sync::Arc;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::{prelude::Uuid, DatabaseBackend, DatabaseConnection, MockDatabase, ModelTrait};

use crate::{auth::jwt::Claims, graphql::schema::Context};
use entity::sea_orm_active_enums::Role;
use migration::DbErr;

#[allow(dead_code)]
pub fn create_mock_database() -> MockDatabase {
    MockDatabase::new(DatabaseBackend::Postgres)
}
#[allow(dead_code)]
pub fn create_mock_conn<T: ModelTrait>(results: Vec<Vec<T>>) -> DatabaseConnection {
    let db = create_mock_database().append_query_results(results);
    db.into_connection()
}

#[allow(dead_code)]
pub fn create_mock_context<T: ModelTrait>(results: Vec<Vec<T>>, token: Option<String>) -> Context {
    let token = token.unwrap_or_default();
    let connection = Arc::new(create_mock_conn(results));
    Context { token, connection }
}

#[allow(dead_code)]
pub fn create_mock_errored_conn(results: Vec<DbErr>) -> DatabaseConnection {
    let db = create_mock_database().append_query_errors(results);
    db.into_connection()
}

#[allow(dead_code)]
pub fn create_errored_context(results: Vec<DbErr>, token: Option<String>) -> Context {
    let token = token.unwrap_or_default();
    let connection = Arc::new(create_mock_errored_conn(results));
    Context { connection, token }
}

#[allow(dead_code)]
pub fn create_test_jwt(id: &Uuid, role: &Role, time: u64) -> String {
    let secret = "jwtsecret".as_bytes();
    let claims = Claims {
        sub: id.to_owned(),
        role: role.to_str(),
        exp: time,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret)).unwrap()
}

#[allow(dead_code)]
pub fn print_response_body(body: &Vec<u8>) {
    let str = String::from_utf8(body.to_vec()).unwrap();
    println!("{}", str);
}

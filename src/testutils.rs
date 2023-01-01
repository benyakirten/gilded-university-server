use std::sync::Arc;

use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, ModelTrait};

use crate::graphql::schema::Context;
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

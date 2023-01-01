use std::sync::Arc;

use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, ModelTrait};

use crate::graphql::schema::Context;
#[allow(dead_code)]
pub fn create_mock_conn<T: ModelTrait>(results: Vec<Vec<T>>) -> DatabaseConnection {
    let db = MockDatabase::new(DatabaseBackend::Postgres).append_query_results(results);
    db.into_connection()
}

#[allow(dead_code)]
pub fn create_mock_context<T: ModelTrait>(results: Vec<Vec<T>>, token: Option<String>) -> Context {
    let token = token.unwrap_or_else(|| "".to_string());
    let connection = Arc::new(create_mock_conn(results));
    Context { token, connection }
}

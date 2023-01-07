use std::{env, sync::Arc};

use graphql::schema::create_schema;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::graphql::schema::Context;
use migration::{DbErr, Migrator, MigratorTrait};
use warp::{filters::BoxedFilter, http::Response, Filter};

pub mod auth;
pub mod errors;
pub mod graphql;
pub mod testutils;
pub mod time;

pub async fn connect_to_database(key: &str) -> Result<DatabaseConnection, DbErr> {
    let url = get_env(key);
    let opt = ConnectOptions::new(url);
    // Connections options can be inserted

    let connection = Database::connect(opt).await?;
    Migrator::up(&connection, None).await?;
    Ok(connection)
}

pub fn create_gql_filter(connection: DatabaseConnection) -> BoxedFilter<(Response<Vec<u8>>,)> {
    let connection = Arc::new(connection);
    let state = warp::any()
        .and(warp::header::optional::<String>("Authorization"))
        .map(move |auth: Option<String>| -> Context {
            let token = get_token_from_header(auth);
            Context {
                connection: connection.clone(),
                token,
            }
        });
    juniper_warp::make_graphql_filter(create_schema(), state.boxed())
}

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} environment variable is not defined", key))
}

pub fn get_token_from_header(header: Option<String>) -> String {
    match header {
        None => "".to_string(),
        Some(header) => {
            let mut split = header.split_whitespace();
            match split.next() {
                None => "".to_string(),
                Some(bearer) => {
                    if bearer != "Bearer" {
                        return "".to_string();
                    }
                    let token = split.next();
                    match token {
                        None => "".to_string(),
                        Some(val) => val.to_string(),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::get_env;
    #[test]
    fn get_env_success() {
        env::remove_var("TEST_VALUE");
        env::set_var("TEST_VALUE", "secretkey");

        let res = get_env("TEST_VALUE");
        assert_eq!(res, "secretkey".to_string());

        env::remove_var("TEST_VALUE");
    }

    #[test]
    #[should_panic(expected = "TEST_VALUE environment variable is not defined")]
    fn get_env_failure() {
        env::remove_var("TEST_VALUE");
        get_env("TEST_VALUE");
    }

    #[test]
    fn get_token_returns_empty_if_no_header() {}

    #[test]
    fn get_token_returns_empty_if_header_is_empty_string() {}
    #[test]
    fn get_token_returns_empty_if_first_word_not_bearer() {}

    #[test]
    fn get_token_returns_empty_if_no_token() {}

    #[test]
    fn get_token_returns_token_otherwise() {}
}

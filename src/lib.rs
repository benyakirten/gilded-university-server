use std::env;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use migration::{DbErr, Migrator, MigratorTrait};

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

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} environment variable is not defined", key))
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
}

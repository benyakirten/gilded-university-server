use std::env;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect_to_database() -> DatabaseConnection {
    let uri = get_env("DATABASE_URI");
    let opt = ConnectOptions::new(uri);
    // Connections options can be inserted
    let connection = Database::connect(opt).await;

    match connection {
        Err(e) => panic!("Unable to connect to database: {}", e.to_string()),
        Ok(connection) => connection,
    }
}

pub fn get_env(key: &str) -> String {
    env::var(key).expect(format!("{} environment variable is not defined", key).as_str())
}

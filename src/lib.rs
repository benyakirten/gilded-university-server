use std::env;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

pub async fn connect_to_database() -> Result<DatabaseConnection> {
    let uri = get_env("DATABASE_URL");
    let opt = ConnectOptions::new(uri);
    // Connections options can be inserted

    let connection = Database::connect(opt).await?;
    Migrator::up(&connection, None).await?;
    Ok(connection)
}

pub fn get_env(key: &str) -> String {
    env::var(key).expect(format!("{} environment variable is not defined", key).as_str())
}

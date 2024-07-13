use crate::config::DatabaseConfig;
use crate::error::DaoError;
pub use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;
pub mod config;
pub mod dao;
pub mod dto;
pub mod error;
pub async fn init_database_connection(
    database_config: &DatabaseConfig,
) -> Result<DatabaseConnection, DaoError> {
    let mut database_connect_options =
        ConnectOptions::new(database_config.generate_url()?).to_owned();
    database_connect_options.max_connections(database_config.max_connections());
    database_connect_options.min_connections(database_config.min_connections());
    database_connect_options.acquire_timeout(Duration::from_secs(
        database_config.connection_acquire_timeout().into(),
    ));
    let database = Database::connect(database_connect_options).await?;
    Ok(database)
}

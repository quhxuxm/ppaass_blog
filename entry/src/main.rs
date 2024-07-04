use std::sync::Arc;
use std::time::Duration;
use anyhow::{Ok, Result};
use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use migration::{Migrator, MigratorTrait};
use migration::sea_orm::{ConnectOptions, Database, DatabaseConnection};
mod bo;
mod handler;
const DB_URL: &str = "mysql://root:123456@localhost/ppaass-db";

async fn init_database_connection() -> Result<Arc<DatabaseConnection>> {
    let mut database_connect_options = ConnectOptions::new(DB_URL).to_owned();
    database_connect_options.max_connections(32);
    database_connect_options.min_connections(32);
    database_connect_options.acquire_timeout(Duration::from_secs(1));
    let database = Database::connect(database_connect_options).await?;
    // Migrator::down(&database, None).await?;
    Migrator::up(&database, None).await?;
    Ok(Arc::new(database))
}

#[tokio::main]
async fn main() -> Result<()> {
    let database = init_database_connection().await?;
    let router = Router::new()
        .route("/user/register", post(handler::user::register))
        .route("/user/:username", get(handler::user::get))
        .with_state(database);
    let tcp_listener = TcpListener::bind("0.0.0.0:9090").await?;
    axum::serve(tcp_listener, router).await?;
    Ok(())
}

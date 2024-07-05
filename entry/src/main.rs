use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use anyhow::{Ok, Result};
use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::format::{DefaultFields, Format, Full};
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::fmt::time::ChronoUtc;
use migration::{Migrator, MigratorTrait};
use migration::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::config::{Config, DatabaseConfig, LogConfig};
mod bo;
mod config;
mod handler;

const CONFIG_FILE_PATH: &str = "resource/config.toml";
async fn init_database_connection(
    database_config: &DatabaseConfig,
) -> Result<Arc<DatabaseConnection>> {
    let mut database_connect_options =
        ConnectOptions::new(database_config.generate_url()?).to_owned();
    database_connect_options.max_connections(database_config.max_connections());
    database_connect_options.min_connections(database_config.min_connections());
    database_connect_options.acquire_timeout(Duration::from_secs(
        database_config.connection_acquire_timeout().into(),
    ));
    let database = Database::connect(database_connect_options).await?;
    // Migrator::down(&database, None).await?;
    Migrator::up(&database, None).await?;
    Ok(Arc::new(database))
}

pub fn init_tracing_subscriber(
    log_config: &LogConfig,
) -> Result<(
    Subscriber<DefaultFields, Format<Full, ChronoUtc>, LevelFilter, NonBlocking>,
    WorkerGuard,
)> {
    let (trace_file_appender, trace_appender_guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily(
            Path::new(log_config.log_folder()),
            log_config.file_name_prefix(),
        ));
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(LevelFilter::from_str(log_config.max_log_level())?)
        .with_writer(trace_file_appender)
        .with_line_number(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(ChronoUtc::rfc_3339())
        .with_ansi(false)
        .finish();
    Ok((subscriber, trace_appender_guard))
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_content = read_to_string(CONFIG_FILE_PATH)?;
    let config = toml::from_str::<Config>(&config_content)?;
    let (subscriber, _tracing_guard) = init_tracing_subscriber(config.log())?;
    tracing::subscriber::set_global_default(subscriber)?;
    info!("Initialize log success.");
    let database = init_database_connection(&config.database()).await?;
    info!("Initialize database success.");
    let router = Router::new()
        .route("/user/register", post(handler::user::register))
        // .route("/user/:username", get(handler::user::get))
        .with_state(database);
    info!("Initialize http server route success.");
    let tcp_listener = TcpListener::bind("0.0.0.0:9090").await?;
    axum::serve(tcp_listener, router).await?;
    Ok(())
}

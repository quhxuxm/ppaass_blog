use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use anyhow::{Ok, Result};
use axum::middleware::from_extractor_with_state;
use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::format::{DefaultFields, Format, Full};
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::fmt::time::ChronoUtc;
use ppaass_blog_persistence::{DatabaseConnection, init_database_connection};
use crate::config::{Config, LogConfig};
use crate::extractor::auth_token::UserAuthToken;
use crate::state::ApplicationState;
mod bo;
mod config;
mod error;
mod extractor;
mod handler;
mod state;
const CONFIG_FILE_PATH: &str = "resource/config.toml";

fn init_tracing_subscriber(
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

fn init_router(database: DatabaseConnection, config: Config) -> Router {
    let state = ApplicationState::new(database, config);
    Router::new()
        .route("/blog/create", post(handler::blog::create_blog))
        .route("/post/create/:blog_token", post(handler::post::create_post))
        .layer(from_extractor_with_state::<UserAuthToken, ApplicationState>(state.clone()))
        .route("/user/register", post(handler::user::register_user))
        .route("/user/:username", get(handler::user::get_user))
        .route("/user/auth", post(handler::user::auth_user))
        .route("/blog/:blog_token", get(handler::blog::get_blog_detail))
        .route("/blog/list/:username", get(handler::blog::list_blogs))
        .route("/post/list/:blog_token", get(handler::post::list_posts))
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_content = read_to_string(CONFIG_FILE_PATH)?;
    let config = toml::from_str::<Config>(&config_content)?;
    let bind_address = *config.server().bind_address();
    let (subscriber, _tracing_guard) = init_tracing_subscriber(config.log())?;
    tracing::subscriber::set_global_default(subscriber)?;
    info!("Initialize log success.");
    let database = init_database_connection(config.database()).await?;
    info!("Initialize database success.");
    let router = init_router(database, config);
    info!("Initialize http server route success.");
    let tcp_listener = TcpListener::bind(bind_address).await?;
    axum::serve(tcp_listener, router).await?;
    Ok(())
}

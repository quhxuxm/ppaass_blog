use ppaass_blog_persistence::config::DatabaseConfig;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    file_name_prefix: String,
    max_log_level: String,
    log_folder: String,
}

impl LogConfig {
    pub fn file_name_prefix(&self) -> &str {
        &self.file_name_prefix
    }
    pub fn max_log_level(&self) -> &str {
        &self.max_log_level
    }
    pub fn log_folder(&self) -> &str {
        &self.log_folder
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtConfig {
    secret: String,
    expire_duration_seconds: u64,
}

impl JwtConfig {
    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn expire_duration_seconds(&self) -> u64 {
        self.expire_duration_seconds
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    bind_address: SocketAddr,
}

impl ServerConfig {
    pub fn bind_address(&self) -> &SocketAddr {
        &self.bind_address
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    database: DatabaseConfig,
    log: LogConfig,
    jwt: JwtConfig,
    server: ServerConfig,
}

impl Config {
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn log(&self) -> &LogConfig {
        &self.log
    }

    pub fn jwt(&self) -> &JwtConfig {
        &self.jwt
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
}

use std::net::SocketAddr;
use std::path::PathBuf;
use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "database_type")]
pub enum DatabaseConfig {
    PostgreSQL {
        host: String,
        port: u16,
        username: String,
        password: String,
        database_name: String,
        max_connections: u32,
        min_connections: u32,
        connection_acquire_timeout: u32,
    },
    MySQL {
        host: String,
        port: u16,
        username: String,
        password: String,
        database_name: String,
        max_connections: u32,
        min_connections: u32,
        connection_acquire_timeout: u32,
    },
    SQLiteMemory {
        database_name: String,
        max_connections: u32,
        min_connections: u32,
        connection_acquire_timeout: u32,
    },
    SQLiteFile {
        file_path: PathBuf,
        max_connections: u32,
        min_connections: u32,
        connection_acquire_timeout: u32,
    },
}

impl DatabaseConfig {
    pub fn connection_acquire_timeout(&self) -> u32 {
        match self {
            DatabaseConfig::PostgreSQL {
                connection_acquire_timeout,
                ..
            } => *connection_acquire_timeout,
            DatabaseConfig::MySQL {
                connection_acquire_timeout,
                ..
            } => *connection_acquire_timeout,
            DatabaseConfig::SQLiteMemory {
                connection_acquire_timeout,
                ..
            } => *connection_acquire_timeout,
            DatabaseConfig::SQLiteFile {
                connection_acquire_timeout,
                ..
            } => *connection_acquire_timeout,
        }
    }
    pub fn max_connections(&self) -> u32 {
        match self {
            DatabaseConfig::PostgreSQL {
                max_connections, ..
            } => *max_connections,
            DatabaseConfig::MySQL {
                max_connections, ..
            } => *max_connections,
            DatabaseConfig::SQLiteMemory {
                max_connections, ..
            } => *max_connections,
            DatabaseConfig::SQLiteFile {
                max_connections, ..
            } => *max_connections,
        }
    }

    pub fn min_connections(&self) -> u32 {
        match self {
            DatabaseConfig::PostgreSQL {
                min_connections, ..
            } => *min_connections,
            DatabaseConfig::MySQL {
                min_connections, ..
            } => *min_connections,
            DatabaseConfig::SQLiteMemory {
                min_connections, ..
            } => *min_connections,
            DatabaseConfig::SQLiteFile {
                min_connections, ..
            } => *min_connections,
        }
    }

    pub fn generate_url(&self) -> Result<String> {
        match self {
            DatabaseConfig::PostgreSQL {
                username,
                password,
                host,
                port,
                database_name,
                ..
            } => Ok(format!(
                "postgres://{username}:{password}@{host}:{port}/{database_name}"
            )),
            DatabaseConfig::MySQL {
                host,
                port,
                username,
                password,
                database_name,
                ..
            } => Ok(format!(
                "mysql://{username}:{password}@{host}:{port}/{database_name}"
            )),
            DatabaseConfig::SQLiteMemory { database_name, .. } => {
                Ok(format!("sqlite::memory://{database_name}"))
            }
            DatabaseConfig::SQLiteFile { file_path, .. } => {
                let file_path = file_path
                    .to_str()
                    .ok_or(anyhow!("Fail to generate SQLite file database url"))?;
                Ok(format!("sqlite://{file_path}"))
            }
        }
    }
}

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
    expire_duration_seconds: u64
}

impl JwtConfig{
    pub fn secret(&self)->&str{
        &self.secret
    }
    
    pub fn expire_duration_seconds(&self)->u64{
        self.expire_duration_seconds
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig{
    bind_address: SocketAddr
}

impl ServerConfig{
    pub fn bind_address(&self)->&SocketAddr{
        &self.bind_address
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    database: DatabaseConfig,
    log: LogConfig,
    jwt: JwtConfig,
    server:ServerConfig
}

impl Config {
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn log(&self) -> &LogConfig {
        &self.log
    }
    
    pub fn jwt(&self)->&JwtConfig{
        &self.jwt
    }
    
    pub fn server(&self)->&ServerConfig{
        &self.server
    }
}

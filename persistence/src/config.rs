use crate::error::DaoError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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

    pub fn generate_url(&self) -> Result<String, DaoError> {
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
                let file_path = file_path.to_str().ok_or(DaoError::DatabaseConfiguration(
                    "Fail to generate SQLite file database url".to_string(),
                ))?;
                Ok(format!("sqlite://{file_path}"))
            }
        }
    }
}

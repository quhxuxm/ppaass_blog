use crate::config::Config;
use ppaass_blog_persistence::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApplicationState {
    database: Arc<DatabaseConnection>,
    config: Arc<Config>,
}

impl ApplicationState {
    pub fn new(database: DatabaseConnection, config: Config) -> Self {
        Self {
            database: Arc::new(database),
            config: Arc::new(config),
        }
    }

    pub fn database(&self) -> &DatabaseConnection {
        &self.database
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

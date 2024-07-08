use std::sync::Arc;
use migration::sea_orm::DatabaseConnection;
use crate::config::Config;

#[derive(Clone)]
pub struct ApplicationState {
     database: Arc<DatabaseConnection>,
    config: Arc<Config>,
}

impl ApplicationState{
    
    pub fn new(database: DatabaseConnection, config: Config)->Self{
        Self{
            database: Arc::new(database),
            config: Arc::new(config)
        }
    }
    
    pub fn database(&self)->&DatabaseConnection{
        &self.database
    }
    
    pub fn config(&self)->&Config{
        &self.config
    }
}

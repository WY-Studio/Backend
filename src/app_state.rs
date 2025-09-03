use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::oauth2::OAuth2Config;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub oauth_config: OAuth2Config,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db),
            oauth_config: OAuth2Config::new(),
        }
    }
}

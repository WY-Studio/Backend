use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::{
    app_config::{JwtConfig, load_config},
    oauth2::OAuth2Config,
};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub oauth_config: OAuth2Config,
    pub jwt_config: JwtConfig,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        let cfg = load_config().expect("config.yaml 로드 실패");
        Self {
            db: Arc::new(db),
            oauth_config: OAuth2Config::new(),
            jwt_config: cfg.jwt,
        }
    }
}

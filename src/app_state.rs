use std::sync::Arc;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db),
        }
    }
}
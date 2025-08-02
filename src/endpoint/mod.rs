use axum::{Router, routing::get};

use crate::app_state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/ping", get(health_check))
        .with_state(app_state)
}

async fn health_check() -> &'static str {
    "pong"
}

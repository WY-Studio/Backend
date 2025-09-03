use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    app_state::AppState,
    features::auth::handlers::{oauth_login, oauth_login_callback},
};

pub fn auth_api_router(app_state: AppState) -> Router {
    Router::new()
        .route("/auth/{provider}/login", get(oauth_login))
        .route("/auth/{provider}/callback", get(oauth_login_callback))
        .with_state(Arc::new(app_state))
}

pub mod auth_endpoint;

use axum::{Router, middleware, routing::get};

use crate::{
    app_state::AppState, endpoint::auth_endpoint::auth_api_router,
    middleware::logger::performance_logger, swagger::swagger_ui,
};

//TODO 많아 질경우 router 나눠서 폴더로 관리하기
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .nest("/api", auth_api_router(app_state)) // /api 접두사로 중첩
        .nest("/api", Router::new().route("/ping", get(health_check)))
        .merge(swagger_ui()) // Swagger UI 추가
        .layer(middleware::from_fn(performance_logger))
}

/// 헬스 체크 엔드포인트
///
/// 서버가 정상적으로 동작하고 있는지 확인합니다.
#[utoipa::path(
    get,
    path = "/api/ping",
    responses(
        (status = 200, description = "서버 정상 동작", body = String)
    ),
    tag = "health"
)]
pub async fn health_check() -> &'static str {
    "pong"
}

use crate::middleware::{auth::BearerAuth, logger::PerformanceLogger};
use actix_web::{Responder, get, web};

use crate::{
    core::{error::AppError, response::Base},
    features::auth::handler::configure_auth,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(configure_auth)
            .service(health_check)
            .wrap(PerformanceLogger)
            .service(web::scope("").wrap(BearerAuth).service(protect_ping)),
    );
}

/// 서버 정상 동작 확인
///
/// 서버가 200 내려주면 살아있는거
#[utoipa::path(
    get,
    path = "/api/ping",
    responses(
        (status = 200, description = "서버 정상 동작", body = Base<String>)
    ),
    tag = "health"
)]
#[get("/ping")]
pub async fn health_check() -> Result<impl Responder, AppError> {
    let body = Base::success("pong".to_string());
    Ok(web::Json(body))
}

/// 헤더 토큰 동작확인
///
/// 아직은 헤더에 bearer 붙이고 아무거나 넣으면 동작
#[utoipa::path(
    get,
    path = "/api/protect_ping",
    responses(
        (status = 200, description = "헤더 토큰 확인 동작", body = Base<String>)
    ),
    security(("bearerAuth" = []),),
    tag = "health"
)]
#[get("/protect_ping")]
pub async fn protect_ping(req: actix_web::HttpRequest) -> Result<impl Responder, AppError> {
    // 간단한 토큰 확인 (기존 미들웨어 대체)
    let auth = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !auth.starts_with("Bearer ") || auth.len() <= 7 {
        return Err(AppError::Unauthorized("Unauthorized".to_string()));
    }

    let body = Base::success("pong".to_string());
    Ok(web::Json(body))
}

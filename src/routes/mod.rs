use crate::middleware::{auth::BearerAuth, logger::PerformanceLogger};
use actix_web::{Responder, get, web};

use crate::{
    core::{error::AppError, response::Base},
    features::auth::handler::configure_auth,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(PerformanceLogger)
            .configure(configure_auth)
            .service(health_check)
            .service(ping)
            .service(web::scope("")
            .wrap(BearerAuth)
            .service(protect_ping)),
    );
}

/// aws health check 용
///
/// 서버 상태 모니터링용
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "서버 정상 동작", body = Base<String>)
    ),
    tag = "health"
)]
#[get("/")]
pub async fn health_check() -> Result<impl Responder, AppError> {
    let body = Base::success("success".to_string());
    Ok(web::Json(body))
}

/// 서버 정상 동작 확인
///
/// 서버가 200 내려주면 살아있는거
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "서버 정상 동작", body = Base<String>)
    ),
    tag = "health"
)]
#[get("/ping")]
pub async fn ping() -> Result<impl Responder, AppError> {
    let body = Base::success("pong".to_string());
    Ok(web::Json(body))
}

/// 헤더 토큰 동작확인
///
/// 아직은 헤더에 bearer 붙이고 아무거나 넣으면 동작
#[utoipa::path(
    get,
    path = "/protect_ping",
    responses(
        (status = 200, description = "헤더 토큰 확인 동작", body = Base<String>)
    ),
    security(("bearerAuth" = []),),
    tag = "health"
)]
#[get("/protect_ping")]
pub async fn protect_ping() -> Result<impl Responder, AppError> {
    let body = Base::success("pong".to_string());
    Ok(web::Json(body))
}

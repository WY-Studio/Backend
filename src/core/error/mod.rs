use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::core::response::Base;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("데이터베이스 에러: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("인증 에러: {0}")]
    AuthError(#[from] jsonwebtoken::errors::Error),

    #[error("잘못된 요청: {0}")]
    BadRequest(String),

    #[error("권한 없음: {0}")]
    Unauthorized(String),

    #[error("서버 에러: {0}")]
    InternalServerError(String),

    #[error("외부 서비스 에러: {0}")]
    ExternalServiceError(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (status, error_message) = match self {
            AppError::AuthError(_) => (StatusCode::UNAUTHORIZED, "인증 에러".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::ExternalServiceError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::DatabaseError(db_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("데이터베이스 에러: {}", db_err),
            ),
        };

        let body = Base::<()> {
            code: status.as_u16(),
            data: None,
            message: error_message,
        };
        HttpResponse::build(status).json(body)
    }
}

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthError(_) => (StatusCode::UNAUTHORIZED, "인증 에러".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::DatabaseError(db_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("데이터베이스 에러: {}", db_err),
            ),
        };

        (
            status,
            Json(json!({
                "error": error_message
            })),
        )
            .into_response()
    }
}

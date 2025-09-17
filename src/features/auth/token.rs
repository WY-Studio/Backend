use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::core::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
    pub email: Option<String>,
    pub typ: String, // "access" | "refresh"
}

pub struct TokenService;

impl TokenService {
    pub fn generate_access_token(
        secret: &str,
        issuer: &str,
        user_id: &str,
        email: Option<&str>,
        ttl_minutes: i64,
    ) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let exp = (Utc::now() + Duration::minutes(ttl_minutes)).timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            iss: issuer.to_string(),
            iat: now,
            exp,
            email: email.map(|e| e.to_string()),
            typ: "access".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("JWT 생성 실패: {}", e)))
    }

    pub fn generate_refresh_token(
        secret: &str,
        issuer: &str,
        user_id: &str,
        ttl_days: i64,
    ) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let exp = (Utc::now() + Duration::days(ttl_days)).timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            iss: issuer.to_string(),
            iat: now,
            exp,
            email: None,
            typ: "refresh".to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("JWT 생성 실패: {}", e)))
    }

    pub fn validate_access_token(
        token: &str,
        secret: &str,
        expected_issuer: &str,
    ) -> Result<Claims, AppError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map_err(|e| AppError::Unauthorized(format!("JWT 검증 실패: {}", e)))?;

        let claims = data.claims;
        if claims.typ != "access" {
            return Err(AppError::Unauthorized(
                "유효하지 않은 토큰 타입".to_string(),
            ));
        }
        if claims.iss != expected_issuer {
            return Err(AppError::Unauthorized("유효하지 않은 issuer".to_string()));
        }
        Ok(claims)
    }
}

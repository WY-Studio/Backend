use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::oauth2::AppleConfig;
use crate::core::error::AppError;
use crate::features::auth::apple::dto::{
    AppleLoginRequest, AppleLoginResponse, AppleTokenResponse, AppleUserInfo,
};
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::{self, Engine};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use reqwest::Client;

pub struct AppleAuthService;

impl AppleAuthService {
    pub async fn login(
        apple_config: &AppleConfig,
        request: AppleLoginRequest,
    ) -> Result<AppleLoginResponse, AppError> {
        // 1. Authorization code로부터 access token 획득
        let token_response = Self::get_apple_access_token(apple_config, &request.code).await?;

        // 2. ID token에서 사용자 정보 추출
        let user_info =
            Self::extract_user_from_id_token(&token_response.id_token, apple_config).await?;

        // 3. 사용자 정보로 로그인/회원가입 처리
        let result = Self::handle_apple_user_login(&user_info).await?;

        return Ok(result);
    }

    async fn get_apple_access_token(
        apple_config: &AppleConfig,
        authorization_code: &str,
    ) -> Result<AppleTokenResponse, AppError> {
        let client = Client::new();

        // Apple client_secret (JWT) 생성
        let client_secret = Self::generate_apple_client_secret(apple_config).await?;

        let params = [
            ("client_id", apple_config.client_id.as_str()),
            ("client_secret", &client_secret),
            ("code", authorization_code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", apple_config.redirect_uri.as_str()),
        ];

        let response = client
            .post(&apple_config.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalServiceError(format!("Apple token request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalServiceError(format!(
                "Apple token request failed: {}",
                error_text
            )));
        }

        let token_response: AppleTokenResponse = response.json().await.map_err(|e| {
            AppError::ExternalServiceError(format!("Failed to parse Apple token response: {}", e))
        })?;

        Ok(token_response)
    }

    async fn generate_apple_client_secret(apple_config: &AppleConfig) -> Result<String, AppError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to get current time: {}", e))
            })?
            .as_secs();

        // JWT Header 생성
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(apple_config.key_id.clone());

        // JWT Payload 생성
        let payload = serde_json::json!({
            "iss": apple_config.team_id,
            "iat": now,
            "exp": now + 3600,
            "aud": "https://appleid.apple.com",
            "sub": apple_config.client_id,
        });

        // Private key를 EncodingKey로 변환
        let encoding_key =
            EncodingKey::from_ec_pem(apple_config.private_key.as_bytes()).map_err(|e| {
                AppError::InternalServerError(format!("Failed to parse private key: {}", e))
            })?;

        // JWT 토큰 생성
        let token = encode(&header, &payload, &encoding_key).map_err(|e| {
            AppError::InternalServerError(format!("Failed to create JWT token: {}", e))
        })?;

        return Ok(token);
    }

    async fn extract_user_from_id_token(
        id_token: &str,
        apple_config: &AppleConfig,
    ) -> Result<AppleUserInfo, AppError> {
        // ID token의 payload 부분만 파싱 (간단하게)
        let token_parts: Vec<&str> = id_token.split('.').collect();
        if token_parts.len() != 3 {
            return Err(AppError::ExternalServiceError(
                "Invalid token format".to_string(),
            ));
        }

        // Payload 부분 디코딩
        let payload_json = BASE64_URL_SAFE_NO_PAD
            .decode(token_parts[1])
            .map_err(|_| AppError::ExternalServiceError("Invalid token payload".to_string()))?;

        let payload: serde_json::Value = serde_json::from_slice(&payload_json).map_err(|_| {
            AppError::ExternalServiceError("Failed to parse token payload".to_string())
        })?;

        // 기본 검증
        if payload["iss"] != "https://appleid.apple.com" {
            return Err(AppError::ExternalServiceError("Invalid issuer".to_string()));
        }

        if payload["aud"] != apple_config.client_id {
            return Err(AppError::ExternalServiceError(
                "Invalid audience".to_string(),
            ));
        }

        // 사용자 정보 추출
        let user_info = AppleUserInfo {
            sub: payload["sub"].as_str().unwrap_or("").to_string(),
            email: payload["email"].as_str().map(|s| s.to_string()),
            email_verified: payload["email_verified"].as_str().map(|s| s.to_string()),
            is_private_email: payload["is_private_email"].as_str().map(|s| s.to_string()),
            name: None,
        };

        Ok(user_info)
    }

    async fn handle_apple_user_login(
        user_info: &AppleUserInfo,
    ) -> Result<AppleLoginResponse, AppError> {
        // TODO: 실제 DB 조회/생성 로직 구현
        // 현재는 임시 응답
        Ok(AppleLoginResponse {
            access_token: "temp_access_token".to_string(),
            refresh_token: "temp_refresh_token".to_string(),
            user_id: user_info.sub.clone(),
            email: user_info.email.clone(),
            name: None,
        })
    }
}

use std::sync::Arc;

use rand::Rng;

use crate::{
    app_state::AppState,
    core::error::AppError,
    features::auth::{
        apple::{dto::AppleLoginRequest, service::AppleAuthService},
        dto::{OAuthQuery, OAuthResponse},
        kakao::{dto::KakaoLoginRequest, service::KakaoAuthService},
    },
};

pub async fn handle_apple_login(
    state: &Arc<AppState>,
    query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    let apple_config = &state.oauth_config.apple;
    let apple_request = AppleLoginRequest {
        code: query.code,
        state: query.state,
    };

    let result = AppleAuthService::login(apple_config, apple_request).await?;

    return Ok(OAuthResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        user_id: result.user_id,
        email: result.email,
        name: result.name,
    });
}

// Google 로그인 처리
pub async fn handle_google_login(
    _state: &Arc<AppState>,
    _query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    // TODO: Google 로그인 구현
    todo!("Google login not implemented yet")
}

// Kakao 로그인 처리
pub async fn handle_kakao_login(
    state: &Arc<AppState>,
    query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    let kakao_config = &state.oauth_config.kakao;
    let kakao_request = KakaoLoginRequest {
        code: query.code,
        state: query.state,
    };

    let result = KakaoAuthService::login(kakao_config, kakao_request).await?;

    Ok(OAuthResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        user_id: result.user_id,
        email: result.email,
        name: result.name,
    })
}

// Naver 로그인 처리
pub async fn handle_naver_login(
    _state: &Arc<AppState>,
    _query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    // TODO: Naver 로그인 구현
    todo!("Naver login not implemented yet")
}

pub fn generate_state() -> String {
    let mut rng = rand::rng();
    let state: u64 = rng.random();
    format!("{:x}", state)
}

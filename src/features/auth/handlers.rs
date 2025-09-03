use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::Redirect,
};
use rand::Rng;

use crate::{
    app_state::AppState,
    core::error::AppError,
    features::auth::{
        apple::{dto::AppleLoginRequest, service::AppleAuthService},
        dto::{OAuthQuery, OAuthResponse},
    },
};

/// OAuth 로그인 URL 생성
///
/// 지원하는 OAuth 제공자: apple, google, kakao, naver
#[utoipa::path(
    get,
    path = "/api/auth/{provider}/login",
    params(
        ("provider" = String, Path, description = "OAuth 제공자 (apple, google, kakao, naver)")
    ),
    responses(
        (status = 302, description = "OAuth 제공자 로그인 페이지로 리다이렉트"),
        (status = 400, description = "지원하지 않는 제공자")
    ),
    tag = "auth"
)]
pub async fn oauth_login(
    State(state): State<Arc<AppState>>,
    Path(provider): Path<String>,
) -> Result<Redirect, AppError> {
    let auth_url = match provider.as_str() {
        "apple" => {
            let apple_config = &state.oauth_config.apple;
            format!(
                "{}?client_id={}&redirect_uri={}&response_type=code&response_mode=form_post&scope=name%20email&state={}",
                apple_config.auth_url,
                apple_config.client_id,
                apple_config.redirect_uri,
                generate_state()
            )
        }
        "google" => {
            let google_config = &state.oauth_config.google;
            format!(
                "{}?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}",
                google_config.auth_url,
                google_config.client_id,
                google_config.redirect_uri,
                generate_state()
            )
        }
        "kakao" => {
            let kakao_config = &state.oauth_config.kakao;
            format!(
                "{}?client_id={}&redirect_uri={}&response_type=code&state={}",
                kakao_config.auth_url,
                kakao_config.client_id,
                kakao_config.redirect_uri,
                generate_state()
            )
        }
        "naver" => {
            let naver_config = &state.oauth_config.naver;
            format!(
                "{}?client_id={}&redirect_uri={}&response_type=code&state={}",
                naver_config.auth_url,
                naver_config.client_id,
                naver_config.redirect_uri,
                generate_state()
            )
        }
        _ => return Err(AppError::BadRequest("Unsupported provider".to_string())),
    };

    Ok(Redirect::to(&auth_url))
}

// Provider별 로그인 콜백
/// OAuth 로그인 콜백 처리
///
/// OAuth 제공자로부터 인증 코드를 받아 사용자 정보를 처리합니다.
#[utoipa::path(
    get,
    path = "/api/auth/{provider}/callback",
    params(
        ("provider" = String, Path, description = "OAuth 제공자 (apple, google, kakao, naver)")
    ),
    request_body = OAuthQuery,
    responses(
        (status = 200, description = "로그인 성공", body = OAuthResponse),
        (status = 400, description = "잘못된 요청"),
        (status = 500, description = "서버 내부 오류")
    ),
    tag = "auth"
)]
pub async fn oauth_login_callback(
    State(state): State<Arc<AppState>>,
    Path(provider): Path<String>,
    Query(query): Query<OAuthQuery>,
) -> Result<Json<OAuthResponse>, AppError> {
    let response = match provider.as_str() {
        "apple" => handle_apple_login(&state, query).await?,
        "google" => handle_google_login(&state, query).await?,
        "kakao" => handle_kakao_login(&state, query).await?,
        "naver" => handle_naver_login(&state, query).await?,
        _ => return Err(AppError::BadRequest("Unsupported provider".to_string())),
    };

    Ok(Json(response))
}

async fn handle_apple_login(
    state: &Arc<AppState>,
    query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    let apple_config = &state.oauth_config.apple;
    let apple_request = AppleLoginRequest {
        code: query.code,
        state: query.state,
    };

    let result = AppleAuthService::login(apple_config, apple_request).await?;

    Ok(OAuthResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        user_id: result.user_id,
        email: result.email,
        name: result.name,
    })
}

// Google 로그인 처리
async fn handle_google_login(
    _state: &Arc<AppState>,
    _query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    // TODO: Google 로그인 구현
    todo!("Google login not implemented yet")
}

// Kakao 로그인 처리
async fn handle_kakao_login(
    _state: &Arc<AppState>,
    _query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    // TODO: Kakao 로그인 구현
    todo!("Kakao login not implemented yet")
}

// Naver 로그인 처리
async fn handle_naver_login(
    _state: &Arc<AppState>,
    _query: OAuthQuery,
) -> Result<OAuthResponse, AppError> {
    // TODO: Naver 로그인 구현
    todo!("Naver login not implemented yet")
}

fn generate_state() -> String {
    let mut rng = rand::rng();
    let state: u64 = rng.random();
    format!("{:x}", state)
}

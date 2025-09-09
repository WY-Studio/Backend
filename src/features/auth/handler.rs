use std::sync::Arc;

use actix_web::http::header;
use actix_web::{HttpResponse, Responder, get, web};

use crate::features::auth::service::*;
use crate::{
    app_state::AppState,
    core::{error::AppError, response::Base},
    features::auth::dto::{OAuthQuery, OAuthResponse},
};

// --- Route registration for auth ---
pub fn configure_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(oauth_login).service(oauth_login_callback);
}

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
#[get("/auth/{provider}/login")]
pub async fn oauth_login(
    state: web::Data<AppState>,
    provider: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let provider = provider.into_inner();
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

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, auth_url))
        .finish())
}

/// OAuth 로그인 콜백 처리
///
/// OAuth 제공자로부터 인증 코드를 받아 사용자 정보를 처리합니다.
#[utoipa::path(
    get,
    path = "/auth/{provider}/callback",
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
#[get("/auth/{provider}/callback")]
pub async fn oauth_login_callback(
    state: web::Data<AppState>,
    provider: web::Path<String>,
    query: web::Query<OAuthQuery>,
) -> Result<impl Responder, AppError> {
    let provider = provider.into_inner();
    let query = query.into_inner();
    let app_state = Arc::new(state.get_ref().clone());
    let response = match provider.as_str() {
        "apple" => handle_apple_login(&app_state, query).await?,
        "google" => handle_google_login(&app_state, query).await?,
        "kakao" => handle_kakao_login(&app_state, query).await?,
        "naver" => handle_naver_login(&app_state, query).await?,
        _ => return Err(AppError::BadRequest("Unsupported provider".to_string())),
    };

    Ok(web::Json(Base::success(response)))
}

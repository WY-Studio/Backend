use std::sync::Arc;

use actix_web::http::header;
use actix_web::{HttpResponse, Responder, get, post, web};
use rand::Rng;
use uuid::Uuid;

use crate::core::features::service::token::TokenService;
use crate::database::entities::prelude::TbUserProvider;
use crate::database::entities::tb_user_provider;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::features::auth::apple::dto::AppleLoginRequest;
use crate::features::auth::apple::service::AppleAuthService;
use crate::features::auth::kakao::dto::KakaoLoginRequest;
use crate::features::auth::kakao::service::KakaoAuthService;

use crate::{
    app_state::AppState,
    core::error::AppError,
    features::auth::dto::{OAuthQuery, OAuthResponse},
};

// --- Route registration for auth ---
pub fn configure_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(oauth_login)
        .service(oauth_login_callback)
        .service(oauth_login_callback_apple);
}

/// OAuth 로그인 URL 생성
///
/// 지원하는 OAuth 제공자: apple, google, kakao, naver
#[utoipa::path(
    get,
    path = "/auth/{provider}/login",
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

    // 1) 프로바이더별 로그인 처리 → provider 측 user_id, email, name 수신
    let response = match provider.as_str() {
        "apple" => handle_apple_login(&app_state, query).await?,
        "google" => handle_google_login(&app_state, query).await?,
        "kakao" => handle_kakao_login(&app_state, query).await?,
        "naver" => handle_naver_login(&app_state, query).await?,
        _ => return Err(AppError::BadRequest("Unsupported provider".to_string())),
    };

    let db = state.db.as_ref();
    let provider_str = provider.clone();
    let provider_user_id = response.user_id.clone();

    let existing = TbUserProvider::find()
        .filter(tb_user_provider::Column::Provider.eq(provider_str.clone()))
        .filter(tb_user_provider::Column::ProviderUserId.eq(provider_user_id.clone()))
        .one(db)
        .await
        .map_err(|e| AppError::InternalServerError(format!("DB 조회 실패: {}", e)))?;

    let app_user_id: Uuid = if let Some(model) = existing {
        model.user_id
    } else {
        let new_user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let active = tb_user_provider::ActiveModel {
            provider_id: sea_orm::ActiveValue::NotSet,
            user_id: Set(new_user_id),
            provider: Set(provider_str.clone()),
            provider_user_id: Set(provider_user_id.clone()),
            email: Set(response.email.clone()),
            display_name: Set(response.name.clone()),
            connected_at: Set(now.into()),
            last_login_at: Set(Some(now.into())),
        };
        active
            .insert(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("DB 생성 실패: {}", e)))?;
        new_user_id
    };

    let jwt = &state.jwt_config;
    let access = TokenService::generate_access_token(
        &jwt.secret,
        &jwt.issuer,
        &app_user_id.to_string(),
        response.email.as_deref(),
        jwt.access_ttl_minutes,
    )?;
    let refresh = TokenService::generate_refresh_token(
        &jwt.secret,
        &jwt.issuer,
        &app_user_id.to_string(),
        jwt.refresh_ttl_days,
    )?;

    let base = "wooyeon://";
    let redirect = format!(
        "{}?accessToken={}&refreshToken={}&userId={}&email={}&name={}",
        base,
        access,
        refresh,
        app_user_id,
        response.email.clone().unwrap_or_default(),
        response.name.clone().unwrap_or_default(),
    );

    return Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, redirect))
        .finish());
}

#[post("/auth/apple/callback")]
pub async fn oauth_login_callback_apple(
    state: web::Data<AppState>,
    form: web::Form<OAuthQuery>,
) -> Result<impl Responder, AppError> {
    let query = form.into_inner();
    let app_state = Arc::new(state.get_ref().clone());

    // 1) 프로바이더별 로그인 처리 → provider 측 user_id, email, name 수신
    let response = handle_apple_login(&app_state, query).await?;

    let db = state.db.as_ref();

    let provider_str = "apple".to_string();

    let provider_user_id = response.user_id.clone();

    let existing = TbUserProvider::find()
        .filter(tb_user_provider::Column::Provider.eq(provider_str.clone()))
        .filter(tb_user_provider::Column::ProviderUserId.eq(provider_user_id.clone()))
        .one(db)
        .await
        .map_err(|e| AppError::InternalServerError(format!("DB 조회 실패: {}", e)))?;

    let app_user_id: Uuid = if let Some(model) = existing {
        model.user_id
    } else {
        let new_user_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let active = tb_user_provider::ActiveModel {
            provider_id: sea_orm::ActiveValue::NotSet,
            user_id: Set(new_user_id),
            provider: Set(provider_str.clone()),
            provider_user_id: Set(provider_user_id.clone()),
            email: Set(response.email.clone()),
            display_name: Set(response.name.clone()),
            connected_at: Set(now.into()),
            last_login_at: Set(Some(now.into())),
        };
        active
            .insert(db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("DB 생성 실패: {}", e)))?;
        new_user_id
    };

    let jwt = &state.jwt_config;
    let access = TokenService::generate_access_token(
        &jwt.secret,
        &jwt.issuer,
        &app_user_id.to_string(),
        response.email.as_deref(),
        jwt.access_ttl_minutes,
    )?;
    let refresh = TokenService::generate_refresh_token(
        &jwt.secret,
        &jwt.issuer,
        &app_user_id.to_string(),
        jwt.refresh_ttl_days,
    )?;

    let base = "wooyeon://";
    let redirect = format!(
        "{}?accessToken={}&refreshToken={}&userId={}&email={}&name={}",
        base,
        access,
        refresh,
        app_user_id,
        response.email.clone().unwrap_or_default(),
        response.name.clone().unwrap_or_default(),
    );

    return Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, redirect))
        .finish());
}

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

    return Ok(OAuthResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        user_id: result.user_id,
        email: result.email,
        name: result.name,
    });
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

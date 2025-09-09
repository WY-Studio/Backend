use crate::config::oauth2::KakaoConfig;
use crate::core::error::AppError;
use crate::features::auth::kakao::dto::{
    KakaoLoginRequest, KakaoLoginResponse, KakaoTokenResponse, KakaoUserInfo,
};
use reqwest::Client;

pub struct KakaoAuthService;

impl KakaoAuthService {
    pub async fn login(
        kakao_config: &KakaoConfig,
        request: KakaoLoginRequest,
    ) -> Result<KakaoLoginResponse, AppError> {
        // 1. Authorization code로부터 access token 획득
        let token_response = Self::get_kakao_access_token(kakao_config, &request.code).await?;

        // 2. Access token으로 사용자 정보 조회
        let user_info = Self::get_kakao_user_info(&token_response.access_token).await?;

        // 3. 사용자 정보로 로그인/회원가입 처리
        let result = Self::handle_kakao_user_login(&user_info, &token_response).await?;

        Ok(result)
    }

    async fn get_kakao_access_token(
        kakao_config: &KakaoConfig,
        authorization_code: &str,
    ) -> Result<KakaoTokenResponse, AppError> {
        let client = Client::new();

        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", kakao_config.client_id.as_str()),
            ("redirect_uri", kakao_config.redirect_uri.as_str()),
            ("code", authorization_code),
        ];

        let response = client
            .post(&kakao_config.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalServiceError(format!("Kakao token request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalServiceError(format!(
                "Kakao token request failed: {}",
                error_text
            )));
        }

        let token_response: KakaoTokenResponse = response.json().await.map_err(|e| {
            AppError::ExternalServiceError(format!("Failed to parse Kakao token response: {}", e))
        })?;

        Ok(token_response)
    }

    async fn get_kakao_user_info(access_token: &str) -> Result<KakaoUserInfo, AppError> {
        let client = Client::new();

        let response = client
            .get("https://kapi.kakao.com/v2/user/me")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalServiceError(format!("Kakao user info request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalServiceError(format!(
                "Kakao user info request failed: {}",
                error_text
            )));
        }

        let user_info: KakaoUserInfo = response.json().await.map_err(|e| {
            AppError::ExternalServiceError(format!("Failed to parse Kakao user info: {}", e))
        })?;

        Ok(user_info)
    }

    async fn handle_kakao_user_login(
        user_info: &KakaoUserInfo,
        token_response: &KakaoTokenResponse,
    ) -> Result<KakaoLoginResponse, AppError> {
        // TODO: 실제 DB 조회/생성 로직 구현
        // 현재는 임시 응답

        // 사용자 이름 추출 (nickname 우선, 없으면 email 사용)
        let name = user_info
            .properties
            .as_ref()
            .and_then(|p| p.nickname.clone())
            .or_else(|| {
                user_info
                    .kakao_account
                    .as_ref()
                    .and_then(|a| a.profile.as_ref())
                    .and_then(|p| p.nickname.clone())
            });

        // 이메일 추출
        let email = user_info
            .kakao_account
            .as_ref()
            .and_then(|a| a.email.clone());

        Ok(KakaoLoginResponse {
            access_token: token_response.access_token.clone(),
            refresh_token: token_response.refresh_token.clone().unwrap_or_default(),
            user_id: user_info.id.to_string(),
            email,
            name,
        })
    }
}

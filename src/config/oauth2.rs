use std::fs;

use crate::config::app_config::{AppConfig, load_config};

#[derive(Debug, Clone)]
pub struct OAuth2Config {
    pub google: GoogleConfig,
    pub naver: NaverConfig,
    pub kakao: KakaoConfig,
    pub apple: AppleConfig,
}

#[derive(Debug, Clone)]
pub struct GoogleConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
}

#[derive(Debug, Clone)]
pub struct NaverConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
}

#[derive(Debug, Clone)]
pub struct KakaoConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
}

#[derive(Debug, Clone)]
pub struct AppleConfig {
    pub client_id: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub team_id: String,
    pub key_id: String,
    pub private_key: String,
}

impl OAuth2Config {
    pub fn new() -> Self {
        let file_cfg: Option<AppConfig> = load_config().ok();

        let google_client_id = file_cfg
            .as_ref()
            .map(|c| c.oauth.google.client_id.clone())
            .expect("GOOGLE_CLIENT_ID 설정이 필요합니다.");
        let google_client_secret = file_cfg
            .as_ref()
            .map(|c| c.oauth.google.client_secret.clone())
            .expect("GOOGLE_CLIENT_SECRET 설정이 필요합니다.");
        let google_redirect_uri = file_cfg
            .as_ref()
            .map(|c| c.oauth.google.redirect_uri.clone())
            .unwrap_or_else(|| "http://localhost:3000/auth/google/callback".to_string());

        let naver_client_id = file_cfg
            .as_ref()
            .map(|c| c.oauth.naver.client_id.clone())
            .expect("NAVER_CLIENT_ID 설정이 필요합니다.");
        let naver_client_secret = file_cfg
            .as_ref()
            .map(|c| c.oauth.naver.client_secret.clone())
            .expect("NAVER_CLIENT_SECRET 설정이 필요합니다.");
        let naver_redirect_uri = file_cfg
            .as_ref()
            .map(|c| c.oauth.naver.redirect_uri.clone())
            .unwrap_or_else(|| "http://localhost:3000/auth/naver/callback".to_string());

        let kakao_client_id = file_cfg
            .as_ref()
            .map(|c| c.oauth.kakao.client_id.clone())
            .expect("KAKAO_CLIENT_ID 설정이 필요합니다.");
        let kakao_client_secret = file_cfg
            .as_ref()
            .map(|c| c.oauth.kakao.client_secret.clone())
            .expect("KAKAO_CLIENT_SECRET 설정이 필요합니다.");
        let kakao_redirect_uri = file_cfg
            .as_ref()
            .map(|c| c.oauth.kakao.redirect_uri.clone())
            .unwrap_or_else(|| "http://localhost:3000/auth/kakao/callback".to_string());

        let apple_client_id = file_cfg
            .as_ref()
            .map(|c| c.oauth.apple.client_id.clone())
            .expect("APPLE_CLIENT_ID 설정이 필요합니다.");
        let apple_redirect_uri = file_cfg
            .as_ref()
            .map(|c| c.oauth.apple.redirect_uri.clone())
            .unwrap_or_else(|| "http://localhost:3000/auth/apple/callback".to_string());
        let apple_team_id = file_cfg
            .as_ref()
            .map(|c| c.oauth.apple.team_id.clone())
            .expect("APPLE_TEAM_ID 설정이 필요합니다.");
        let apple_key_id = file_cfg
            .as_ref()
            .map(|c| c.oauth.apple.key_id.clone())
            .expect("APPLE_KEY_ID 설정이 필요합니다.");
        let apple_private_key = file_cfg
            .as_ref()
            .map(|c| c.oauth.apple.private_key.clone())
            .unwrap_or_else(|| Self::read_private_key_from_file());

        Self {
            google: GoogleConfig {
                client_id: google_client_id,
                client_secret: google_client_secret,
                redirect_uri: google_redirect_uri,
                auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                token_url: "https://oauth2.googleapis.com/token".to_string(),
                user_info_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
            },
            naver: NaverConfig {
                client_id: naver_client_id,
                client_secret: naver_client_secret,
                redirect_uri: naver_redirect_uri,
                auth_url: "https://nid.naver.com/oauth2.0/authorize".to_string(),
                token_url: "https://nid.naver.com/oauth2.0/token".to_string(),
                user_info_url: "https://openapi.naver.com/v1/nid/me".to_string(),
            },
            kakao: KakaoConfig {
                client_id: kakao_client_id,
                client_secret: kakao_client_secret,
                redirect_uri: kakao_redirect_uri,
                auth_url: "https://kauth.kakao.com/oauth/authorize".to_string(),
                token_url: "https://kauth.kakao.com/oauth/token".to_string(),
                user_info_url: "https://kapi.kakao.com/v2/user/me".to_string(),
            },
            apple: AppleConfig {
                client_id: apple_client_id,
                redirect_uri: apple_redirect_uri,
                auth_url: "https://appleid.apple.com/auth/authorize".to_string(),
                token_url: "https://appleid.apple.com/auth/token".to_string(),
                user_info_url: "https://appleid.apple.com/auth/userinfo".to_string(),
                team_id: apple_team_id,
                key_id: apple_key_id,
                private_key: apple_private_key,
            },
        }
    }

    fn read_private_key_from_file() -> String {
        let key_path = "resources/Apple Auth Key.p8";

        match fs::read_to_string(key_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read private key file: {}", e);
                panic!("Private key file not found or unreadable");
            }
        }
    }
}

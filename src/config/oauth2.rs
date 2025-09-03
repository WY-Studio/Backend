use std::{env, fs};

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
        dotenvy::dotenv().ok();

        Self {
            google: GoogleConfig {
                client_id: env::var("GOOGLE_CLIENT_ID")
                    .expect("GOOGLE_CLIENT_ID 환경변수가 설정되어 있지 않습니다."),
                client_secret: env::var("GOOGLE_CLIENT_SECRET")
                    .expect("GOOGLE_CLIENT_SECRET 환경변수가 설정되어 있지 않습니다."),
                redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                    .unwrap_or_else(|_| "http://localhost:3000/auth/google/callback".to_string()),
                auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                token_url: "https://oauth2.googleapis.com/token".to_string(),
                user_info_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
            },
            naver: NaverConfig {
                client_id: env::var("NAVER_CLIENT_ID")
                    .expect("NAVER_CLIENT_ID 환경변수가 설정되어 있지 않습니다."),
                client_secret: env::var("NAVER_CLIENT_SECRET")
                    .expect("NAVER_CLIENT_SECRET 환경변수가 설정되어 있지 않습니다."),
                redirect_uri: env::var("NAVER_REDIRECT_URI")
                    .unwrap_or_else(|_| "http://localhost:3000/auth/naver/callback".to_string()),
                auth_url: "https://nid.naver.com/oauth2.0/authorize".to_string(),
                token_url: "https://nid.naver.com/oauth2.0/token".to_string(),
                user_info_url: "https://openapi.naver.com/v1/nid/me".to_string(),
            },
            kakao: KakaoConfig {
                client_id: env::var("KAKAO_CLIENT_ID")
                    .expect("KAKAO_CLIENT_ID 환경변수가 설정되어 있지 않습니다."),
                client_secret: env::var("KAKAO_CLIENT_SECRET")
                    .expect("KAKAO_CLIENT_SECRET 환경변수가 설정되어 있지 않습니다."),
                redirect_uri: env::var("KAKAO_REDIRECT_URI")
                    .unwrap_or_else(|_| "http://localhost:3000/auth/kakao/callback".to_string()),
                auth_url: "https://kauth.kakao.com/oauth/authorize".to_string(),
                token_url: "https://kauth.kakao.com/oauth/token".to_string(),
                user_info_url: "https://kapi.kakao.com/v2/user/me".to_string(),
            },
            apple: AppleConfig {
                client_id: env::var("APPLE_CLIENT_ID")
                    .expect("APPLE_CLIENT_ID 환경변수가 설정되어 있지 않습니다."),

                redirect_uri: env::var("APPLE_REDIRECT_URI")
                    .unwrap_or_else(|_| "http://localhost:3000/auth/apple/callback".to_string()),
                auth_url: "https://appleid.apple.com/auth/authorize".to_string(),
                token_url: "https://appleid.apple.com/auth/token".to_string(),
                user_info_url: "https://appleid.apple.com/auth/userinfo".to_string(),
                team_id: env::var("APPLE_TEAM_ID")
                    .expect("APPLE_TEAM_ID 환경변수가 설정되어 있지 않습니다."),
                key_id: env::var("APPLE_KEY_ID")
                    .expect("APPLE_KEY_ID 환경변수가 설정되어 있지 않습니다."),
                private_key: Self::read_private_key_from_file(),
            },
        }
    }

    fn read_private_key_from_file() -> String {
        let key_path = "resources/AuthKey_F4NFBUH3G7.p8";

        match fs::read_to_string(key_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read private key file: {}", e);
                panic!("Private key file not found or unreadable");
            }
        }
    }
}

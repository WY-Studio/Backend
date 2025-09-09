use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KakaoLoginRequest {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KakaoTokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_in: Option<i32>,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KakaoUserInfo {
    pub id: i64, // Kakao user ID
    pub connected_at: Option<String>,
    pub properties: Option<KakaoUserProperties>,
    pub kakao_account: Option<KakaoAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KakaoUserProperties {
    pub nickname: Option<String>,
    pub profile_image: Option<String>,
    pub thumbnail_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KakaoAccount {
    pub profile_nickname_needs_agreement: Option<bool>,
    pub profile_image_needs_agreement: Option<bool>,
    pub profile: Option<KakaoProfile>,
    pub has_email: Option<bool>,
    pub email_needs_agreement: Option<bool>,
    pub is_email_valid: Option<bool>,
    pub is_email_verified: Option<bool>,
    pub email: Option<String>,
    pub has_age_range: Option<bool>,
    pub age_range_needs_agreement: Option<bool>,
    pub age_range: Option<String>,
    pub has_birthday: Option<bool>,
    pub birthday_needs_agreement: Option<bool>,
    pub birthday: Option<String>,
    pub birthday_type: Option<String>,
    pub has_gender: Option<bool>,
    pub gender_needs_agreement: Option<bool>,
    pub gender: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KakaoProfile {
    pub nickname: Option<String>,
    pub thumbnail_image_url: Option<String>,
    pub profile_image_url: Option<String>,
    pub is_default_image: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KakaoLoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct KakaoAuthQuery {
    pub code: String,
    pub state: Option<String>,
}

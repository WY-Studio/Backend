use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleLoginRequest {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleUserInfo {
    pub sub: String, // Apple user ID
    pub email: Option<String>,
    pub email_verified: Option<String>,
    pub is_private_email: Option<String>,
    pub name: Option<AppleUserName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleUserName {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleLoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct AppleAuthQuery {
    pub code: String,
    pub state: Option<String>,
}
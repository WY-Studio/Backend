use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct OAuthQuery {
    #[schema(example = "auth_code_123")]
    pub code: String,
    #[schema(example = "state_value")]
    pub state: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    #[schema(example = "refresh_token_123")]
    pub refresh_token: String,
    #[schema(example = "user_123")]
    pub user_id: String,
    #[schema(example = "user@example.com")]
    pub email: Option<String>,
    #[schema(example = "홍길동")]
    pub name: Option<String>,
}

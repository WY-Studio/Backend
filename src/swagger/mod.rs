use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::features::{auth::dto::{OAuthQuery, OAuthResponse}, user::dto::{SearchKeywords, User}};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::auth::handler::oauth_login,
        crate::features::auth::handler::oauth_login_callback,
        crate::routes::health_check,
        crate::routes::ping,
        crate::routes::protect_ping,
        crate::features::user::handler::get_me,
        crate::features::user::handler::read_user,
        crate::features::user::handler::create_user,
        crate::features::user::handler::get_user_by_phone,        
        crate::features::user::handler::search_users,
    ),
    components(
        schemas(OAuthQuery, OAuthResponse, User, SearchKeywords),         
    ),
    
    tags(
        (name = "auth", description = "인증 관련 API"),
        (name = "health", description = "헬스 체크 API"),
        (name = "user", description = "사용자 관련 API")
    ),
    info(
        title = "WY Backend API",
        version = "1.0.0",
        description = "WY Backend 서버의 API 문서",
        contact(
            name = "WY Team",
            email = "contact@wy.com"
        )
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

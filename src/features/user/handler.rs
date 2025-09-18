use actix_web::{HttpRequest, Responder, get, post, web};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    core::{
        error::AppError,
        response::{Base, Page, Tokens},
    },
    features::user::{
        dto::{SearchKeywords, User},
        service::UserService,
    },
};

pub fn configure_user(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user)
        .service(get_user_by_phone)
        .service(get_me)
        .service(read_user)
        .service(search_users);
}

/// 내 정보 조회
///
/// 토큰에서 유저 아이디 추출 후 유저 정보 조회

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "내 정보 조회", body = Base<User>),
        (status = 400, description = "존재하지 않는 사용자"),
        (status = 401, description = "인증 에러"),
        (status = 500, description = "서버 에러"),
        (status = 901, description = "활성화 안된 사용자")
    ),
    security(("bearerAuth" = [])),
    tag = "user"
)]
#[get("/user")]
pub async fn get_me(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let user_id = UserService::extract_user_id_from_request(&req)?;
    let user = UserService::get_user_info(state.db.as_ref(), user_id).await?;
    return Ok(web::Json(Base::success(user)));
}

/// 유저 상세 조회
///
/// 유저 상세 조회 사용예정

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    responses(
        (status = 200, description = "유저 정보", body = Base<User>),
    ),
    tag = "user"
)]
#[get("/user/{user_id}")]
pub async fn read_user(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let user = UserService::get_user_info(state.db.as_ref(), user_id.into_inner()).await?;
    return Ok(web::Json(Base::success(user)));
}

/// 유저 생성
///
/// 바디에 User 객체로 생성

#[utoipa::path(
    post,
    path = "/user",
    request_body = User,
    responses(
        (status = 200, description = "유저 생성 완료", body = Base<bool>),
        (status = 400, description = "존재하지 않는 사용자"),
        (status = 401, description = "인증 에러"),
        (status = 500, description = "서버 에러"),

    ),
    tag = "user"
)]
#[post("/user")]
pub async fn create_user(
    state: web::Data<AppState>,
    body: web::Json<User>,
) -> Result<impl Responder, AppError> {
    UserService::insert_user(state.db.as_ref(), body.into_inner()).await?;
    return Ok(web::Json(Base::success(true)));
}

/// 핸드폰 번호로 유저 검색
///
/// 바디에 p_num 으로 검색

#[utoipa::path(
    post,
    path = "/user/p_num_verify",
    request_body = SearchKeywords,
    responses(
        (status = 200, description = "존재하는 사용자 정보", body = Base<Tokens>),
        (status = 400, description = "존재하지 않는 사용자"),
        (status = 401, description = "인증 에러"),
        (status = 500, description = "서버 에러"),
    ),
    tag = "user"
)]
#[post("/user/p_num_verify")]
pub async fn get_user_by_phone(
    state: web::Data<AppState>,
    body: web::Json<SearchKeywords>,
) -> Result<impl Responder, AppError> {
    let tokens = UserService::get_user_by_phone(
        state.db.as_ref(),
        &body.p_num.as_ref().unwrap(),
        &state.jwt_config,
    )
    .await?;
    return Ok(web::Json(Base::success(tokens)));
}

/// 유저 검색
///
/// 현재 검색가능 성별, mbti

#[utoipa::path(
    post,
    path = "/user/search",
    request_body = SearchKeywords,
    responses(
        (status = 200, description = "존재하는 사용자 정보", body = Page<User>),
        (status = 400, description = "존재하지 않는 사용자"),
        (status = 401, description = "인증 에러"),
        (status = 500, description = "서버 에러"),
    ),
    tag = "user"
)]
#[post("/user/search")]
pub async fn search_users(
    state: web::Data<AppState>,
    body: web::Json<SearchKeywords>,
) -> Result<impl Responder, AppError> {
    let result = UserService::search_users(state.db.as_ref(), body.into_inner()).await?;
    return Ok(web::Json(result));
}

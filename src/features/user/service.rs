use actix_web::{HttpMessage, HttpRequest};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::config::app_config::JwtConfig;
use crate::core::features::service::token::{Claims, TokenService};
use crate::core::response::{Page, Tokens};
use crate::database::entities::prelude::TbUser;

use crate::database::entities::tb_user;

use crate::features::user::dto::SearchKeywords;
use crate::{core::error::AppError, features::user::dto::User};

pub struct UserService {}

impl UserService {
    pub fn extract_user_id_from_request(req: &HttpRequest) -> Result<Uuid, AppError> {
        let user_id_str = req
            .extensions()
            .get::<Claims>()
            .map(|c| c.sub.clone())
            .ok_or_else(|| AppError::Unauthorized("토큰이 없습니다".to_string()))?;

        return Uuid::parse_str(&user_id_str)
            .map_err(|_| AppError::Unauthorized("유효하지 않은 사용자 ID".to_string()));
    }

    pub async fn get_user_info(db: &DatabaseConnection, user_id: Uuid) -> Result<User, AppError> {
        let user = TbUser::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::BadRequest("존재하지 않는 사용자".to_string()))?;

        if user.status == 1 {
            let dto = User {
                user_id: user.user_id,
                p_num: user.p_num,
                n_name: user.n_name,
                gender: user.gender,
                birth_date: user.birth_date,
                created_at: user.created_at,
                status: user.status,
                job: user.job,
                city: user.city,
                district: user.district,
                height_cm: user.height_cm,
                body_type: user.body_type,
                smoking: user.smoking,
                drinking: user.drinking,
                religion: user.religion,
                mbti: user.mbti,
                preferred_age_group: user.preferred_age_group,
                personalities: user.personalities,
                hobbies: user.hobbies,
                introduction: user.introduction,
                appeal_topics: user.appeal_topics,
            };

            return Ok(dto);
        }

        Err(AppError::InactiveUserError(
            "활성화 되지 않은 사용자입니다.".to_string(),
        ))
    }

    pub async fn insert_user(db: &DatabaseConnection, user: User) -> Result<(), AppError> {
        let user = tb_user::ActiveModel {
            user_id: Set(user.user_id),
            p_num: Set(user.p_num),
            n_name: Set(user.n_name),
            gender: Set(user.gender),
            birth_date: Set(user.birth_date),
            created_at: Set(user.created_at),
            status: Set(user.status),
            job: Set(user.job),
            city: Set(user.city),
            district: Set(user.district),
            height_cm: Set(user.height_cm),
            body_type: Set(user.body_type),
            smoking: Set(user.smoking),
            drinking: Set(user.drinking),
            religion: Set(user.religion),
            mbti: Set(user.mbti),
            preferred_age_group: Set(user.preferred_age_group),
            personalities: Set(user.personalities),
            hobbies: Set(user.hobbies),
            introduction: Set(user.introduction),
            appeal_topics: Set(user.appeal_topics),
        };
        user.insert(db).await?;
        return Ok(());
    }

    pub async fn get_user_by_phone(
        db: &DatabaseConnection,
        p_num: &str,
        jwt: &JwtConfig,
    ) -> Result<Tokens, AppError> {
        let user = TbUser::find()
            .filter(tb_user::Column::PNum.eq(p_num))
            .one(db)
            .await?
            .ok_or_else(|| AppError::BadRequest("존재하지 않는 사용자".to_string()))?;

        let access_token = TokenService::generate_access_token(
            &jwt.secret,
            &jwt.issuer,
            &user.user_id.to_string(),
            None,
            jwt.access_ttl_minutes,
        )?;

        let refresh_token = TokenService::generate_refresh_token(
            &jwt.secret,
            &jwt.issuer,
            &user.user_id.to_string(),
            jwt.refresh_ttl_days,
        )?;

        return Ok(Tokens {
            user_id: user.user_id.to_string(),
            access_token: access_token,
            refresh_token: refresh_token,
        });
    }

    pub async fn search_users(
        db: &DatabaseConnection,
        body: SearchKeywords,
    ) -> Result<Page<User>, AppError> {
        let page = body.page.unwrap_or(1).max(1);
        let size = body.size.unwrap_or(20).max(1);

        let cond = Condition::all()
            .add_option(body.gender.map(|v| tb_user::Column::Gender.eq(v)))
            .add_option(
                body.mbti
                    .as_ref()
                    .and_then(|v| (!v.is_empty()).then(|| tb_user::Column::Mbti.is_in(v.clone()))),
            );

        let base = TbUser::find()
            .filter(cond)
            .order_by_desc(tb_user::Column::CreatedAt);

        let total_u64 = base.clone().count(db).await?;

        let models = base
            .limit(size as u64)
            .offset(((page - 1) as u64) * (size as u64))
            .all(db)
            .await?;

        let items: Vec<User> = models
            .into_iter()
            .map(|user| User {
                user_id: user.user_id,
                p_num: user.p_num,
                n_name: user.n_name,
                gender: user.gender,
                birth_date: user.birth_date,
                created_at: user.created_at,
                status: user.status,
                job: user.job,
                city: user.city,
                district: user.district,
                height_cm: user.height_cm,
                body_type: user.body_type,
                smoking: user.smoking,
                drinking: user.drinking,
                religion: user.religion,
                mbti: user.mbti,
                preferred_age_group: user.preferred_age_group,
                personalities: user.personalities,
                hobbies: user.hobbies,
                introduction: user.introduction,
                appeal_topics: user.appeal_topics,
            })
            .collect();

        return Ok(Page::success(items, page, size, total_u64 as i64));
    }
}

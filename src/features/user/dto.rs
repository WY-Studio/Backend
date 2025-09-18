use sea_orm::prelude::{Date, DateTimeWithTimeZone};
use serde::{Deserialize, Serialize};

use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[schema(value_type = String, example = "6ee421ec-684e-4f04-b7e1-38a384645934")]
    pub user_id: Uuid,
    #[schema(value_type = String, example = "01012345678")]
    pub p_num: String,
    #[schema(value_type = String, example = "홍길동")]
    pub n_name: String,
    #[schema(value_type = i16, example = 0)]
    pub gender: i16,
    #[schema(value_type = String, example = "1995-01-01")]
    pub birth_date: Date,
    #[schema(value_type = String, example = "2025-01-01T12:34:56Z")]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = i16, example = 0)]
    pub status: i16,
    #[schema(value_type = String, example = "개발자")]
    pub job: String,

    pub city: String,
    #[schema(value_type = String, example = "서울특별시")]
    pub district: String,
    pub height_cm: i32,
    pub body_type: String,
    pub smoking: String,
    pub drinking: String,
    pub religion: String,
    pub mbti: Option<String>,
    pub preferred_age_group: Option<String>,
    #[schema(value_type = Object)]
    pub personalities: Option<Value>,
    #[schema(value_type = Object)]
    pub hobbies: Option<Value>,
    pub introduction: Option<String>,
    #[schema(value_type = Object)]
    pub appeal_topics: Option<Value>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchKeywords {
    #[schema(value_type = String, example = "01012345678")]
    pub p_num: Option<String>,
    pub n_name: Option<String>,
    pub gender: Option<i16>,
    pub drinking: Option<String>,

    pub age_min: Option<i16>,
    pub age_max: Option<i16>,
    pub city: Option<String>,
    pub district: Option<String>,

    pub height_min: Option<i32>,
    pub height_max: Option<i32>,

    pub mbti: Option<Vec<String>>,

    pub personalities_tags: Option<Vec<String>>,
    pub hobbies_tags: Option<Vec<String>>,
    pub appeal_topics_tags: Option<Vec<String>>,

    pub keyword: Option<String>,

    pub page: Option<i32>,
    pub size: Option<i32>,
}

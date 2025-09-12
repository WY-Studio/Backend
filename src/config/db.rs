use std::env;

use dotenvy::{dotenv, from_filename};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::core::error::AppError;

pub async fn get_db_connection() -> Result<DatabaseConnection, AppError> {
    // ENV_FILE이 설정되면 해당 파일을, 아니면 APP_ENV에 따라 기본 파일을 로드
    // 기본: .env, stage: .env-stage
    if let Ok(env_file) = env::var("ENV_FILE") {
        let _ = from_filename(env_file);
    } else if let Ok(app_env) = env::var("APP_ENV") {
        if app_env.eq_ignore_ascii_case("stage") {
            let _ = from_filename(".env-stage");
        } else {
            dotenv().ok();
        }
    } else {
        dotenv().ok();
    }
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL 환경변수가 설정되어 있지 않습니다.");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(20)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(8))
        .acquire_timeout(std::time::Duration::from_secs(8))
        .idle_timeout(std::time::Duration::from_secs(8))
        .max_lifetime(std::time::Duration::from_secs(8))
        .sqlx_logging(true);

    let conn = Database::connect(opt).await?;
    return Ok(conn);
}

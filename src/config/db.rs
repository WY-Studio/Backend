use std::env;

use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::core::error::AppError;

pub async fn get_db_connection() -> Result<DatabaseConnection, AppError> {
    dotenv().ok();
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

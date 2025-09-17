use crate::config::app_config::load_config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::core::error::AppError;

pub async fn get_db_connection() -> Result<DatabaseConnection, AppError> {
    // YAML 필수
    let database_url = load_config()
        .map(|cfg| cfg.database.url)
        .expect("config.yaml 로드 실패 또는 database.url 누락");

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

use actix_web::{App, HttpServer};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use wy_backend::{
    app_state::AppState,
    config::{cors::build_cors, db::get_db_connection},
    routes,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = get_db_connection().await.expect("DB 연결 실패");
    let app_state = AppState::new(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::info!(
        r#"
        ██████╗ ██╗   ██╗███████╗████████╗
        ██╔══██╗██║   ██║██╔════╝╚══██╔══╝
        ██████╔╝██║   ██║███████╗   ██║   
        ██╔══██╗██║   ██║╚════██║   ██║   
        ██║  ██║╚██████╔╝███████║   ██║   
        ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   

        🚀 Rust Backend Server Starting...
        ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        ✅ 서버가 시작되었습니다: {}
        접속 URL: http://{}
        스웨거 URL: http://localhost:3000/swagger-ui/
        "#,
        addr,
        addr
    );
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(app_state.clone()))
            .wrap(build_cors())
            .service(wy_backend::swagger::swagger_ui())
            .configure(routes::configure)
    })
    .bind(addr)
    .unwrap()
    .run()
    .await
    .unwrap();
}

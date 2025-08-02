use WY_backend::{app_state::AppState, db::get_db_connection, endpoint::create_router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    let app = create_router(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

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
        "#,
        addr,
        addr
    );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

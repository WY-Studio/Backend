use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{info, warn};

pub async fn performance_logger(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    // 성능 로깅
    if duration.as_millis() > 100 {
        warn!(
            "느린 요청 감지: {} {} - {}ms (상태: {})",
            method,
            uri,
            duration.as_millis(),
            status
        );
    } else {
        info!(
            "요청 처리: {} {} - {}ms (상태: {})",
            method,
            uri,
            duration.as_millis(),
            status
        );
    }

    response
}

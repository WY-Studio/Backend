use actix_cors::Cors;
use actix_web::http::header;

pub fn build_cors() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| {
            // 필요시 화이트리스트 로직으로 교체
            origin.as_bytes().starts_with(b"http://localhost")
                || origin.as_bytes().starts_with(b"https://localhost")
        })
        .allowed_methods(["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
        .allowed_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .expose_headers([header::CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}

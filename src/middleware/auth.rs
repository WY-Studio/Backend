use std::future::{Ready, ready};
use std::rc::Rc;

use crate::core::features::service::token::{Claims, TokenService};
use crate::core::response::Base;
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage, HttpResponse};
use futures_util::future::LocalBoxFuture;

// 추가: 토큰 Claims, AppState, 검증에 필요한 크레이트
use crate::app_state::AppState;

pub struct BearerAuth;

impl<S, B: 'static> Transform<S, ServiceRequest> for BearerAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = BearerAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(BearerAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct BearerAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B: 'static> Service<ServiceRequest> for BearerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        if !auth_header.starts_with("Bearer ") || auth_header.len() <= 7 {
            let (req, _pl) = req.into_parts();
            let body = Base::<()> {
                code: 401,
                data: None,
                message: "토큰 없다잉".to_string(),
            };
            let res = HttpResponse::Unauthorized()
                .json(body)
                .map_into_right_body();
            return Box::pin(async move { Ok(ServiceResponse::new(req, res)) });
        }

        let token = &auth_header[7..];

        // AppState에서 시크릿/issuer 가져오기
        let state = match req.app_data::<actix_web::web::Data<AppState>>() {
            Some(s) => s,
            None => {
                let (req, _pl) = req.into_parts();
                let body = Base::<()> {
                    code: 500,
                    data: None,
                    message: "서버 설정 오류".to_string(),
                };
                let res = HttpResponse::InternalServerError()
                    .json(body)
                    .map_into_right_body();
                return Box::pin(async move { Ok(ServiceResponse::new(req, res)) });
            }
        };

        // 토큰 검증
        let jwt = &state.jwt_config;
        let claims_res: Result<Claims, _> =
            TokenService::validate_access_token(token, &jwt.secret, &jwt.issuer);

        if let Err(e) = claims_res {
            let (req, _pl) = req.into_parts();
            let body = Base::<()> {
                code: 401,
                data: None,
                message: format!("인증 실패: {}", e),
            };
            let res = HttpResponse::Unauthorized()
                .json(body)
                .map_into_right_body();
            return Box::pin(async move { Ok(ServiceResponse::new(req, res)) });
        }

        let claims = claims_res.unwrap();
        // 이후 핸들러에서 사용할 수 있도록 Claims 저장
        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?.map_into_left_body();
            Ok(res)
        })
    }
}

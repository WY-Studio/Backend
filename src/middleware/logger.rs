use std::future::{Ready, ready};
use std::rc::Rc;
use std::time::Instant;

use actix_web::Error;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::LocalBoxFuture;
use tracing::{info, warn};

pub struct PerformanceLogger;

impl<S, B> Transform<S, ServiceRequest> for PerformanceLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PerformanceLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PerformanceLoggerMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct PerformanceLoggerMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for PerformanceLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed();
            let status = res.status();

            if duration.as_millis() > 100 {
                warn!(
                    "느린 요청 감지: {} {} - {}ms (상태: {})",
                    method,
                    path,
                    duration.as_millis(),
                    status
                );
            } else {
                info!(
                    "요청 처리: {} {} - {}ms (상태: {})",
                    method,
                    path,
                    duration.as_millis(),
                    status
                );
            }

            Ok(res)
        })
    }
}

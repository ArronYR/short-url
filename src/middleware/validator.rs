use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

const HEADER_KEY: &str = "Api-Secret";

pub struct ApiValidateMiddleware {
    pub secret: String,
}

impl<S, B> Transform<S, ServiceRequest> for ApiValidateMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiValidateMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiValidateMiddlewareService {
            service,
            secret: self.secret.clone(),
        }))
    }
}

pub struct ApiValidateMiddlewareService<S> {
    service: S,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for ApiValidateMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let uri = req.uri().clone();
        let headers = req.headers().clone();

        let path = uri.path();
        let token = headers.get(HEADER_KEY).and_then(|val| val.to_str().ok());
        let fut = self.service.call(req);

        // 排除非 /api/ 开头的地址
        if !path.starts_with("/api/") {
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        match token {
            Some(t) if t == self.secret.as_str() => Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            }),
            _ => Box::pin(async move { Err(error::ErrorUnauthorized("Unauthorized")) }),
        }
    }
}

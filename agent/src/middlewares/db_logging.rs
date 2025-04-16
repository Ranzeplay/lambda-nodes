use crate::db::{create_log, LogLevel};
use actix_web::dev::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::ops::Deref;
use std::sync::Arc;
use log::error;
use tokio_postgres::Client;

pub struct DbLogger {
    pub client: Arc<Client>,
}

impl<S, B> Transform<S, ServiceRequest> for DbLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = DbLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(DbLoggerMiddleware {
            service,
            client: self.client.clone(),
        })
    }
}

pub struct DbLoggerMiddleware<S> {
    service: S,
    client: Arc<Client>,
}

impl<S, B> Service<ServiceRequest> for DbLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
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
        let client = self.client.clone();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let remote_ip = req.connection_info().host().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            let level = if res.response().status().is_success() {
                LogLevel::Info
            } else {
                LogLevel::Warn
            };

            let log_result = create_log(
                client.deref(),
                level,
                "Request",
                &format!("{} {} - {} {}", method, path, res.response().status(), remote_ip),
            )
            .await;
            
            if let Err(e) = log_result {
                error!("Failed to log request into database: {}", e);
            }

            Ok(res)
        })
    }
}

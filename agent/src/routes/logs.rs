use actix_web::{web, HttpResponse, Responder, get, post, delete};
use crate::db::LogLevel;
use serde::Deserialize;
use tokio_postgres::Client;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateLogRequest {
    level: LogLevel,
    message: String,
}

#[derive(Deserialize)]
pub struct ListLogsQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[post("")]
pub async fn create_log(
    client: web::Data<Arc<Client>>,
    req: web::Json<CreateLogRequest>,
) -> impl Responder {
    match crate::db::create_log(&client, req.level.clone(), &req.message).await {
        Ok(log) => HttpResponse::Created().json(log),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_log(
    client: web::Data<Arc<Client>>,
    id: web::Path<i32>,
) -> impl Responder {
    match crate::db::get_log(&client, id.into_inner()).await {
        Ok(Some(log)) => HttpResponse::Ok().json(log),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("")]
pub async fn list_logs(
    client: web::Data<Arc<Client>>,
    query: web::Query<ListLogsQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    match crate::db::list_logs(&client, limit, offset).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_log(
    client: web::Data<Arc<Client>>,
    id: web::Path<i32>,
) -> impl Responder {
    match crate::db::delete_log(&client, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/count")]
pub async fn count_logs(
    client: web::Data<Arc<Client>>,
) -> impl Responder {
    match crate::db::count_logs(&client).await {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/logs")
            .service(count_logs)
            .service(create_log)
            .service(list_logs)
            .service(get_log)
            .service(delete_log)
    );
} 
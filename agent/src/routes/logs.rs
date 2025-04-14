use actix_web::{web, HttpResponse, Responder};
use crate::db::LogLevel;
use serde::Deserialize;
use tokio_postgres::Client;

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

pub async fn create_log(
    client: web::Data<Client>,
    req: web::Json<CreateLogRequest>,
) -> impl Responder {
    match crate::db::create_log(&client, req.level.clone(), &req.message).await {
        Ok(log) => HttpResponse::Created().json(log),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_log(
    client: web::Data<Client>,
    id: web::Path<i32>,
) -> impl Responder {
    match crate::db::get_log(&client, id.into_inner()).await {
        Ok(Some(log)) => HttpResponse::Ok().json(log),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn list_logs(
    client: web::Data<Client>,
    query: web::Query<ListLogsQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    match crate::db::list_logs(&client, limit, offset).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_log(
    client: web::Data<Client>,
    id: web::Path<i32>,
) -> impl Responder {
    match crate::db::delete_log(&client, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/logs")
            .route("", web::post().to(create_log))
            .route("/{id}", web::get().to(get_log))
            .route("", web::get().to(list_logs))
            .route("/{id}", web::delete().to(delete_log))
    );
} 
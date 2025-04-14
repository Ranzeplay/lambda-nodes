use actix_web::{web, HttpResponse, Responder};
use crate::db::HttpMethod;
use serde::Deserialize;
use tokio_postgres::Client;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePipelineRequest {
    name: String,
    content: serde_json::Value,
    method: HttpMethod,
    url: String,
}

#[derive(Deserialize)]
pub struct UpdatePipelineRequest {
    name: String,
    content: serde_json::Value,
    method: HttpMethod,
    url: String,
}

pub async fn create_pipeline(
    client: web::Data<Client>,
    req: web::Json<CreatePipelineRequest>,
) -> impl Responder {
    match crate::db::create_pipeline(
        &client,
        &req.name,
        &req.content,
        req.method.clone(),
        &req.url,
    ).await {
        Ok(pipeline) => HttpResponse::Created().json(pipeline),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_pipeline(
    client: web::Data<Client>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::get_pipeline(&client, id.into_inner()).await {
        Ok(Some(pipeline)) => HttpResponse::Ok().json(pipeline),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_pipeline(
    client: web::Data<Client>,
    id: web::Path<Uuid>,
    req: web::Json<UpdatePipelineRequest>,
) -> impl Responder {
    match crate::db::update_pipeline(
        &client,
        id.into_inner(),
        &req.name,
        &req.content,
        req.method.clone(),
        &req.url,
    ).await {
        Ok(Some(pipeline)) => HttpResponse::Ok().json(pipeline),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_pipeline(
    client: web::Data<Client>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::delete_pipeline(&client, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pipelines")
            .route("", web::post().to(create_pipeline))
            .route("/{id}", web::get().to(get_pipeline))
            .route("/{id}", web::put().to(update_pipeline))
            .route("/{id}", web::delete().to(delete_pipeline))
    );
} 
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use tokio_postgres::Client;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateNodeRequest {
    name: String,
    content: serde_json::Value,
}

#[derive(Deserialize)]
pub struct UpdateNodeRequest {
    name: String,
    content: serde_json::Value,
}

pub async fn create_node(
    client: web::Data<Client>,
    req: web::Json<CreateNodeRequest>,
) -> impl Responder {
    match crate::db::create_node(&client, &req.name, &req.content).await {
        Ok(node) => HttpResponse::Created().json(node),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_node(
    client: web::Data<Client>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::get_node(&client, id.into_inner()).await {
        Ok(Some(node)) => HttpResponse::Ok().json(node),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_node(
    client: web::Data<Client>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateNodeRequest>,
) -> impl Responder {
    match crate::db::update_node(&client, id.into_inner(), &req.name, &req.content).await {
        Ok(Some(node)) => HttpResponse::Ok().json(node),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_node(
    client: web::Data<Client>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::delete_node(&client, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nodes")
            .route("", web::post().to(create_node))
            .route("/{id}", web::get().to(get_node))
            .route("/{id}", web::put().to(update_node))
            .route("/{id}", web::delete().to(delete_node))
    );
} 
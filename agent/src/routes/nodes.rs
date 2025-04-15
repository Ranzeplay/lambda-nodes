use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use serde::Deserialize;
use tokio_postgres::Client;
use uuid::Uuid;
use std::sync::Arc;

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

#[derive(Deserialize)]
pub struct ListNodesQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[post("")]
pub async fn create_node(
    client: web::Data<Arc<Client>>,
    req: web::Json<CreateNodeRequest>,
) -> impl Responder {
    match crate::db::create_node(&client, &req.name, &req.content).await {
        Ok(node) => HttpResponse::Created().json(node),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_node(
    client: web::Data<Arc<Client>>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::get_node(&client, id.into_inner()).await {
        Ok(Some(node)) => HttpResponse::Ok().json(node),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/{id}")]
pub async fn update_node(
    client: web::Data<Arc<Client>>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateNodeRequest>,
) -> impl Responder {
    match crate::db::update_node(&client, id.into_inner(), &req.name, &req.content).await {
        Ok(Some(node)) => HttpResponse::Ok().json(node),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_node(
    client: web::Data<Arc<Client>>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::delete_node(&client, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("")]
pub async fn list_nodes(
    client: web::Data<Arc<Client>>,
    query: web::Query<ListNodesQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    match crate::db::list_nodes(&client, limit, offset).await {
        Ok(nodes) => HttpResponse::Ok().json(nodes),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/count")]
pub async fn count_nodes(
    client: web::Data<Arc<Client>>,
) -> impl Responder {
    match crate::db::count_nodes(&client).await {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/nodes")
            .service(count_nodes)
            .service(create_node)
            .service(list_nodes)
            .service(get_node)
            .service(update_node)
            .service(delete_node)
    );
} 
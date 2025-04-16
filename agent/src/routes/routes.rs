use std::sync::Arc;
use crate::db::routes;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use tokio_postgres::Client;
use uuid::Uuid;

const DEFAULT_LIMIT: i64 = 20;
const DEFAULT_OFFSET: i64 = 0;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteCreateRequest {
    pipeline_id: Uuid,
    path: String,
    method: String,
}

#[derive(Deserialize)]
pub struct RouteUpdateRequest {
    path: String,
    method: String,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[post("")]
pub async fn create_route(
    client: web::Data<Arc<Client>>,
    route: web::Json<RouteCreateRequest>,
) -> impl Responder {
    let result = routes::create_route(
        &client,
        route.pipeline_id,
        &route.path,
        &route.method,
    ).await;

    match result {
        Ok(route) => HttpResponse::Created().json(route),
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to create route: {}", e)),
    }
}

#[get("{id}")]
pub async fn get_route(
    client: web::Data<Arc<Client>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    let result = routes::get_route(&client, id).await;

    match result {
        Ok(Some(route)) => HttpResponse::Ok().json(route),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get route: {}", e)),
    }
}

#[get("")]
pub async fn list_routes(
    client: web::Data<Arc<Client>>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT);
    let offset = query.offset.unwrap_or(DEFAULT_OFFSET);

    let result = routes::list_routes(&client, limit, offset).await;

    match result {
        Ok(routes) => HttpResponse::Ok().json(routes),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get route list: {}", e)),
    }
}

#[delete("{id}")]
pub async fn delete_route(
    client: web::Data<Arc<Client>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    let result = routes::delete_route(&client, id).await;

    match result {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete route: {}", e)),
    }
}

#[get("count")]
pub async fn count_routes(client: web::Data<Arc<Client>>,) -> impl Responder {
    let result = routes::count_routes(&client).await;

    match result {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get route count: {}", e)),
    }
}

#[get("/pipeline/{id}")]
pub async fn get_routes_by_pipeline(
    client: web::Data<Arc<Client>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let pipeline_id = path.into_inner();
    let result = routes::get_routes_by_pipeline_id(&client, pipeline_id).await;

    match result {
        Ok(routes) => HttpResponse::Ok().json(routes),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get route for a pipeline: {}", e)),
    }
}

#[put("{id}")]
pub async fn update_route(
    client: web::Data<Arc<Client>>,
    path: web::Path<Uuid>,
    route: web::Json<RouteUpdateRequest>,
) -> impl Responder {
    let id = path.into_inner();
    let result = routes::update_route(&client, id, &route.path, &route.method).await;

    match result {
        Ok(route) => HttpResponse::Ok().json(route),
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to update route: {}", e)),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/routes")
            .service(create_route)
            .service(get_route)
            .service(list_routes)
            .service(delete_route)
            .service(count_routes)
            .service(get_routes_by_pipeline)
            .service(update_route),
    );
}
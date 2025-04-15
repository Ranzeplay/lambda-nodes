use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use tokio_postgres::Client;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePipelineRequest {
    name: String,
    content: serde_json::Value,
    method: String,
    url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePipelineRequest {
    name: String,
    content: serde_json::Value,
    method: String,
    url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPipelinesQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[post("")]
pub async fn create_pipeline(
    client: web::Data<Arc<Client>>,
    req: web::Json<CreatePipelineRequest>,
) -> impl Responder {
    match crate::db::create_pipeline(
        &client,
        &req.name,
        &req.content,
        req.method.clone(),
        &req.url,
    )
        .await
    {
        Ok(pipeline) => HttpResponse::Created().json(pipeline),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_pipeline(client: web::Data<Arc<Client>>, id: web::Path<Uuid>) -> impl Responder {
    match crate::db::get_pipeline(&client, id.into_inner()).await {
        Ok(Some(pipeline)) => HttpResponse::Ok().json(pipeline),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/{id}")]
pub async fn update_pipeline(
    client: web::Data<Arc<Client>>,
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
    )
        .await
    {
        Ok(Some(pipeline)) => HttpResponse::Ok().json(pipeline),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_pipeline(
    client: web::Data<Arc<Client>>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::delete_pipeline(&client, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("")]
pub async fn list_pipelines(
    client: web::Data<Arc<Client>>,
    query: web::Query<ListPipelinesQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    match crate::db::list_pipelines(&client, limit, offset).await {
        Ok(pipelines) => HttpResponse::Ok().json(pipelines),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/count")]
pub async fn count_pipelines(client: web::Data<Arc<Client>>) -> impl Responder {
    match crate::db::count_pipelines(&client).await {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pipelines")
            .service(count_pipelines)
            .service(create_pipeline)
            .service(list_pipelines)
            .service(get_pipeline)
            .service(update_pipeline)
            .service(delete_pipeline),
    );
}

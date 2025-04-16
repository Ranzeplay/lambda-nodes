use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use tokio_postgres::Client;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHistoryStatusRequest {
    status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListHistoriesQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[get("/{id}")]
pub async fn get_history(client: web::Data<Arc<Client>>, id: web::Path<Uuid>) -> impl Responder {
    match crate::db::history::get_history(&client, id.into_inner()).await {
        Ok(Some(history)) => HttpResponse::Ok().json(history),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("")]
pub async fn list_histories(
    client: web::Data<Arc<Client>>,
    query: web::Query<ListHistoriesQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    match crate::db::history::list_histories(&client, limit, offset).await {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/pipeline/{pipeline_id}")]
pub async fn list_histories_by_pipeline(
    client: web::Data<Arc<Client>>,
    pipeline_id: web::Path<Uuid>,
    query: web::Query<ListHistoriesQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    match crate::db::history::list_histories_by_pipeline(&client, pipeline_id.into_inner(), limit, offset).await {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/{id}/status")]
pub async fn update_history_status(
    client: web::Data<Arc<Client>>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateHistoryStatusRequest>,
) -> impl Responder {
    match crate::db::history::update_history_status(&client, id.into_inner(), &req.status).await {
        Ok(Some(history)) => HttpResponse::Ok().json(history),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_history(
    client: web::Data<Arc<Client>>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::history::delete_history(&client, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/count")]
pub async fn count_histories(client: web::Data<Arc<Client>>) -> impl Responder {
    match crate::db::history::count_histories(&client).await {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/pipeline/{pipeline_id}/count")]
pub async fn count_histories_by_pipeline(
    client: web::Data<Arc<Client>>,
    pipeline_id: web::Path<Uuid>,
) -> impl Responder {
    match crate::db::history::count_histories_by_pipeline(&client, pipeline_id.into_inner()).await {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/history")
            .service(get_history)
            .service(list_histories)
            .service(list_histories_by_pipeline)
            .service(update_history_status)
            .service(delete_history)
            .service(count_histories)
            .service(count_histories_by_pipeline),
    );
}

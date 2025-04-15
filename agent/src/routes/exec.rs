use actix_web::{route, web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;
use serde_json::Value;
use tokio_postgres::Client;
use crate::db::flow::Graph;
use crate::exec_graph::exec_graph;

#[route(
    "/exec/{tail:.*}",
    method = "GET",
    method = "POST",
    method = "PUT",
    method = "DELETE",
    method = "PATCH"
)]
pub async fn exec(
    client: web::Data<Arc<Client>>,
    path: web::Path<String>,
    req: HttpRequest,
    json: web::Json<Value>,
) -> impl Responder {
    let tail = path.into_inner();

    println!("path: {}", tail);

    let pipeline_graph_result = client.query_one(
        "SELECT content FROM pipelines WHERE url = $1 AND method = $2 LIMIT 1",
        &[&tail, &req.method().as_str()],
    ).await;

    if let Err(e) = pipeline_graph_result {
        return HttpResponse::NotFound().body(format!("{:?}", e));
    }

    let pipeline_graph_item = pipeline_graph_result.unwrap().get(0);
    let pipeline_graph: Graph = serde_json::from_value(pipeline_graph_item).unwrap();

    exec_graph(pipeline_graph, json.into_inner(), &client).await;

    HttpResponse::Ok().finish()
}
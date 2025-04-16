use crate::db::flow::Graph;
use crate::executor::GraphExecutor;
use actix_web::{route, web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;
use std::sync::Arc;
use log::{info, warn};
use tokio_postgres::Client;
use crate::db::{create_log, LogLevel};

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

    let pipeline_result = client
        .query_one(
            "SELECT name, content FROM pipelines WHERE url = $1 AND method = $2 LIMIT 1",
            &[&tail, &req.method().as_str()],
        )
        .await;

    if let Err(e) = pipeline_result {
        return HttpResponse::NotFound().body(format!("{:?}", e));
    }
    
    let pipeline_graph_result = pipeline_result.unwrap();
    let pipeline_name: String = pipeline_graph_result.get(0);
    let pipeline_graph_item = pipeline_graph_result.get(1);
    let pipeline_graph: Graph = serde_json::from_value(pipeline_graph_item).unwrap();
    
    info!("Initializing GraphExecutor for pipeline graph: {}", pipeline_name);
    create_log(&client, LogLevel::Info, "Execution", &format!("Initializing GraphExecutor for pipeline graph: {}", pipeline_name)).await.unwrap();

    let mut executor = GraphExecutor::new(pipeline_graph, &client).await.unwrap();
    if let Err(_) = executor.init_entry(json.into_inner()) {
        warn!("Failed to initialize GraphExecutor for pipeline graph: {}", pipeline_name);
        
        create_log(&client, LogLevel::Error, "Execution", &format!("Failed to initialize GraphExecutor for pipeline graph: {}", pipeline_name)).await.unwrap();
        return HttpResponse::InternalServerError().body("Failed to initialize pipeline entry");
    }

    executor.init_node_queue();

    while !executor.reached_end {
        if let Err(_) = executor.exec_current_queue() {
            warn!("Failed to execute current queue for pipeline graph: {}", pipeline_name);
            create_log(&client, LogLevel::Error, "Execution", &format!("Failed to execute current queue for pipeline graph: {}", pipeline_name)).await.unwrap();
            
            return HttpResponse::InternalServerError().body("Failed to execute current queue");
        }

        executor.update_next_node_queue();
        executor.apply_next_queue();
    }

    let result = executor.get_result();
    if result.is_err() {
        warn!("Failed to get execution result for pipeline graph: {}", pipeline_name);
        create_log(&client, LogLevel::Error, "Execution", &format!("Failed to get execution result for pipeline graph: {}", pipeline_name)).await.unwrap();
        
        return HttpResponse::InternalServerError().body("Failed to get execution result");
    }

    create_log(&client, LogLevel::Info, "Execution", &format!("Execution completed for pipeline graph: {}", pipeline_name)).await.unwrap();
    HttpResponse::Ok().json(result.unwrap())
}

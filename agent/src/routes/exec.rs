use crate::db::flow::Graph;
use crate::db::history::{create_history, fail_history, success_history, update_history_status};
use crate::db::{create_log, LogLevel};
use crate::executor::GraphExecutor;
use actix_web::{route, web, HttpRequest, HttpResponse, Responder};
use log::{info, warn};
use serde_json::Value;
use std::sync::Arc;
use tokio_postgres::Client;
use uuid::Uuid;

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
            "SELECT name, content, id FROM pipelines WHERE url = $1 AND method = $2 LIMIT 1",
            &[&tail, &req.method().as_str()],
        )
        .await;

    if let Err(e) = pipeline_result {
        return HttpResponse::NotFound().body(format!("{:?}", e));
    }
    
    let pipeline_graph_result = pipeline_result.unwrap();
    let pipeline_name: String = pipeline_graph_result.get(0);
    let pipeline_graph_item = pipeline_graph_result.get(1);
    let pipeline_id: Uuid = pipeline_graph_result.get(2);
    let pipeline_graph: Graph = serde_json::from_value(pipeline_graph_item).unwrap();
    
    let history = create_history(&client, pipeline_id, "preparing").await;
    if let Err(e) = history {
        warn!("Failed to create history: {:?}", e);
        create_log(&client, LogLevel::Error, "Execution", "Failed to create history").await.unwrap();
        
        return HttpResponse::InternalServerError().body(format!("{:?}", e));
    }
    let history = history.unwrap();
    
    info!("Initializing GraphExecutor for pipeline graph: {}", pipeline_name);
    create_log(&client, LogLevel::Info, "Execution", &format!("Initializing GraphExecutor for pipeline graph: {}", pipeline_name)).await.unwrap();
    
    let mut executor = GraphExecutor::new(pipeline_graph, &client).await.unwrap();
    if let Err(e) = executor.init_entry(json.into_inner()) {
        warn!("Failed to initialize GraphExecutor for pipeline graph: {}", pipeline_name);
        create_log(&client, LogLevel::Error, "Execution", &format!("Failed to initialize GraphExecutor for pipeline graph: {}", pipeline_name)).await.unwrap();
        fail_history(&client, history.id, e.to_string().as_str()).await.unwrap();
        
        return HttpResponse::InternalServerError().body("Failed to initialize pipeline entry");
    }

    executor.init_node_queue();
    update_history_status(&client, history.id, "running").await.unwrap();
    
    while !executor.reached_end {
        if let Err(e) = executor.exec_current_queue() {
            warn!("Failed to execute current queue for pipeline graph: {}", pipeline_name);
            create_log(&client, LogLevel::Error, "Execution", &format!("Failed to execute current queue for pipeline graph: {}", pipeline_name)).await.unwrap();
            fail_history(&client, history.id, e.to_string().as_str()).await.unwrap();
            
            return HttpResponse::InternalServerError().body("Failed to execute current queue");
        }

        executor.update_next_node_queue();
        executor.apply_next_queue();
    }

    let result = executor.get_result();
    if let Err(e) = result {
        warn!("Failed to get execution result for pipeline graph: {}", pipeline_name);
        create_log(&client, LogLevel::Error, "Execution", &format!("Failed to get execution result for pipeline graph: {}", pipeline_name)).await.unwrap();
        fail_history(&client, history.id, e.to_string().as_str()).await.unwrap();
        
        return HttpResponse::InternalServerError().body("Failed to get execution result");
    }
    let result = result.unwrap();

    info!("Execution completed for pipeline graph: {}", pipeline_name);
    create_log(&client, LogLevel::Info, "Execution", &format!("Execution completed for pipeline graph: {}", pipeline_name)).await.unwrap();
    success_history(&client, history.id, Some(result.clone())).await.unwrap();
    
    HttpResponse::Ok().json(result)
}

use crate::db::flow::{Graph, GraphNode};
use crate::db::get_node;
use crate::db::models::Node;
use deno_core::_ops::RustToV8;
use deno_core::serde_v8::to_v8;
use deno_core::v8::{Local, Object};
use deno_core::{serde_v8, v8, JsRuntime, RuntimeOptions};
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;
use deno_core::error::AnyError;
use tokio_postgres::Client;
use uuid::Uuid;

pub async fn exec_graph(graph: Graph, init_value: Value, client: &Client) -> Result<Value, AnyError> {
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    // Get all nodes
    let node_ids = graph
        .nodes
        .iter()
        .map(|node| node.data.id.clone())
        .collect::<Vec<_>>();

    let mut nodes = vec![];
    for node_id in node_ids {
        let node_id = node_id.parse::<Uuid>()?;
        let node = get_node(client, node_id).await;
        match node {
            Ok(Some(node)) => {
                nodes.push(node);
            }
            Ok(None) => {
                eprintln!("Node not found");
            }
            Err(e) => {
                eprintln!("Error fetching node: {}", e);
            }
        }
    }

    let entry_node = nodes
        .iter()
        .find(|node| node.name == "BeginRequest" && node.is_internal)
        .unwrap();
    println!("BeginRequest: {}", entry_node.id.to_string());

    let graph_entry = graph
        .nodes
        .iter()
        .find(|node| node.data.id == entry_node.id.to_string())
        .unwrap();

    let combined_nodes = get_combined_nodes(graph.clone(), nodes);

    let mut data_cache: HashMap<String, HashMap<String, Local<Object>>> = HashMap::new();

    let current_node = combined_nodes.get(&graph_entry.id).unwrap();

    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    let scope = &mut runtime.handle_scope();

    let data = to_v8(scope, init_value)?
        .to_object(scope)
        .unwrap();

    let mut init_out = HashMap::new();
    init_out.insert("data".to_string(), data);
    data_cache.insert(graph_entry.id.to_string(), init_out);

    loop {
        let step_edge = graph
            .edges
            .iter()
            .find(|edge| {
                edge.source == current_node.graph_node.id && edge.source_handle == "from-node"
            })
            .unwrap();
        let current_node = combined_nodes.get(step_edge.target.as_str()).unwrap().clone();

        let data_edges = graph
            .edges
            .iter()
            .filter(|edge| edge.target == current_node.graph_node.id)
            .collect::<Vec<_>>();

        let mut in_data: HashMap<String, Local<Object>> = HashMap::new();

        for edge in data_edges {
            in_data.insert(
                edge.target_handle.clone(),
                data_cache
                    .get(&edge.source)
                    .unwrap()
                    .get(edge.source_handle.as_str())
                    .unwrap_or(&to_v8(scope, {})?.to_object(scope).unwrap())
                    .clone(),
            );
        }

        let in_obj = to_v8(scope, {})?;
        let in_obj = in_obj.to_object(scope).unwrap();

        for (key, value) in in_data.iter() {
            let v8_key = v8::String::new(scope, key).unwrap();
            in_obj.set(scope, v8_key.into(), (*value).into()).unwrap();
        }

        if current_node.db_node.is_internal && current_node.db_node.name == "EndRequest" {
            let final_result = in_data.get("data").unwrap();

            return serde_v8::from_v8::<Value>(scope, (*final_result).into())
                .map_err(|e| AnyError::from(e));
        }

        let result = run_js(&mut js_runtime, &current_node.db_node.name, &current_node.db_node.script, scope)?;

        let mut out_data = HashMap::new();
        for out_item in &current_node.db_node.outputs {
            let out_key = v8::String::new(scope, out_item.as_str()).unwrap();
            let out_value = result.get(scope, out_key.into()).unwrap();
            let out_value = out_value.to_object(scope).unwrap();
            out_data.insert(out_item.clone(), out_value);
        }

        data_cache.insert(graph_entry.id.to_string(), in_data);
    }
}

fn run_js<'a>(
    js_runtime: &'a mut JsRuntime,
    name: &'a str,
    script: &'a str,
    scope: &'a mut v8::HandleScope<'a>,
) -> Result<Local<'a, Object>, AnyError> {
    let result = js_runtime.execute_script(name, script)?
        .to_v8(scope)
        .to_object(scope)
        .unwrap();
    Ok(result)
}

#[derive(Debug, Clone)]
struct CombinedNode {
    graph_node: GraphNode,
    db_node: Node,
}

fn get_combined_nodes(graph: Graph, db_nodes: Vec<Node>) -> HashMap<String, CombinedNode> {
    let mut combined_nodes = HashMap::new();

    for node in graph.nodes {
        let db_node = db_nodes
            .iter()
            .find(|db_node| db_node.id.to_string() == node.data.id)
            .unwrap();

        let combined_node = CombinedNode {
            graph_node: node.clone(),
            db_node: db_node.clone(),
        };

        combined_nodes.insert(node.data.id, combined_node);
    }

    combined_nodes
}

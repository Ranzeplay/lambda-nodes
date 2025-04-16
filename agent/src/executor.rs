use crate::blocks::boolean::{bool_false, bool_true};
use crate::db::flow::{Graph, GraphNode};
use crate::db::get_node;
use crate::db::models::Node;
use deno_core::_ops::{RustToV8, RustToV8NoScope};
use deno_core::error::AnyError;
use deno_core::serde_v8::to_v8;
use deno_core::v8::{ContextOptions, Function, Global, HandleScope, Local, ObjectTemplate};
use deno_core::{serde_v8, v8, JsRuntime, RuntimeOptions};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use tokio_postgres::Client;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CombinedNode {
    pub graph_node: GraphNode,
    pub db_node: Node,
}

pub struct GraphExecutor {
    nodes: HashMap<String, CombinedNode>,
    graph: Graph,
    runtime: JsRuntime,
    data_cache: HashMap<String, HashMap<String, Global<v8::Value>>>,
    current_node: CombinedNode,
    entry_node_graph_id: String,
    end_node_graph_id: String,

    pub current_node_queue: Vec<CombinedNode>,
    pub next_node_queue: Vec<CombinedNode>,
    pub reached_end: bool,
}

impl GraphExecutor {
    pub async fn new(graph: Graph, client: &Client) -> Result<Self, AnyError> {
        let runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            ..Default::default()
        });
        let data_cache = HashMap::new();

        // Fetch nodes from the database
        let node_ids = graph
            .nodes
            .iter()
            .map(|node| node.data.id.parse::<Uuid>().unwrap());
        let mut nodes = vec![];
        for node_id in node_ids {
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
        let nodes = get_combined_nodes(graph.clone(), nodes);

        let entry_node = nodes
            .iter()
            .map(|node| node.1.clone())
            .find(|node| node.db_node.name == "BeginRequest" && node.db_node.is_internal)
            .unwrap();
        let end_node = nodes
            .iter()
            .map(|node| node.1.clone())
            .find(|node| node.db_node.name == "EndRequest" && node.db_node.is_internal)
            .unwrap();

        Ok(GraphExecutor {
            nodes,
            graph,
            runtime,
            data_cache,
            current_node: entry_node.clone(),
            entry_node_graph_id: entry_node.graph_node.id.clone(),
            end_node_graph_id: end_node.graph_node.id.clone(),
            current_node_queue: vec![],
            next_node_queue: vec![],
            reached_end: false,
        })
    }

    pub fn init_entry(&mut self, init_value: serde_json::Value) -> Result<(), AnyError> {
        let scope = &mut self.runtime.handle_scope();
        let mut init_out = HashMap::new();
        let global_obj = to_v8(scope, init_value)?.to_v8();
        let global_obj = Global::new(scope, global_obj);
        init_out.insert("data".to_string(), global_obj);
        self.data_cache
            .insert(self.entry_node_graph_id.clone(), init_out);

        Ok(())
    }

    pub fn init_node_queue(&mut self) {
        let nodes = self
            .nodes
            .values()
            .filter(|&n| {
                self.graph
                    .edges
                    .iter()
                    .find(|&e| e.target == n.graph_node.id && e.target_handle == "to-node")
                    .is_none()
            })
            .map(|n| n.clone())
            .collect::<Vec<CombinedNode>>();
        self.current_node_queue = nodes;
    }

    pub fn update_next_node_queue(&mut self) {
        let mut nodes = vec![];
        for node in &self.current_node_queue {
            let edges = self
                .graph
                .edges
                .iter()
                .filter(|edge| {
                    edge.source == node.graph_node.id && edge.source_handle == "from-node"
                })
                .collect::<Vec<_>>();

            for edge in edges {
                let target_node = self
                    .nodes
                    .get(&edge.target)
                    .expect(format!("Target node not found for edge: {}", edge.id).as_str());

                nodes.push(target_node.clone());
            }
        }
        // Distinct by node graph id
        // nodes.dedup_by(|a, b| a.graph_node.id == b.graph_node.id);

        self.next_node_queue = nodes;
    }

    pub fn exec_current_queue(&mut self) -> Result<(), AnyError> {
        let queue = self.current_node_queue.clone();
        for node in queue {
            self.current_node = node;
            self.exec_current_node()?;
        }
        Ok(())
    }

    pub fn apply_next_queue(&mut self) {
        self.current_node_queue = self.next_node_queue.clone();
        self.next_node_queue = vec![];
    }

    pub fn exec_current_node(&mut self) -> Result<(), AnyError> {
        let isolated = self.runtime.v8_isolate();
        let handle_scope = &mut v8::HandleScope::new(isolated);
        let context = v8::Context::new(
            handle_scope,
            ContextOptions {
                global_template: None,
                global_object: None,
                microtask_queue: None,
            },
        );
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        if self.current_node.graph_node.id == self.entry_node_graph_id {
            return Ok(());
        }

        let in_data = self
            .graph
            .edges
            .iter()
            .filter(|edge| {
                edge.target == self.current_node.graph_node.id && edge.target_handle != "to-node"
            })
            .map(|edge| {
                let source_data = self
                    .data_cache
                    .get(&edge.source)
                    .expect(format!("Source data not found for edge: {}", edge.id).as_str());
                let source_handle = edge.source_handle.trim_start_matches("output-").to_string();
                let data = source_data.get(&source_handle).unwrap().clone();

                let target_handle = edge.target_handle.trim_start_matches("input-").to_string();
                (target_handle, data)
            })
            .collect::<HashMap<_, _>>();

        if self.current_node.db_node.is_internal {
            return process_internal_nodes(
                &self.current_node,
                &mut self.current_node_queue,
                &mut self.data_cache,
                &mut self.reached_end,
                &mut self.end_node_graph_id,
                in_data,
                scope,
            );
        }

        // Build the input object for the current node
        let in_obj_map = {
            let in_obj_template = ObjectTemplate::new(scope);
            let in_obj = in_obj_template.new_instance(scope).unwrap();
            for (key, value) in in_data.iter() {
                let v8_key = v8::String::new(scope, key).unwrap();
                let v8_value = value.clone().to_v8(scope);
                in_obj.set(scope, v8_key.into(), v8_value).unwrap();
            }

            in_obj.to_v8()
        };

        // Execute the current node's script
        let current_node = self
            .nodes
            .get(&self.current_node.graph_node.id)
            .expect("Current node not found");
        let script = current_node.db_node.script.clone();

        let fn_name = v8::String::new(scope, "handle").unwrap();
        let script = v8::String::new(scope, &script).unwrap();
        let script = v8::Script::compile(scope, script, None).unwrap();
        script.run(scope).expect("Failed to execute script");

        let global = context.global(scope);
        let function_obj = global.get(scope, fn_name.into()).unwrap();

        let function: Local<Function> = function_obj.cast();

        let result = function.call(scope, function_obj.into(), &[in_obj_map]);

        if result.is_none() {
            return Err(AnyError::msg(
                "Function call failed: ".to_owned() + &*self.current_node.graph_node.id,
            ));
        }

        let result = result.unwrap().to_object(scope).unwrap();

        let out_data = current_node
            .db_node
            .outputs
            .iter()
            .map(|out_item| {
                let out_key = v8::String::new(scope, out_item.as_str()).unwrap();
                let out_value = result.get(scope, out_key.into()).unwrap();
                (out_item.clone(), Global::new(scope, out_value))
            })
            .collect::<HashMap<_, _>>();

        // Store the output data in the data cache
        self.data_cache
            .insert(self.current_node.graph_node.id.clone(), out_data);

        Ok(())
    }

    pub fn get_result(&mut self) -> Result<serde_json::Value, AnyError> {
        let scope = &mut self.runtime.handle_scope();

        let final_result = self
            .data_cache
            .get(&self.end_node_graph_id)
            .unwrap()
            .get("data")
            .unwrap()
            .clone()
            .to_v8(scope);
        let result = serde_v8::from_v8::<serde_json::Value>(scope, final_result)?;
        Ok(result)
    }
}

fn process_internal_nodes(
    current_node: &CombinedNode,
    current_node_queue: &mut Vec<CombinedNode>,
    data_cache: &mut HashMap<String, HashMap<String, Global<v8::Value>>>,
    reached_end: &mut bool,
    end_node_graph_id: &mut String,
    in_data: HashMap<String, Global<v8::Value>>,
    scope: &mut HandleScope,
) -> Result<(), AnyError> {
    if current_node.db_node.name == "Breaker" {
        let condition = in_data.get("condition").unwrap().clone();
        let condition = condition.to_v8(scope);
        let condition = serde_v8::from_v8::<bool>(scope, condition)?;

        if !condition {
            *current_node_queue = current_node_queue
                .iter()
                .filter(|node| node.graph_node.id != current_node.graph_node.id)
                .cloned()
                .collect();
        }
    } else if current_node.db_node.name == "True" {
        let mut out_data = HashMap::new();
        out_data.insert("out".to_string(), bool_true(scope));
        data_cache.insert(current_node.graph_node.id.clone(), out_data);
    } else if current_node.db_node.name == "False" {
        let mut out_data = HashMap::new();
        out_data.insert("out".to_string(), bool_false(scope));
        data_cache.insert(current_node.graph_node.id.clone(), out_data);
    } else if current_node.db_node.name == "Empty" {
        let obj = v8::undefined(scope).to_v8();
        let obj = Global::new(scope, obj);

        let mut out_data = HashMap::new();
        out_data.insert("out".to_string(), obj);
        data_cache.insert(current_node.graph_node.id.clone(), out_data);
    } else if current_node.db_node.name == "EndRequest" {
        *reached_end = true;
        *end_node_graph_id = current_node.graph_node.id.clone();
        data_cache.insert(current_node.graph_node.id.clone(), in_data);
    } else {
        let err = fmt::Error {};
        return Err(AnyError::new(err).context("Internal node not found"));
    }
    
    Ok(())
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

        combined_nodes.insert(node.id.to_string(), combined_node);
    }

    combined_nodes
}

#[allow(dead_code)]
fn value_to_json(
    scope: &mut v8::HandleScope,
    value: Local<v8::Value>,
) -> Result<String, anyhow::Error> {
    // First deserialize the V8 value to a Rust value that serde can handle
    let rust_value: serde_json::Value = serde_v8::from_v8(scope, value)?;

    // Then serialize the Rust value to a JSON string
    let json_string = serde_json::to_string(&rust_value)?;

    Ok(json_string)
}

#[allow(dead_code)]
pub fn local_object_to_json(
    scope: &mut v8::HandleScope,
    value: Local<v8::Object>,
) -> Result<String, anyhow::Error> {
    // First deserialize the V8 value to a Rust value that serde can handle
    let value = value.to_v8();
    let rust_value: serde_json::Value = serde_v8::from_v8(scope, value)?;

    // Then serialize the Rust value to a JSON string
    let json_string = serde_json::to_string(&rust_value)?;

    Ok(json_string)
}

#[allow(dead_code)]
pub fn global_object_to_json(
    scope: &mut v8::HandleScope,
    value: Global<v8::Object>,
) -> Result<String, anyhow::Error> {
    // First deserialize the V8 value to a Rust value that serde can handle
    let value = value.to_v8(scope);
    let rust_value: serde_json::Value = serde_v8::from_v8(scope, value)?;

    // Then serialize the Rust value to a JSON string
    let json_string = serde_json::to_string(&rust_value)?;

    Ok(json_string)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub position: GraphNodePosition,
    pub data: GraphNodeData,
    pub measured: GraphNodeMeasured,
    pub selected: bool,
    pub dragging: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNodePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNodeData {
    pub id: String,
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphNodeMeasured {
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphEdge {
    pub source: String,
    #[serde(rename = "sourceHandle")]
    pub source_handle: String,
    pub target: String,
    #[serde(rename = "targetHandle")]
    pub target_handle: String,
    pub id: String,
}

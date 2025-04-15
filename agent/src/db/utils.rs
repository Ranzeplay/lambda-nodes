use anyhow::Result;
use std::error::Error;
use tokio_postgres::Row;
use crate::db::models::{Log, Node, Pipeline};

// Helper function to convert serde_json::Value to SQL string
pub fn json_to_sql(value: &serde_json::Value) -> String {
    value.to_string()
}

// Helper function to convert SQL string to serde_json::Value
pub fn sql_to_json(s: &str) -> Result<serde_json::Value, Box<dyn Error + Sync + Send>> {
    Ok(serde_json::from_str(s)?)
}

// Helper functions to convert database rows to structs
pub fn row_to_log(row: Row) -> Log {
    Log {
        id: row.get(0),
        level: row.get(1),
        message: row.get(2),
        create_at: row.get(3),
    }
}

pub fn row_to_node(row: Row) -> Node {
    Node {
        id: row.get(0),
        is_internal: row.get(1),
        name: row.get(2),
        script: row.get(3),
        inputs: row.get(4),
        outputs: row.get(5),
    }
}

pub fn row_to_pipeline(row: Row) -> Pipeline {
    let content_str: String = row.get(2);
    Pipeline {
        id: row.get(0),
        name: row.get(1),
        content: sql_to_json(&content_str).unwrap_or(serde_json::Value::Null),
        method: row.get(3),
        url: row.get(4),
    }
} 
use crate::db::models::{Log, Node, Pipeline};
use tokio_postgres::Row;

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
    Pipeline {
        id: row.get(0),
        name: row.get(1),
        content: row.get(2),
        method: row.get(3),
        url: row.get(4),
    }
} 
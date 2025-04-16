use crate::db::models::{History, Log, Node, Pipeline, Route};
use tokio_postgres::Row;

// Helper functions to convert database rows to structs
pub fn row_to_log(row: Row) -> Log {
    Log {
        id: row.get(0),
        level: row.get(1),
        category: row.get(2),
        message: row.get(3),
        create_at: row.get(4),
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
    }
}

pub fn row_to_history(row: Row) -> History {
    History {
        id: row.get("id"),
        pipeline_id: row.get("pipeline_id"),
        status: row.get("status"),
        start_at: row.get("start_at"),
        end_at: row.get("end_at"),
        error: row.get("error"),
        result: row.get("result"),
    }
}

pub fn row_to_route(row: Row) -> Route {
    Route {
        id: row.get("id"),
        pipeline_id: row.get("pipeline_id"),
        path: row.get("path"),
        method: row.get("method"),
    }
}

use anyhow::Result;
use tokio_postgres::Client;
use uuid::Uuid;
use crate::db::models::Node;
use crate::db::utils::{json_to_sql, row_to_node};

pub async fn create_node(client: &Client, name: &str, content: &serde_json::Value) -> Result<Node> {
    let content_str = json_to_sql(content);
    let row = client
        .query_one(
            "INSERT INTO nodes (name, content) VALUES ($1, $2) RETURNING id, name, content",
            &[&name, &content_str],
        )
        .await?;
    Ok(row_to_node(row))
}

pub async fn get_node(client: &Client, id: Uuid) -> Result<Option<Node>> {
    let row = client
        .query_opt(
            "SELECT id, name, content FROM nodes WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(row.map(row_to_node))
}

pub async fn update_node(
    client: &Client,
    id: Uuid,
    name: &str,
    content: &serde_json::Value,
) -> Result<Option<Node>> {
    let content_str = json_to_sql(content);
    let row = client
        .query_opt(
            "UPDATE nodes SET name = $2, content = $3 WHERE id = $1 RETURNING id, name, content",
            &[&id, &name, &content_str],
        )
        .await?;
    Ok(row.map(row_to_node))
}

pub async fn delete_node(client: &Client, id: Uuid) -> Result<bool> {
    let rows_affected = client
        .execute("DELETE FROM nodes WHERE id = $1", &[&id])
        .await?;
    Ok(rows_affected > 0)
}

pub async fn list_nodes(client: &Client, limit: i64, offset: i64) -> Result<Vec<Node>> {
    let rows = client
        .query(
            "SELECT id, name, content FROM nodes ORDER BY name ASC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await?;
    Ok(rows.into_iter().map(row_to_node).collect())
}

pub async fn count_nodes(client: &Client) -> Result<i64> {
    let row = client
        .query_one("SELECT COUNT(*) FROM nodes", &[])
        .await?;
    Ok(row.get(0))
} 
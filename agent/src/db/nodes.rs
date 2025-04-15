use anyhow::Result;
use tokio_postgres::Client;
use uuid::Uuid;
use crate::db::models::Node;
use crate::db::utils::row_to_node;

pub async fn create_node(
    client: &Client, 
    name: &str, 
    script: &str, 
    inputs: &[String], 
    outputs: &[String]
) -> Result<Node> {
    let row = client
        .query_one(
            "INSERT INTO nodes (name, script, inputs, outputs) VALUES ($1, $2, $3, $4) RETURNING id, is_internal, name, script, inputs, outputs",
            &[&name, &script, &inputs, &outputs],
        )
        .await?;
    Ok(row_to_node(row))
}

pub async fn get_node(client: &Client, id: Uuid) -> Result<Option<Node>> {
    let row = client
        .query_opt(
            "SELECT id, is_internal, name, script, inputs, outputs FROM nodes WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(row.map(row_to_node))
}

pub async fn update_node(
    client: &Client,
    id: Uuid,
    name: &str,
    script: &str,
    inputs: &[String],
    outputs: &[String],
) -> Result<Option<Node>> {
    let row = client
        .query_opt(
            "UPDATE nodes SET name = $2, script = $3, inputs = $4, outputs = $5 WHERE id = $1 RETURNING id, is_internal, name, script, inputs, outputs",
            &[&id, &name, &script, &inputs, &outputs],
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
            "SELECT id, is_internal, name, script, inputs, outputs FROM nodes ORDER BY name ASC LIMIT $1 OFFSET $2",
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
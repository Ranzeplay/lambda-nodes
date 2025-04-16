use crate::db::models::Pipeline;
use crate::db::utils::row_to_pipeline;
use anyhow::Result;
use tokio_postgres::Client;
use uuid::Uuid;

pub async fn create_pipeline(
    client: &Client,
    name: &str,
    content: &serde_json::Value,
) -> Result<Pipeline> {
    let row = client
        .query_one(
            "INSERT INTO pipelines (name, content) VALUES ($1, $2) RETURNING id, name, content",
            &[&name, &content],
        )
        .await?;
    Ok(row_to_pipeline(row))
}

pub async fn get_pipeline(client: &Client, id: Uuid) -> Result<Option<Pipeline>> {
    let row = client
        .query_opt(
            "SELECT id, name, content FROM pipelines WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(row.map(row_to_pipeline))
}

pub async fn update_pipeline(
    client: &Client,
    id: Uuid,
    name: &str,
    content: &serde_json::Value,
) -> Result<Option<Pipeline>> {
    let row = client
        .query_opt(
            "UPDATE pipelines SET name = $2, content = $3 WHERE id = $1 RETURNING id, name, content",
            &[&id, &name, &content],
        )
        .await?;
    Ok(row.map(row_to_pipeline))
}

pub async fn delete_pipeline(client: &Client, id: Uuid) -> Result<bool> {
    let rows_affected = client
        .execute("DELETE FROM pipelines WHERE id = $1", &[&id])
        .await?;
    Ok(rows_affected > 0)
}

pub async fn list_pipelines(client: &Client, limit: i64, offset: i64) -> Result<Vec<Pipeline>> {
    let rows = client
        .query(
            "SELECT id, name, content FROM pipelines ORDER BY name ASC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await?;
    Ok(rows.into_iter().map(row_to_pipeline).collect())
}

pub async fn count_pipelines(client: &Client) -> Result<i64> {
    let row = client
        .query_one("SELECT COUNT(*) FROM pipelines", &[])
        .await?;
    Ok(row.get(0))
}

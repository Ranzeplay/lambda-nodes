use crate::db::models::History;
use crate::db::utils::row_to_history;
use anyhow::Result;
use chrono::Utc;
use serde_json::Value;
use tokio_postgres::Client;
use uuid::Uuid;

pub async fn create_history(client: &Client, pipeline_id: Uuid, status: &str) -> Result<History> {
    let row = client
        .query_one(
            "INSERT INTO history (pipeline_id, status) VALUES ($1, $2) RETURNING id, pipeline_id, status, start_at, end_at, error, result",
            &[&pipeline_id, &status],
        )
        .await?;
    Ok(row_to_history(row))
}

pub async fn get_history(client: &Client, id: Uuid) -> Result<Option<History>> {
    let row = client
        .query_opt(
            "SELECT id, pipeline_id, status, start_at, end_at, error, result FROM history WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(row.map(row_to_history))
}

pub async fn list_histories(client: &Client, limit: i64, offset: i64) -> Result<Vec<History>> {
    let rows = client
        .query(
            "SELECT id, pipeline_id, status, start_at, end_at, error, result FROM history ORDER BY start_at DESC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await?;
    Ok(rows.into_iter().map(row_to_history).collect())
}

pub async fn list_histories_by_pipeline(
    client: &Client,
    pipeline_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<History>> {
    let rows = client
        .query(
            "SELECT id, pipeline_id, status, start_at, end_at, error, result FROM history WHERE pipeline_id = $1 ORDER BY start_at DESC LIMIT $2 OFFSET $3",
            &[&pipeline_id, &limit, &offset],
        )
        .await?;
    Ok(rows.into_iter().map(row_to_history).collect())
}

pub async fn update_history_status(
    client: &Client,
    id: Uuid,
    status: &str,
) -> Result<Option<History>> {
    let row = client
        .query_opt(
            "UPDATE history SET status = $2 WHERE id = $1 RETURNING id, pipeline_id, status, start_at, end_at, error, result",
            &[&id, &status],
        )
        .await?;
    Ok(row.map(row_to_history))
}

pub async fn complete_history(
    client: &Client,
    id: Uuid,
    status: &str,
    result: Option<Value>,
) -> Result<Option<History>> {
    let now = Utc::now();
    let row = client
        .query_opt(
            "UPDATE history SET status = $2, end_at = $3, result = $4 WHERE id = $1 RETURNING id, pipeline_id, status, start_at, end_at, error, result",
            &[&id, &status, &now, &result],
        )
        .await?;
    Ok(row.map(row_to_history))
}

pub async fn success_history(
    client: &Client,
    id: Uuid,
    result: Option<Value>,
) -> Result<Option<History>> {
    complete_history(client, id, "succeeded", result).await
}

pub async fn fail_history(client: &Client, id: Uuid, error: &str) -> Result<Option<History>> {
    let now = Utc::now();
    let row = client
        .query_opt(
            "UPDATE history SET status = 'failed', end_at = $2, error = $3 WHERE id = $1 RETURNING id, pipeline_id, status, start_at, end_at, error, result",
            &[&id, &now, &error],
        )
        .await?;
    Ok(row.map(row_to_history))
}

pub async fn delete_history(client: &Client, id: Uuid) -> Result<bool> {
    let rows_affected = client
        .execute("DELETE FROM history WHERE id = $1", &[&id])
        .await?;
    Ok(rows_affected > 0)
}

pub async fn count_histories(client: &Client) -> Result<i64> {
    let row = client
        .query_one("SELECT COUNT(*) FROM history", &[])
        .await?;
    Ok(row.get(0))
}

pub async fn count_histories_by_pipeline(client: &Client, pipeline_id: Uuid) -> Result<i64> {
    let row = client
        .query_one(
            "SELECT COUNT(*) FROM history WHERE pipeline_id = $1",
            &[&pipeline_id],
        )
        .await?;
    Ok(row.get(0))
}

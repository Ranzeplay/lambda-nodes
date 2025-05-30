use crate::db::models::{Log, LogLevel};
use crate::db::utils::row_to_log;
use anyhow::Result;
use tokio_postgres::Client;

pub async fn create_log(client: &Client, level: LogLevel, category: &str, message: &str) -> Result<Log> {
    let row = client
        .query_one(
            "INSERT INTO logs (level, category, message) VALUES ($1, $2, $3) RETURNING id, level, category, message, create_at",
            &[&level, &category, &message],
        )
        .await?;
    Ok(row_to_log(row))
}

pub async fn get_log(client: &Client, id: i32) -> Result<Option<Log>> {
    let row = client
        .query_opt(
            "SELECT id, level, category, message, create_at FROM logs WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(row.map(row_to_log))
}

pub async fn list_logs(client: &Client, limit: i64, offset: i64) -> Result<Vec<Log>> {
    let rows = client
        .query(
            "SELECT id, level, category, message, create_at FROM logs ORDER BY create_at DESC LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await?;
    Ok(rows.into_iter().map(row_to_log).collect())
}

pub async fn delete_log(client: &Client, id: i32) -> Result<bool> {
    let rows_affected = client
        .execute("DELETE FROM logs WHERE id = $1", &[&id])
        .await?;
    Ok(rows_affected > 0)
}

pub async fn count_logs(client: &Client) -> Result<i64> {
    let row = client.query_one("SELECT COUNT(*) FROM logs", &[]).await?;
    Ok(row.get(0))
}

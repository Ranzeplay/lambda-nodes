use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Row, types::{Type, IsNull, to_sql_checked, FromSql, ToSql}};
use uuid::Uuid;
use std::error::Error;
use postgres_types::private::BytesMut;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl<'a> ToSql for LogLevel {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::VARCHAR
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for LogLevel {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s: String = FromSql::from_sql(ty, raw)?;
        match s.as_str() {
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("invalid log level: {}", s).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::VARCHAR
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub id: i32,
    pub level: LogLevel,
    pub message: String,
    pub create_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl<'a> ToSql for HttpMethod {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
        }.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::VARCHAR
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for HttpMethod {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s: String = FromSql::from_sql(ty, raw)?;
        match s.as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(format!("invalid HTTP method: {}", s).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::VARCHAR
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub content: serde_json::Value,
    pub method: HttpMethod,
    pub url: String,
}

// Helper function to convert serde_json::Value to SQL string
fn json_to_sql(value: &serde_json::Value) -> String {
    value.to_string()
}

// Helper function to convert SQL string to serde_json::Value
fn sql_to_json(s: &str) -> Result<serde_json::Value, Box<dyn Error + Sync + Send>> {
    Ok(serde_json::from_str(s)?)
}

// Log operations
pub async fn create_log(client: &Client, level: LogLevel, message: &str) -> Result<Log> {
    let row = client
        .query_one(
            "INSERT INTO logs (level, message) VALUES ($1, $2) RETURNING id, level, message, create_at",
            &[&level, &message],
        )
        .await?;
    Ok(row_to_log(row))
}

pub async fn get_log(client: &Client, id: i32) -> Result<Option<Log>> {
    let row = client
        .query_opt(
            "SELECT id, level, message, create_at FROM logs WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(row.map(row_to_log))
}

pub async fn list_logs(client: &Client, limit: i64, offset: i64) -> Result<Vec<Log>> {
    let rows = client
        .query(
            "SELECT id, level, message, create_at FROM logs ORDER BY create_at DESC LIMIT $1 OFFSET $2",
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

// Node operations
pub async fn create_node(client: &Client, name: &str, content: &serde_json::Value) -> Result<Node> {
    let id = Uuid::new_v4();
    let content_str = json_to_sql(content);
    let row = client
        .query_one(
            "INSERT INTO nodes (id, name, content) VALUES ($1, $2, $3) RETURNING id, name, content",
            &[&id, &name, &content_str],
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

// Pipeline operations
pub async fn create_pipeline(
    client: &Client,
    name: &str,
    content: &serde_json::Value,
    method: HttpMethod,
    url: &str,
) -> Result<Pipeline> {
    let id = Uuid::new_v4();
    let content_str = json_to_sql(content);
    let row = client
        .query_one(
            "INSERT INTO pipelines (id, name, content, method, url) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, content, method, url",
            &[&id, &name, &content_str, &method, &url],
        )
        .await?;
    Ok(row_to_pipeline(row))
}

pub async fn get_pipeline(client: &Client, id: Uuid) -> Result<Option<Pipeline>> {
    let row = client
        .query_opt(
            "SELECT id, name, content, method, url FROM pipelines WHERE id = $1",
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
    method: HttpMethod,
    url: &str,
) -> Result<Option<Pipeline>> {
    let content_str = json_to_sql(content);
    let row = client
        .query_opt(
            "UPDATE pipelines SET name = $2, content = $3, method = $4, url = $5 WHERE id = $1 RETURNING id, name, content, method, url",
            &[&id, &name, &content_str, &method, &url],
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

// Helper functions to convert database rows to structs
fn row_to_log(row: Row) -> Log {
    Log {
        id: row.get(0),
        level: row.get(1),
        message: row.get(2),
        create_at: row.get(3),
    }
}

fn row_to_node(row: Row) -> Node {
    let content_str: String = row.get(2);
    Node {
        id: row.get(0),
        name: row.get(1),
        content: sql_to_json(&content_str).unwrap_or(serde_json::Value::Null),
    }
}

fn row_to_pipeline(row: Row) -> Pipeline {
    let content_str: String = row.get(2);
    Pipeline {
        id: row.get(0),
        name: row.get(1),
        content: sql_to_json(&content_str).unwrap_or(serde_json::Value::Null),
        method: row.get(3),
        url: row.get(4),
    }
} 
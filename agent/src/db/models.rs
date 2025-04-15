use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::{Type, IsNull, to_sql_checked, FromSql, ToSql};
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
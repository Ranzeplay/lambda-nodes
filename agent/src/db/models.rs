use anyhow::Result;
use chrono::{DateTime, Utc};
use postgres_types::private::BytesMut;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl<'a> ToSql for LogLevel {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
            .to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::VARCHAR || ty.name() == "log_level"
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
        ty == &Type::VARCHAR || ty == &Type::TEXT || ty.name() == "log_level"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub id: i32,
    pub level: LogLevel,
    pub category: String,
    pub message: String,
    pub create_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: Uuid,
    pub is_internal: bool,
    pub name: String,
    pub script: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

pub type HttpMethod = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub content: serde_json::Value,
    pub method: HttpMethod,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub id: Uuid,
    pub pipeline_id: Uuid,
    pub status: String,
    pub start_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub result: Option<serde_json::Value>,
}

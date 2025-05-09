pub(crate) mod blocks;
mod db;
mod executor;
mod routes;
mod middlewares;

use crate::middlewares::db_logging::DbLogger;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use log::{error, info};
use std::env;
use std::sync::Arc;
use tokio_postgres::{Config, NoTls};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Database connection setup
    let mut cfg = Config::new();
    cfg.host(&env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()))
        .port(
            env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()?,
        )
        .dbname(&env::var("DB_NAME").unwrap_or_else(|_| "lambda-nodes".to_string()))
        .user(&env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()))
        .password(&env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string()));

    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or(env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string())),
    );
    info!("Initialized logger");

    let client = cfg.connect(NoTls).await;
    let (client, connection) = match client {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to connect to the database: {}", e);
            return Err(anyhow::anyhow!("Database connection error"));
        }
    };

    // Spawn the connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("database connection error: {}", e);
        }
    });
    let client = Arc::new(client);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(DbLogger {
                client: client.clone(),
            })
            .app_data(web::Data::new(client.clone()))
            .configure(routes::configure)
    })
        // .workers(1)
        .bind(("127.0.0.1", 3000))?
        .run()
        .await?;

    Ok(())
}


mod routes;
mod db;

use actix_web::{web, App, HttpServer};
use tokio_postgres::{NoTls, Config};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use actix_web::middleware::Logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Database connection setup
    let mut cfg = Config::new();
    cfg.host(&env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()))
        .port(env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse()?)
        .dbname(&env::var("DB_NAME").unwrap_or_else(|_| "lambda-nodes".to_string()))
        .user(&env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()))
        .password(&env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string()));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (client, connection) = cfg.connect(NoTls).await?;
    
    // Spawn the connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("database connection error: {}", e);
        }
    });

    let client = Arc::new(client);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await?;

    Ok(())
}

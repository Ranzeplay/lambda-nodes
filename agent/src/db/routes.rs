use crate::db::models::Route;
use crate::db::utils::row_to_route;
use anyhow::Result;
use tokio_postgres::Client;
use uuid::Uuid;

pub async fn create_route(client: &Client, pipeline_id: Uuid, path: &str, method: &str) -> Result<Route> {
    // check if there is already a route with the same path and method
    let existing_route = client
        .query_opt(
            "SELECT id, pipeline_id, path, method FROM routes WHERE path = $1 AND method = $2",
            &[&path, &method],
        )
        .await?;
    if existing_route.is_some() {
        return Err(anyhow::anyhow!("Route with path {} and method {} already exists", path, method));
    }
    
    let row = client
        .query_one(
            "INSERT INTO routes (pipeline_id, path, method) VALUES ($1, $2, $3) RETURNING id, pipeline_id, path, method",
            &[&pipeline_id, &path, &method],
        )
        .await?;
    Ok(row_to_route(row))
}

pub async fn get_route(client: &Client, id: Uuid) -> Result<Option<Route>> {
    let row = client
        .query_opt(
            "SELECT id, pipeline_id, path, method FROM routes WHERE id = $1",
            &[&id],
        )
        .await?;
    Ok(row.map(row_to_route))
}

pub async fn list_routes(client: &Client, limit: i64, offset: i64) -> Result<Vec<Route>> {
    let rows = client
        .query(
            "SELECT id, pipeline_id, path, method FROM routes ORDER BY path LIMIT $1 OFFSET $2",
            &[&limit, &offset],
        )
        .await?;
    Ok(rows.into_iter().map(row_to_route).collect())
}

pub async fn delete_route(client: &Client, id: Uuid) -> Result<bool> {
    let rows_affected = client
        .execute("DELETE FROM routes WHERE id = $1", &[&id])
        .await?;
    Ok(rows_affected > 0)
}

pub async fn count_routes(client: &Client) -> Result<i64> {
    let row = client.query_one("SELECT COUNT(*) FROM routes", &[]).await?;
    Ok(row.get(0))
}

pub async fn get_routes_by_pipeline_id(client: &Client, pipeline_id: Uuid) -> Result<Vec<Route>> {
    let rows = client
        .query(
            "SELECT id, pipeline_id, path, method FROM routes WHERE pipeline_id = $1",
            &[&pipeline_id],
        )
        .await?;
    Ok(rows.into_iter().map(row_to_route).collect())
}

pub async fn update_route(client: &Client, id: Uuid, path: &str, method: &str) -> Result<Route> {
    // check if there is already a route with the same path and method
    let existing_route = client
        .query_opt(
            "SELECT id, pipeline_id, path, method FROM routes WHERE path = $1 AND method = $2",
            &[&path, &method],
        )
        .await?;
    if existing_route.is_some() {
        return Err(anyhow::anyhow!("Route with path {} and method {} already exists", path, method));
    }
    
    let row = client
        .query_one(
            "UPDATE routes SET path = $2, method = $3 WHERE id = $1 RETURNING id, pipeline_id, path, method",
            &[&id, &path, &method],
        )
        .await?;
    Ok(row_to_route(row))
}

pub async fn find_route(client: &Client, path: String, method: String) -> Result<Route> {
    let row = client
        .query_one(
            "SELECT id, pipeline_id, path, method FROM routes WHERE path = $1 AND method = $2",
            &[&path, &method],
        )
        .await?;
    Ok(row_to_route(row))
}
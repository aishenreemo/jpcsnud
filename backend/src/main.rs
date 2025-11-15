use anyhow::Result;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde_json::json;
use serde_json::Value;
use sqlx::PgPool;

use crate::error::ServerError;

mod error;

fn create_app() -> Router {
    Router::new()
        .route("/checkhealth", get(checkhealth))
        .route("/auth/register", get(register))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")?;

    #[allow(unused)]
    let pool = PgPool::connect(&database_url).await?;
    println!("Connected to PostgreSQL!");

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = create_app();
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    Ok(axum::serve(listener, app).await?)
}

async fn checkhealth() -> Json<Value> {
    Json(json!({
        "message": "Server is Running",
    }))
}

async fn register() -> ServerError {
    ServerError::InternalServerError
}

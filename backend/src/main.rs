use std::sync::Arc;

use anyhow::Result;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use serde_json::json;
use serde_json::Value;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;

mod auth;
mod email;
mod error;

use email::EmailService;

pub struct App {
    db: Pool<Postgres>,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    email_service: Option<EmailService>,
}

impl App {
    pub fn new(db: Pool<Postgres>, jwt_secret: String, email_service: Option<EmailService>) -> Arc<Self> {
        let bytes = jwt_secret.as_bytes();
        let encoding_key = EncodingKey::from_secret(bytes);
        let decoding_key = DecodingKey::from_secret(bytes);
        Arc::new(Self {
            db,
            encoding_key,
            decoding_key,
            email_service,
        })
    }

    pub fn new_router(app: Arc<App>) -> Router {
        Router::new()
            .route("/checkhealth", get(checkhealth))
            .route("/auth/register", post(auth::register))
            .route("/auth/authorize", post(auth::authorize))
            .route("/auth/request_code", post(auth::request_code))
            .route("/auth/verify_code", post(auth::verify_code))
            .route("/auth/protected", post(auth::protected))
            .route("/auth/prune", post(auth::prune))
            .with_state(app)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let port = std::env::var("PORT")?;
    let addr = format!("0.0.0.0:{}", port);

    #[allow(unused)]
    let pool = PgPool::connect(&database_url).await?;
    println!("Connected to PostgreSQL!");

    let envs = ["SMTP_HOST", "SMTP_USERNAME", "SMTP_PASSWORD", "FROM_EMAIL"];
    let email_service = envs.iter().all(|var| std::env::var(var).is_ok()).then(|| {
        EmailService::new(
            std::env::var("SMTP_HOST").unwrap(),
            std::env::var("SMTP_USERNAME").unwrap(),
            std::env::var("SMTP_PASSWORD").unwrap(),
            std::env::var("FROM_EMAIL").unwrap(),
        )
    });

    let app = App::new(pool.clone(), jwt_secret, email_service);
    let router = App::new_router(app);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    Ok(axum::serve(listener, router).await?)
}

async fn checkhealth() -> Json<Value> {
    Json(json!({
        "message": "Server is Running",
    }))
}

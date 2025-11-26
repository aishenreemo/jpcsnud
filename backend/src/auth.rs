use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::Json;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

use crate::error::ServerError;
use crate::App;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self {
        Claims { sub, exp }
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync + AsRef<App>,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, s: &S) -> Result<Self, Self::Rejection> {
        let decoding_key = &s.as_ref().decoding_key;
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ServerError::BadRequest("Invalid Token".to_owned()))?;

        let token_data = decode(bearer.token(), decoding_key, &Validation::default())
            .map_err(|_| ServerError::BadRequest("Invalid Token".to_owned()))?;

        Ok(token_data.claims)
    }
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    email: String,
    name: String,
}

pub async fn register(
    State(app): State<Arc<App>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<Value>, ServerError> {
    let record = sqlx::query!(
        r#"
            INSERT INTO "Users" (email, name)
            VALUES ($1, $2)
            RETURNING user_id;
        "#,
        payload.email,
        payload.name
    )
    .fetch_one(&app.db)
    .await
    .map_err(|_| ServerError::Conflict)?;

    println!("{record:?}");

    Ok(Json(json!({
        "user_id": record.user_id
    })))
}

#[derive(Deserialize)]
pub struct AuthPayload {
    email: String,
}

pub async fn authorize(
    State(app): State<Arc<App>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<serde_json::Value>, ServerError> {
    let user = sqlx::query!(
        r#"SELECT user_id FROM "Users" WHERE email = $1"#,
        payload.email
    )
    .fetch_one(&app.db)
    .await
    .map_err(|_| ServerError::NotFound)?;

    let claims = Claims::new(user.user_id.to_string(), 5 * 60);
    let token = encode(&Header::default(), &claims, &app.encoding_key)
        .map_err(|_| ServerError::InternalServerError)?;

    Ok(Json(json!({
        "access_token": token,
        "token_type": "Bearer".to_owned(),
    })))
}

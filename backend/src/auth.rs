use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::Json;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::Header;
use jsonwebtoken::TokenData;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::error::ServerError;
use crate::App;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Claims { sub, exp: now + exp }
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


        let token_data: TokenData<Claims> = decode(bearer.token(), decoding_key, &Validation::default())
            .map_err(|_| ServerError::BadRequest("Invalid Token".to_owned()))?;

        let user_exists = sqlx::query_scalar!(
            r#"SELECT EXISTS (SELECT 1 FROM "Users" WHERE user_id = $1)"#,
            token_data.claims.sub.parse::<i32>().unwrap()
        )
            .fetch_one(&s.as_ref().db)
            .await
            .map_err(|_| ServerError::InternalServerError)?;

        if !user_exists.unwrap_or(false) {
            return Err(ServerError::Unauthorized);
        }

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

pub async fn protected(
    claims: Claims,
) -> Result<Json<Value>, ServerError> {
    Ok(Json(json!({
        "message": "You have accessed a protected route",
        "user_id": claims.sub,
    })))
}

pub async fn prune(
    State(app): State<Arc<App>>,
    claims: Claims,
) -> Result<Json<Value>, ServerError> {
    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| ServerError::BadRequest("Invalid user id".to_owned()))?;

    sqlx::query!(
        r#"
        DELETE FROM "Users"
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(&app.db)
    .await
    .map_err(|_| ServerError::InternalServerError)?;

    Ok(Json(json!({
        "message": "User deleted successfully"
    })))
}

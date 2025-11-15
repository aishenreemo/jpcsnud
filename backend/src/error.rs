use std::fmt::Debug;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;

#[allow(unused)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ServerError {
    NotFound,
    BadRequest(String),
    InternalServerError,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = match &self {
            ServerError::NotFound => StatusCode::NOT_FOUND,
            ServerError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServerError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let msg = format!("{self:?}");
        let body = Json(json!({
            "error": msg,
        }));

        (status, body).into_response()
    }
}

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use tracing::error;

use super::{service, state::AppState};

pub async fn live() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

pub async fn ready(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pool = &state.pool;

    match service::test_connection(pool).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            error!("database connection failed: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}

pub async fn get_products(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let pool = &state.pool;

    match service::get_products(pool).await {
        Ok(products) => Ok(Json(products)),
        Err(e) => {
            error!("failed to get products: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "failed to get products" })),
            ))
        }
    }
}

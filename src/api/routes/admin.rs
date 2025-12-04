use axum::{
    Extension,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;
use crate::database::connection::DatabaseConnectionPool;

pub async fn get_status() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "running"})))
}

pub async fn list_users(
    Extension(_db): Extension<DatabaseConnectionPool>,
) -> impl IntoResponse {
    // TODO: Implement user listing
    (StatusCode::OK, Json(json!({"users": []})))
}

pub async fn list_trades(
    Extension(_db): Extension<DatabaseConnectionPool>,
) -> impl IntoResponse {
    // TODO: Implement trades listing
    (StatusCode::OK, Json(json!({"trades": []})))
}

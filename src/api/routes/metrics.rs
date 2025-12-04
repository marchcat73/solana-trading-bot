use axum::{
    Extension,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use crate::monitoring::metrics::MetricsRegistry;

pub async fn get_metrics(
    Extension(metrics): Extension<Arc<MetricsRegistry>>,
) -> impl IntoResponse {
    match metrics.get_metrics() {
        Ok(metrics_text) => (
            StatusCode::OK,
            [("Content-Type", "text/plain; version=0.0.4")],
            metrics_text,
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("Content-Type", "text/plain")],
            format!("Failed to get metrics: {}", e),
        ),
    }
}

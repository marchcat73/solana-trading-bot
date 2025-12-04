use axum::{
    Router,
    routing::{get, post},
    Extension,
};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
};
use std::time::Duration;
use std::sync::Arc;

use crate::{
    config::settings::{ApiSettings},
    database::connection::DatabaseConnectionPool,
    monitoring::metrics::MetricsRegistry,
    security::secrets_manager::SecretsManager,
    api::routes,
};

pub struct ApiServer {
    settings: ApiSettings,
    database: DatabaseConnectionPool,
    metrics: MetricsRegistry,
    secrets: SecretsManager,
}

impl ApiServer {
    pub fn new(
        settings: ApiSettings,
        database: DatabaseConnectionPool,
        metrics: MetricsRegistry,
        secrets: SecretsManager,
    ) -> Self {
        Self {
            settings,
            database,
            metrics,
            secrets,
        }
    }

    pub async fn start(self) -> Result<(), anyhow::Error> {
        let app = self.create_router().await?;

        let addr = format!(
            "{}:{}",
            self.settings.host,
            self.settings.port
        );

        log::info!("Starting API server on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;

        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await?;

        Ok(())
    }

    async fn create_router(&self) -> Result<Router, anyhow::Error> {
        // Создаем основные роуты
        let api_routes = Router::new()
            .route("/health", get(routes::health::health_check))
            .route("/metrics", get(routes::metrics::get_metrics))
            .route("/admin/status", get(routes::admin::get_status))
            .route("/admin/users", get(routes::admin::list_users))
            .route("/admin/trades", get(routes::admin::list_trades))
            .layer(Extension(self.database.clone()))
            .layer(Extension(self.metrics.clone()))
            .layer(Extension(self.secrets.clone()));

        // Создаем основной роутер с middleware
        let app = Router::new()
            .nest("/api/v1", api_routes)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &axum::http::Request<_>| {
                        // Можно использовать существующий span или создать новый
                        let span = tracing::span!(
                            tracing::Level::INFO,
                            "http_request",
                            method = %request.method(),
                            uri = %request.uri(),
                            path = request.uri().path(),
                            query = request.uri().query(),
                        );
                        span
                    })
                    .on_request(|request: &axum::http::Request<_>, span: &tracing::Span| {
                        span.record("start_time", format!("{:?}", std::time::SystemTime::now()));
                        tracing::info!("Starting request: {} {}", request.method(), request.uri());
                    })
                    .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, span: &tracing::Span| {
                        span.record("latency_ms", format!("{}", latency.as_millis()));
                        span.record("status", response.status().as_u16());
                        tracing::info!("Response: {} ({}ms)", response.status(), latency.as_millis());
                    })
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(tower_http::cors::Any)
                    .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                    .allow_headers(tower_http::cors::Any)
            )
            .layer(CompressionLayer::new())
            .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB limit
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                axum::http::header::AUTHORIZATION,
            )));

        Ok(app)
    }

    async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            log::info!("Received CTRL+C signal");
        }
        _ = terminate => {
            log::info!("Received terminate signal");
        }
    }

    log::info!("Starting graceful shutdown...");
}
}

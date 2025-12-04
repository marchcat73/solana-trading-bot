use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod security;
mod telegram;
mod api;
mod monitoring;
mod utils;

use crate::config::Settings;
use crate::database::connection::DatabaseConnectionPool;
use crate::security::secrets_manager::SecretsManager;
use crate::telegram::bot::TelegramBot;
use crate::api::server::ApiServer;
use crate::monitoring::metrics::MetricsRegistry;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing
    init_tracing();

    info!("Starting Solana Trading Bot...");

    // Load configuration
    let settings = Settings::new()?;
    info!("Configuration loaded successfully");

    // Initialize secrets manager
    let secrets_manager = SecretsManager::new(&settings).await?;
    info!("Secrets manager initialized");

    // Initialize metrics registry
    let metrics = MetricsRegistry::new();
    info!("Metrics registry initialized");

    // Connect to database
    let database = DatabaseConnectionPool::connect(&settings.database).await?;
    info!("Database connected successfully");

    // Run migrations
    database.run_migrations().await?;
    info!("Database migrations completed");

    // Initialize API server
    let api_server = ApiServer::new(
        settings.api.clone(),
        database.clone(),
        metrics.clone(),
        secrets_manager.clone(),
    );

    // Initialize Telegram bot
    let telegram_bot = TelegramBot::new(
        settings.telegram.clone(),
        database.clone(),
        secrets_manager.clone(),
        metrics.clone(),
    ).await?;

    // Run services concurrently
    tokio::select! {
        result = api_server.start() => {
            if let Err(e) = result {
                error!("API server failed: {}", e);
                return Err(e.into());
            }
        }
        result = telegram_bot.start() => {
            if let Err(e) = result {
                error!("Telegram bot failed: {}", e);
                return Err(e.into());
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal, shutting down gracefully...");
        }
    }

    info!("Solana Trading Bot shutdown complete");
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
}

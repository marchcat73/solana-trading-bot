use sea_orm::{Database as SeaDatabase, DatabaseConnection, DbErr, ConnectOptions, TransactionTrait};
use tracing::info;
use std::time::Duration;
use secrecy::ExposeSecret;

use crate::config::settings::DatabaseSettings;

#[derive(Clone)]
pub struct DatabaseConnectionPool {
    connection: DatabaseConnection,
}

impl DatabaseConnectionPool {
    pub async fn connect(settings: &DatabaseSettings) -> Result<Self, DbErr> {
        info!("Connecting to database...");

        let mut options = ConnectOptions::new(settings.url.expose_secret());

        options
            .max_connections(settings.pool_max_connections)
            .min_connections(settings.pool_min_connections)
            .connect_timeout(Duration::from_secs(settings.connect_timeout_secs))
            .acquire_timeout(Duration::from_secs(settings.acquire_timeout_secs))
            .idle_timeout(Duration::from_secs(settings.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(settings.max_lifetime_secs))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let connection = SeaDatabase::connect(options).await?;

        info!("Database connection established successfully");

        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn run_migrations(&self) -> Result<(), DbErr> {
        // Для SeaORM миграции нужно использовать sea-orm-cli
        // Если миграции не нужны, можно оставить пустую реализацию
        info!("Skipping migrations (not implemented)");
        Ok(())
    }

    pub async fn health_check(&self) -> Result<(), DbErr> {
        self.connection.ping().await?;
        Ok(())
    }

    pub async fn transaction<F, T, E>(&self, callback: F) -> Result<T, E>
    where
        F: FnOnce(&DatabaseConnection) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>> + Send,
        T: Send,
        E: From<DbErr> + Send,
    {
        let transaction = self.connection.begin().await?;

        match callback(&self.connection).await {
            Ok(result) => {
                transaction.commit().await?;
                Ok(result)
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err)
            }
        }
    }
}

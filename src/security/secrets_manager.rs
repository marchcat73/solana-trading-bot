use crate::config::Settings;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::env;
use dotenvy::dotenv;

#[derive(Debug, Clone, Deserialize)]
pub struct BotSecrets {
    pub telegram_token: SecretString,
    pub jupiter_api_key: Option<SecretString>,
    pub master_encryption_key: SecretString,
    pub session_secret_key: SecretString,
}

#[derive(Debug, Clone)]
pub enum SecretsBackend {
    Environment,
    HashiCorpVault,
    EncryptedFile,
}

#[derive(Debug, Clone)]
pub struct SecretsManager {
    secrets: Arc<RwLock<BotSecrets>>,
    backend: SecretsBackend,
}

impl SecretsManager {
    pub async fn new(settings: &Settings) -> Result<Self> {
        let backend = if settings.is_production() {
            // В production используем AWS Secrets Manager или Vault
            SecretsBackend::EncryptedFile
        } else {
            // В development используем .env файлы
            SecretsBackend::Environment
        };

        let secrets = match backend {
            SecretsBackend::Environment => Self::load_from_env(settings).await?,
            SecretsBackend::HashiCorpVault => Self::load_from_vault(settings).await?,
            SecretsBackend::EncryptedFile => Self::load_from_encrypted_file(settings).await?,
        };

        Ok(Self {
            secrets: Arc::new(RwLock::new(secrets)),
            backend,
        })
    }

    async fn load_from_env(settings: &Settings) -> Result<BotSecrets> {
        dotenv().ok();

        let telegram_token = env::var("TELEGRAM_BOT_TOKEN")
            .context("TELEGRAM_BOT_TOKEN must be set")?;

        let jupiter_api_key = env::var("JUPITER_API_KEY").ok();

        Ok(BotSecrets {
            telegram_token: SecretString::new(telegram_token.into_boxed_str()),
            jupiter_api_key: jupiter_api_key.map(|k| SecretString::new(k.into_boxed_str())),
            master_encryption_key: settings.security.master_encryption_key.clone(),
            session_secret_key: settings.security.session_secret_key.clone(),
        })
    }

    #[cfg(feature = "aws-secrets")]
    async fn load_from_aws(_settings: &Settings) -> Result<BotSecrets> {
        // Реализация для AWS Secrets Manager
        todo!("Implement AWS Secrets Manager integration")
    }

    async fn load_from_vault(_settings: &Settings) -> Result<BotSecrets> {
        // Реализация для HashiCorp Vault
        todo!("Implement Vault integration")
    }

    async fn load_from_encrypted_file(settings: &Settings) -> Result<BotSecrets> {
        // Для начала используем просто .env
        Self::load_from_env(settings).await
    }

    pub async fn get_telegram_token(&self) -> String {
        let secrets = self.secrets.read().await;
        secrets.telegram_token.expose_secret().to_string()
    }

    pub async fn get_jupiter_api_key(&self) -> Option<String> {
        let secrets = self.secrets.read().await;
        secrets.jupiter_api_key.as_ref()
            .map(|s| s.expose_secret().to_string())
    }

    pub async fn get_master_encryption_key(&self) -> SecretString {
        let secrets = self.secrets.read().await;
        secrets.master_encryption_key.clone()
    }

    async fn rotate_vault_secrets(&self) -> Result<()> {
    // TODO: Реализовать ротацию секретов для Vault
    log::warn!("Vault secrets rotation not yet implemented");
    Ok(())
}

    pub async fn rotate_secrets(&self) -> Result<()> {
        match self.backend {
            SecretsBackend::HashiCorpVault => self.rotate_vault_secrets().await,
            _ => Ok(()), // Для других бэкендов ротация не поддерживается
        }
    }

    #[cfg(feature = "aws-secrets")]
    async fn rotate_aws_secrets(&self) -> Result<()> {
        todo!("Implement AWS secrets rotation")
    }
}

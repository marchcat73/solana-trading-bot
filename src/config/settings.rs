use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use secrecy::{SecretString};
use std::env;
use dotenvy::dotenv;




#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: SecretString,
    #[serde(default = "default_pool_max_connections")]
    pub pool_max_connections: u32,
    #[serde(default = "default_pool_min_connections")]
    pub pool_min_connections: u32,
    #[serde(default = "default_connect_timeout_secs")]
    pub connect_timeout_secs: u64,
    #[serde(default = "default_acquire_timeout_secs")]
    pub acquire_timeout_secs: u64,
    #[serde(default = "default_idle_timeout_secs")]
    pub idle_timeout_secs: u64,
    #[serde(default = "default_max_lifetime_secs")]
    pub max_lifetime_secs: u64,
}

pub fn default_pool_max_connections() -> u32 { 20 }
pub fn default_pool_min_connections() -> u32 { 5 }
pub fn default_connect_timeout_secs() -> u64 { 30 }
pub fn default_acquire_timeout_secs() -> u64 { 30 }
pub fn default_idle_timeout_secs() -> u64 { 600 }
pub fn default_max_lifetime_secs() -> u64 { 1800 }

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramSettings {
    pub bot_token: SecretString,
    #[serde(default)]
    pub webhook_url: Option<String>,
    #[serde(default = "default_admin_user_ids")]
    pub admin_user_ids: Vec<i64>,
    #[serde(default = "default_rate_limit_per_minute")]
    pub rate_limit_per_minute: u32,
    #[serde(default = "default_rate_limit_per_hour")]
    pub rate_limit_per_hour: u32,
}

pub fn default_admin_user_ids() -> Vec<i64> { vec![] }
pub fn default_rate_limit_per_minute() -> u32 { 30 }
pub fn default_rate_limit_per_hour() -> u32 { 300 }

#[derive(Debug, Deserialize, Clone)]
pub struct SolanaSettings {
    #[serde(default = "default_solana_rpc_url")]
    pub rpc_url: String,
    #[serde(default = "default_solana_ws_url")]
    pub ws_url: String,
    #[serde(default = "default_commitment")]
    pub commitment: String,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default = "default_retry_count")]
    pub retry_count: u32,
}

pub fn default_solana_rpc_url() -> String { "https://api.mainnet-beta.solana.com".to_string() }
pub fn default_solana_ws_url() -> String { "wss://api.mainnet-beta.solana.com".to_string() }
pub fn default_commitment() -> String { "confirmed".to_string() }
pub fn default_timeout_ms() -> u64 { 30000 }
pub fn default_retry_count() -> u32 { 3 }

#[derive(Debug, Deserialize, Clone)]
pub struct JupiterSettings {
    #[serde(default = "default_jupiter_api_url")]
    pub api_url: String,
    #[serde(default = "default_jupiter_api_version")]
    pub api_version: String,
    #[serde(default = "default_jupiter_timeout_secs")]
    pub timeout_secs: u64,
    #[serde(default = "default_jupiter_max_retries")]
    pub max_retries: u32,
    #[serde(default)]
    pub api_key: Option<SecretString>,
}

pub fn default_jupiter_api_url() -> String { "https://api.jup.ag".to_string() }
pub fn default_jupiter_api_version() -> String { "v6".to_string() }
pub fn default_jupiter_timeout_secs() -> u64 { 30 }
pub fn default_jupiter_max_retries() -> u32 { 3 }


#[derive(Debug, Deserialize, Clone)]
pub struct SecuritySettings {
    #[serde(default = "default_master_encryption_key")]
    pub master_encryption_key: SecretString,
    #[serde(default = "default_session_secret_key")]
    pub session_secret_key: SecretString,
    #[serde(default = "default_encryption_algorithm")]
    pub encryption_algorithm: String,
    #[serde(default = "default_pbkdf2_iterations")]
    pub pbkdf2_iterations: u32,
    #[serde(default = "default_encrypted_keypair_path")]
    pub encrypted_keypair_path: String,
    #[serde(default = "default_encrypted_master_key_path")]
    pub encrypted_master_key_path: String,
}

pub fn default_master_encryption_key() -> SecretString {
    SecretString::new("change_this_to_a_strong_random_key_32_bytes".to_string().into_boxed_str())
}
pub fn default_session_secret_key() -> SecretString {
    SecretString::new("another_strong_random_key_for_sessions".to_string().into_boxed_str())
}
pub fn default_encryption_algorithm() -> String { "AES256-GCM".to_string() }
pub fn default_pbkdf2_iterations() -> u32 { 100000 }
pub fn default_encrypted_keypair_path() -> String { "./secrets/encrypted_wallet.bin".to_string() }
pub fn default_encrypted_master_key_path() -> String { "./secrets/master_key.bin".to_string() }

#[derive(Debug, Deserialize, Clone)]
pub struct TradingLimits {
    #[serde(default = "default_max_trade_amount_sol")]
    pub max_trade_amount_sol: f64,
    #[serde(default = "default_min_trade_amount_sol")]
    pub min_trade_amount_sol: f64,
    #[serde(default = "default_max_slippage_bps")]
    pub max_slippage_bps: u64,
    #[serde(default = "default_max_trades_per_hour")]
    pub max_trades_per_hour: u32,
    #[serde(default = "default_max_trades_per_day")]
    pub max_trades_per_day: u32,
    #[serde(default = "default_daily_trade_limit_sol")]
    pub daily_trade_limit_sol: f64,
}

pub fn default_max_trade_amount_sol() -> f64 { 10.0 }
pub fn default_min_trade_amount_sol() -> f64 { 0.01 }
pub fn default_max_slippage_bps() -> u64 { 200 }
pub fn default_max_trades_per_hour() -> u32 { 10 }
pub fn default_max_trades_per_day() -> u32 { 50 }
pub fn default_daily_trade_limit_sol() -> f64 { 100.0 }

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitSettings {
    #[serde(default = "default_requests_per_second")]
    pub requests_per_second: u32,
    #[serde(default = "default_burst_size")]
    pub burst_size: u32,
    #[serde(default = "default_rate_limit_enabled")]
    pub enabled: bool,
}

pub fn default_requests_per_second() -> u32 { 10 }
pub fn default_burst_size() -> u32 { 30 }
pub fn default_rate_limit_enabled() -> bool { true }

#[derive(Debug, Deserialize, Clone)]
pub struct ApiSettings {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_metrics_port")]
    pub metrics_port: u16,
    #[serde(default = "default_health_check_port")]
    pub health_check_port: u16,
}

pub fn default_host() -> String { "0.0.0.0".to_string() }
pub fn default_port() -> u16 { 8080 }
pub fn default_metrics_port() -> u16 { 9090 }
pub fn default_health_check_port() -> u16 { 8081 }

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringSettings {
    #[serde(default = "default_prometheus_enabled")]
    pub prometheus_enabled: bool,
    #[serde(default = "default_prometheus_port")]
    pub prometheus_port: u16,
    #[serde(default)]
    pub sentry_dsn: Option<String>,
    #[serde(default = "default_telemetry_enabled")]
    pub telemetry_enabled: bool,
}

pub fn default_prometheus_enabled() -> bool { true }
pub fn default_prometheus_port() -> u16 { 9090 }
pub fn default_telemetry_enabled() -> bool { false }

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default = "default_app_name")]
    pub app_name: String,
    #[serde(default = "default_app_version")]
    pub app_version: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default = "default_database")]
    pub database: DatabaseSettings,
    #[serde(default = "default_telegram")]
    pub telegram: TelegramSettings,
    #[serde(default = "default_solana")]
    pub solana: SolanaSettings,
    #[serde(default = "default_jupiter")]
    pub jupiter: JupiterSettings,
    #[serde(default = "default_security")]
    pub security: SecuritySettings,
    #[serde(default = "default_trading_limits")]
    pub trading_limits: TradingLimits,
    #[serde(default = "default_rate_limit")]
    pub rate_limit: RateLimitSettings,
    #[serde(default = "default_api")]
    pub api: ApiSettings,
    #[serde(default = "default_monitoring")]
    pub monitoring: MonitoringSettings,
}

pub fn default_env() -> String { "development".to_string() }
pub fn default_app_name() -> String { "solana-trading-bot".to_string() }
pub fn default_app_version() -> String { "0.1.0".to_string() }
pub fn default_log_level() -> String { "info".to_string() }

pub fn default_database() -> DatabaseSettings {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    DatabaseSettings {
        url: SecretString::new(database_url.to_string().into_boxed_str()),
        pool_max_connections: default_pool_max_connections(),
        pool_min_connections: default_pool_min_connections(),
        connect_timeout_secs: default_connect_timeout_secs(),
        acquire_timeout_secs: default_acquire_timeout_secs(),
        idle_timeout_secs: default_idle_timeout_secs(),
        max_lifetime_secs: default_max_lifetime_secs(),
    }
}

pub fn default_telegram() -> TelegramSettings {
    TelegramSettings {
        bot_token: SecretString::new("".to_string().into_boxed_str()),
        webhook_url: None,
        admin_user_ids: default_admin_user_ids(),
        rate_limit_per_minute: default_rate_limit_per_minute(),
        rate_limit_per_hour: default_rate_limit_per_hour(),
    }
}

pub fn default_solana() -> SolanaSettings {
    SolanaSettings {
        rpc_url: default_solana_rpc_url(),
        ws_url: default_solana_ws_url(),
        commitment: default_commitment(),
        timeout_ms: default_timeout_ms(),
        retry_count: default_retry_count(),
    }
}

pub fn default_jupiter() -> JupiterSettings {
    JupiterSettings {
        api_url: default_jupiter_api_url(),
        api_version: default_jupiter_api_version(),
        timeout_secs: default_jupiter_timeout_secs(),
        max_retries: default_jupiter_max_retries(),
        api_key: None,
    }
}

pub fn default_security() -> SecuritySettings {
    SecuritySettings {
        master_encryption_key: default_master_encryption_key(),
        session_secret_key: default_session_secret_key(),
        encryption_algorithm: default_encryption_algorithm(),
        pbkdf2_iterations: default_pbkdf2_iterations(),
        encrypted_keypair_path: default_encrypted_keypair_path(),
        encrypted_master_key_path: default_encrypted_master_key_path(),
    }
}

pub fn default_trading_limits() -> TradingLimits {
    TradingLimits {
        max_trade_amount_sol: default_max_trade_amount_sol(),
        min_trade_amount_sol: default_min_trade_amount_sol(),
        max_slippage_bps: default_max_slippage_bps(),
        max_trades_per_hour: default_max_trades_per_hour(),
        max_trades_per_day: default_max_trades_per_day(),
        daily_trade_limit_sol: default_daily_trade_limit_sol(),
    }
}

pub fn default_rate_limit() -> RateLimitSettings {
    RateLimitSettings {
        requests_per_second: default_requests_per_second(),
        burst_size: default_burst_size(),
        enabled: default_rate_limit_enabled(),
    }
}

pub fn default_api() -> ApiSettings {
    ApiSettings {
        host: default_host(),
        port: default_port(),
        metrics_port: default_metrics_port(),
        health_check_port: default_health_check_port(),
    }
}

pub fn default_monitoring() -> MonitoringSettings {
    MonitoringSettings {
        prometheus_enabled: default_prometheus_enabled(),
        prometheus_port: default_prometheus_port(),
        sentry_dsn: None,
        telemetry_enabled: default_telemetry_enabled(),
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let run_mode = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let config_builder = Config::builder()
            .set_default("env", run_mode.clone())?
            .add_source(
                File::with_name(&format!("config/{}", run_mode))
                    .required(false)
            )
            .add_source(
                File::with_name("config/local")
                    .required(false)
            )
            .add_source(
                Environment::with_prefix("SOLBOT")
                    .separator("_")
                    .list_separator(",")
                    .try_parsing(true)
            );

        let settings = config_builder.build()?;

        settings.try_deserialize()
    }

    pub fn is_production(&self) -> bool {
        self.env == "production"
    }

    pub fn is_development(&self) -> bool {
        self.env == "development"
    }
}

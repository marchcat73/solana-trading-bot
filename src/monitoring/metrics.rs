use std::sync::Arc;
use prometheus::{Registry, Counter, Histogram, Encoder, TextEncoder};
use anyhow::Result;

#[derive(Clone)]
pub struct MetricsRegistry {
    registry: Arc<Registry>,

    // Trading metrics
    pub trades_total: Counter,
    pub trades_success: Counter,
    pub trades_failed: Counter,
    pub trade_amount: Histogram,
    pub trade_duration: Histogram,

    // API metrics
    pub api_requests_total: Counter,
    pub api_request_duration: Histogram,
    pub api_errors_total: Counter,

    // Telegram metrics
    pub telegram_messages_total: Counter,
    pub telegram_commands_total: Counter,

    // Solana metrics
    pub solana_rpc_calls_total: Counter,
    pub solana_rpc_duration: Histogram,
    pub solana_transactions_total: Counter,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        let registry = Registry::new();

        // Trading metrics
        let trades_total = Counter::new(
            "trades_total",
            "Total number of trades executed"
        ).unwrap();

        let trades_success = Counter::new(
            "trades_success",
            "Number of successful trades"
        ).unwrap();

        let trades_failed = Counter::new(
            "trades_failed",
            "Number of failed trades"
        ).unwrap();

        let trade_amount = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "trade_amount_sol",
                "Amount of SOL per trade"
            ).buckets(vec![0.01, 0.1, 0.5, 1.0, 5.0, 10.0, 50.0, 100.0])
        ).unwrap();

        let trade_duration = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "trade_duration_seconds",
                "Duration of trade execution in seconds"
            ).buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0])
        ).unwrap();

        // API metrics
        let api_requests_total = Counter::new(
            "api_requests_total",
            "Total number of API requests"
        ).unwrap();

        let api_request_duration = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "api_request_duration_seconds",
                "Duration of API requests in seconds"
            ).buckets(vec![0.01, 0.05, 0.1, 0.5, 1.0, 2.0, 5.0])
        ).unwrap();

        let api_errors_total = Counter::new(
            "api_errors_total",
            "Total number of API errors"
        ).unwrap();

        // Telegram metrics
        let telegram_messages_total = Counter::new(
            "telegram_messages_total",
            "Total number of Telegram messages processed"
        ).unwrap();

        let telegram_commands_total = Counter::new(
            "telegram_commands_total",
            "Total number of Telegram commands processed"
        ).unwrap();

        // Solana metrics
        let solana_rpc_calls_total = Counter::new(
            "solana_rpc_calls_total",
            "Total number of Solana RPC calls"
        ).unwrap();

        let solana_rpc_duration = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "solana_rpc_duration_seconds",
                "Duration of Solana RPC calls in seconds"
            ).buckets(vec![0.01, 0.05, 0.1, 0.5, 1.0, 2.0, 5.0])
        ).unwrap();

        let solana_transactions_total = Counter::new(
            "solana_transactions_total",
            "Total number of Solana transactions"
        ).unwrap();

        // Register all metrics
        registry.register(Box::new(trades_total.clone())).unwrap();
        registry.register(Box::new(trades_success.clone())).unwrap();
        registry.register(Box::new(trades_failed.clone())).unwrap();
        registry.register(Box::new(trade_amount.clone())).unwrap();
        registry.register(Box::new(trade_duration.clone())).unwrap();

        registry.register(Box::new(api_requests_total.clone())).unwrap();
        registry.register(Box::new(api_request_duration.clone())).unwrap();
        registry.register(Box::new(api_errors_total.clone())).unwrap();

        registry.register(Box::new(telegram_messages_total.clone())).unwrap();
        registry.register(Box::new(telegram_commands_total.clone())).unwrap();

        registry.register(Box::new(solana_rpc_calls_total.clone())).unwrap();
        registry.register(Box::new(solana_rpc_duration.clone())).unwrap();
        registry.register(Box::new(solana_transactions_total.clone())).unwrap();

        Self {
            registry: Arc::new(registry),
            trades_total,
            trades_success,
            trades_failed,
            trade_amount,
            trade_duration,
            api_requests_total,
            api_request_duration,
            api_errors_total,
            telegram_messages_total,
            telegram_commands_total,
            solana_rpc_calls_total,
            solana_rpc_duration,
            solana_transactions_total,
        }
    }

    pub fn get_registry(&self) -> Arc<Registry> {
        self.registry.clone()
    }

    pub fn get_metrics(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

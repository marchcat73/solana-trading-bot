use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use tracing::{info, warn, error};
use url::Url;

// Структуры данных для Jupiter API
#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteRequest {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: String, // в ламах (минимальных единицах)
    pub slippage_bps: Option<u16>,
    pub dexes: Option<Vec<String>>,
    pub exclude_dexes: Option<Vec<String>>,
    pub only_direct_routes: Option<bool>,
    pub as_legacy_transaction: Option<bool>,
    pub restrict_middleware_take: Option<bool>,
    pub platform_fee_bps: Option<u16>,
    pub max_accounts: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteResponse {
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub swap_mode: String,
    pub slippage_bps: u16,
    pub platform_fee: Option<PlatformFee>,
    pub price_impact_pct: String,
    pub route_plan: Vec<RoutePlan>,
    pub context_slot: u64,
    pub time_taken: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlatformFee {
    pub amount: String,
    pub fee_bps: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapInfo {
    pub amm_key: String,
    pub label: Option<String>,
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub fee_amount: String,
    pub fee_mint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapRequest {
    pub quote_response: QuoteResponse,
    pub user_public_key: String,
    pub wrap_and_unwrap_sol: Option<bool>,
    pub use_shared_accounts: Option<bool>,
    pub prioritization_fee_lamports: Option<u64>,
    pub dynamic_compute_unit_limit: Option<bool>,
    pub use_token_ledger: Option<bool>,
    pub skip_user_checks: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapResponse {
    pub swap_transaction: String, // Base64 encoded transaction
    pub last_valid_block_height: u64,
    pub prioritization_fee_lamports: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceResponse {
    pub data: std::collections::HashMap<String, TokenPrice>,
    pub time_taken: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenPrice {
    pub id: String,
    pub mint_symbol: String,
    pub vs_token: String,
    pub vs_token_symbol: String,
    pub price: f64,
}

pub struct JupiterClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

impl JupiterClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("solana-trading-bot/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Получение котировки для свопа
    pub async fn get_quote(&self, request: &QuoteRequest) -> Result<QuoteResponse> {
        let url = format!("{}/quote", self.base_url);

        info!("Requesting quote from Jupiter: {:?}", request);

        let response = self.client
            .get(&url)
            .query(&[
                ("inputMint", &request.input_mint),
                ("outputMint", &request.output_mint),
                ("amount", &request.amount),
                ("slippageBps", &request.slippage_bps.unwrap_or(50).to_string()),
                ("dexes", &request.dexes.as_ref().map(|d| d.join(",")).unwrap_or_default()),
                ("excludeDexes", &request.exclude_dexes.as_ref().map(|d| d.join(",")).unwrap_or_default()),
                ("onlyDirectRoutes", &request.only_direct_routes.unwrap_or(false).to_string()),
                ("asLegacyTransaction", &request.as_legacy_transaction.unwrap_or(false).to_string()),
                ("maxAccounts", &request.max_accounts.unwrap_or(64).to_string()),
            ])
            .timeout(self.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Jupiter API error ({}): {}", status, error_text);
            return Err(anyhow!("Jupiter API error: {} - {}", status, error_text));
        }

        let quote: QuoteResponse = response.json().await?;
        info!("Quote received: {} -> {}: {} -> {}",
            quote.input_mint, quote.output_mint, quote.in_amount, quote.out_amount);

        Ok(quote)
    }

    /// Получение транзакции для свопа
    pub async fn get_swap_transaction(&self, request: &SwapRequest) -> Result<SwapResponse> {
        let url = format!("{}/swap", self.base_url);

        info!("Requesting swap transaction from Jupiter for user: {}", request.user_public_key);

        let response = self.client
            .post(&url)
            .json(&json!({
                "quoteResponse": request.quote_response,
                "userPublicKey": request.user_public_key,
                "wrapAndUnwrapSol": request.wrap_and_unwrap_sol.unwrap_or(true),
                "useSharedAccounts": request.use_shared_accounts.unwrap_or(true),
                "prioritizationFeeLamports": request.prioritization_fee_lamports,
                "dynamicComputeUnitLimit": request.dynamic_compute_unit_limit.unwrap_or(true),
                "useTokenLedger": request.use_token_ledger.unwrap_or(false),
                "skipUserChecks": request.skip_user_checks.unwrap_or(false),
            }))
            .timeout(self.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Jupiter swap API error ({}): {}", status, error_text);
            return Err(anyhow!("Jupiter swap API error: {} - {}", status, error_text));
        }

        let swap_response: SwapResponse = response.json().await?;
        info!("Swap transaction received, last valid block height: {}",
            swap_response.last_valid_block_height);

        Ok(swap_response)
    }

    /// Получение цен токенов
    pub async fn get_price(&self, ids: &[String]) -> Result<PriceResponse> {
        let url = format!("{}/price", self.base_url);

        let ids_param = ids.join(",");
        info!("Requesting prices for tokens: {}", ids_param);

        let response = self.client
            .get(&url)
            .query(&[("ids", ids_param)])
            .timeout(self.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("Jupiter price API error ({}): {}", status, error_text);
            return Err(anyhow!("Jupiter price API error: {} - {}", status, error_text));
        }

        let price_response: PriceResponse = response.json().await?;
        Ok(price_response)
    }

    /// Получение списка поддерживаемых токенов
    pub async fn get_tokens(&self) -> Result<Vec<TokenInfo>> {
        let url = format!("{}/tokens", self.base_url);

        info!("Requesting token list from Jupiter");

        let response = self.client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("Jupiter tokens API error ({}): {}", status, error_text);
            return Err(anyhow!("Jupiter tokens API error: {} - {}", status, error_text));
        }

        let tokens: Vec<TokenInfo> = response.json().await?;
        info!("Received {} tokens from Jupiter", tokens.len());

        Ok(tokens)
    }

    /// Упрощенный метод для быстрого получения котировки
    pub async fn simple_quote(&self, input_mint: &str, output_mint: &str, amount: u64) -> Result<QuoteResponse> {
        let request = QuoteRequest {
            input_mint: input_mint.to_string(),
            output_mint: output_mint.to_string(),
            amount: amount.to_string(),
            slippage_bps: Some(50), // 0.5%
            dexes: None,
            exclude_dexes: None,
            only_direct_routes: Some(false),
            as_legacy_transaction: Some(false),
            restrict_middleware_take: Some(false),
            platform_fee_bps: None,
            max_accounts: Some(64),
        };

        self.get_quote(&request).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenInfo {
    pub address: String,
    pub chain_id: u64,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub logo_uri: Option<String>,
    pub tags: Option<Vec<String>>,
    pub extensions: Option<serde_json::Value>,
}

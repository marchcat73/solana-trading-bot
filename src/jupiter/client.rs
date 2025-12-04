use std::str::FromStr;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use anyhow::{Result, Context};
use secrecy::Secret;

#[derive(Debug, Clone)]
pub struct JupiterClient {
    client: Client,
    base_url: String,
    api_key: Option<Secret<String>>,
}

impl JupiterClient {
    pub fn new(base_url: &str, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.map(Secret::new),
        }
    }

    pub async fn get_quote_v6(&self, params: &QuoteParamsV6) -> Result<QuoteResponseV6> {
        let url = format!("{}/swap/v6/quote", self.base_url);

        let mut request = self.client.get(&url)
            .query(&[
                ("inputMint", &params.input_mint.to_string()),
                ("outputMint", &params.output_mint.to_string()),
                ("amount", &params.amount.to_string()),
                ("slippageBps", &params.slippage_bps.to_string()),
                ("onlyDirectRoutes", &params.only_direct_routes.to_string()),
                ("asLegacyTransaction", &params.as_legacy_transaction.to_string()),
                ("swapMode", &params.swap_mode.to_string()),
                ("maxAccounts", &params.max_accounts.to_string()),
            ]);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key.expose_secret()));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Jupiter API error: {}", error_text));
        }

        let quote: QuoteResponseV6 = response.json().await?;
        Ok(quote)
    }

    pub async fn get_swap_transaction_v6(&self, params: &SwapParamsV6) -> Result<SwapResponseV6> {
        let url = format!("{}/swap/v6/swap", self.base_url);

        let swap_request = SwapRequestV6 {
            quote_response: params.quote_response.clone(),
            user_public_key: params.user_public_key.to_string(),
            wrap_and_unwrap_sol: params.wrap_and_unwrap_sol,
            dynamic_compute_unit_limit: true,
            prioritization_fee_lamports: "auto".to_string(),
        };

        let mut request = self.client.post(&url)
            .json(&swap_request)
            .header("Content-Type", "application/json");

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key.expose_secret()));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Jupiter swap error: {}", error_text));
        }

        let swap_response: SwapResponseV6 = response.json().await?;
        Ok(swap_response)
    }

    pub async fn get_price(&self, params: &PriceParams) -> Result<PriceResponse> {
        let url = format!("{}/price/v2", self.base_url);

        let request = self.client.get(&url)
            .query(&[
                ("ids", &params.ids),
            ]);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Jupiter price error: {}", error_text));
        }

        let price_response: PriceResponse = response.json().await?;
        Ok(price_response)
    }

    pub async fn get_tokens(&self) -> Result<Vec<TokenInfo>> {
        let url = format!("{}/tokens/v2", self.base_url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Jupiter tokens error: {}", error_text));
        }

        let tokens: Vec<TokenInfo> = response.json().await?;
        Ok(tokens)
    }

    pub async fn search_tokens(&self, query: &str, limit: Option<u32>) -> Result<Vec<TokenInfo>> {
        let mut tokens = self.get_tokens().await?;

        // Фильтрация по запросу
        let query_lower = query.to_lowercase();
        tokens.retain(|token| {
            token.symbol.to_lowercase().contains(&query_lower) ||
            token.name.to_lowercase().contains(&query_lower) ||
            token.address.to_lowercase().contains(&query_lower)
        });

        // Сортировка по релевантности
        tokens.sort_by(|a, b| {
            let a_score = Self::calculate_relevance_score(a, &query_lower);
            let b_score = Self::calculate_relevance_score(b, &query_lower);
            b_score.cmp(&a_score)
        });

        // Ограничение количества результатов
        if let Some(limit) = limit {
            tokens.truncate(limit as usize);
        }

        Ok(tokens)
    }

    fn calculate_relevance_score(token: &TokenInfo, query: &str) -> u32 {
        let mut score = 0;

        if token.symbol.to_lowercase() == query {
            score += 100;
        }
        if token.symbol.to_lowercase().contains(query) {
            score += 50;
        }
        if token.name.to_lowercase().contains(query) {
            score += 30;
        }
        if token.address.to_lowercase().contains(query) {
            score += 20;
        }

        // Бонус за верифицированные токены
        if token.tags.contains(&"verified".to_string()) {
            score += 40;
        }

        score
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteParamsV6 {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub amount: u64, // в наименьших единицах (лампортах для SOL)
    pub slippage_bps: u64,
    pub only_direct_routes: bool,
    pub as_legacy_transaction: bool,
    pub swap_mode: String, // "ExactIn" или "ExactOut"
    pub max_accounts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponseV6 {
    pub input_mint: String,
    pub in_amount: String,
    pub output_mint: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub swap_mode: String,
    pub slippage_bps: u64,
    pub platform_fee: Option<PlatformFee>,
    pub price_impact_pct: Option<String>,
    pub context_slot: Option<u64>,
    pub time_taken: Option<f64>,
    pub route_plan: Vec<RoutePlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapParamsV6 {
    pub quote_response: QuoteResponseV6,
    pub user_public_key: Pubkey,
    pub wrap_and_unwrap_sol: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRequestV6 {
    pub quote_response: QuoteResponseV6,
    pub user_public_key: String,
    pub wrap_and_unwrap_sol: bool,
    pub dynamic_compute_unit_limit: bool,
    pub prioritization_fee_lamports: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResponseV6 {
    pub swap_transaction: String, // base64 encoded transaction
    pub last_valid_block_height: u64,
    pub prioritization_fee_lamports: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceParams {
    pub ids: String, // comma separated token addresses
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceResponse {
    pub data: std::collections::HashMap<String, TokenPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPrice {
    pub id: String,
    #[serde(rename = "type")]
    pub token_type: String,
    pub price: f64,
    pub price_change_24h: Option<f64>,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub chain_id: u64,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    pub tags: Vec<String>,
    pub extensions: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformFee {
    pub amount: String,
    pub fee_bps: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

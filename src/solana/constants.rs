// Основные токены на Solana
pub const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const USDT_MINT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
pub const BONK_MINT: &str = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";
pub const RAY_MINT: &str = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R";
pub const ORCA_MINT: &str = "orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE";

// Декimals для токенов
pub const SOL_DECIMALS: u8 = 9;
pub const USDC_DECIMALS: u8 = 6;
pub const USDT_DECIMALS: u8 = 6;
pub const BONK_DECIMALS: u8 = 5;
pub const RAY_DECIMALS: u8 = 6;
pub const ORCA_DECIMALS: u8 = 6;

// Конвертация суммы в лампорты/минимальные единицы
pub fn to_lamports(amount: f64, decimals: u8) -> u64 {
    (amount * 10_f64.powi(decimals as i32)) as u64
}

// Конвертация из лампортов/минимальных единиц
pub fn from_lamports(lamports: u64, decimals: u8) -> f64 {
    lamports as f64 / 10_f64.powi(decimals as i32)
}


use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub rpc_urls: Vec<String>,
    pub wallet_path: String,
    pub wallet_address: String,
    pub slippage_bps: u32,
    pub max_retries: u32,
    pub budget_sol: f64,
    pub buy_cap_sol: f64,
    pub export_dir: String,
    pub report_prefix: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".into());
        let rpc_urls = env::var("RPC_URLS")
            .unwrap_or_else(|_| rpc_url.clone())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        Ok(Self {
            rpc_url,
            rpc_urls,
            wallet_path: env::var("WALLET_PATH").unwrap_or_else(|_| "wallets/hot.json".into()),
            wallet_address: env::var("WALLET_ADDRESS").unwrap_or_default(),
            slippage_bps: env::var("SLIPPAGE_BPS").ok().and_then(|v| v.parse().ok()).unwrap_or(75),
            max_retries: env::var("MAX_RETRIES").ok().and_then(|v| v.parse().ok()).unwrap_or(3),
            budget_sol: env::var("BUDGET_SOL").ok().and_then(|v| v.parse().ok()).unwrap_or(2.0),
            buy_cap_sol: env::var("BUY_CAP_SOL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.5),
            export_dir: env::var("EXPORT_DIR").unwrap_or_else(|_| "out".into()),
            report_prefix: env::var("REPORT_PREFIX").unwrap_or_else(|_| "trades".into()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
        })
    }

    /// Redacted view for logs (avoid leaking secrets)
    pub fn safe(&self) -> SafeConfig<'_> {
        SafeConfig {
            rpc_url: &self.rpc_url,
            rpc_urls: &self.rpc_urls,
            wallet_path: &self.wallet_path,
            wallet_address: "<redacted>",
            slippage_bps: self.slippage_bps,
            max_retries: self.max_retries,
            budget_sol: self.budget_sol,
            buy_cap_sol: self.buy_cap_sol,
            export_dir: &self.export_dir,
            report_prefix: &self.report_prefix,
            log_level: &self.log_level,
        }
    }
}

#[derive(Debug)]
pub struct SafeConfig<'a> {
    pub rpc_url: &'a str,
    pub rpc_urls: &'a [String],
    pub wallet_path: &'a str,
    pub wallet_address: &'a str,
    pub slippage_bps: u32,
    pub max_retries: u32,
    pub budget_sol: f64,
    pub buy_cap_sol: f64,
    pub export_dir: &'a str,
    pub report_prefix: &'a str,
    pub log_level: &'a str,
}

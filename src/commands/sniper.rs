
use crate::config::Config;
use anyhow::Result;
use tokio::time::{sleep, Duration};

pub async fn run(_cfg: &Config, token: &str, budget_sol: f64) -> Result<()> {
    log::info!("Sniper armed for token={} with budget {:.4} SOL", token, budget_sol);
    log::info!("Racing RPCs ... (simulated)");
    sleep(Duration::from_millis(350)).await;
    log::info!("Detected pool creation & liquidity add (simulated)");
    sleep(Duration::from_millis(250)).await;
    log::info!("BUY executed at slippage within cap (simulated)");
    Ok(())
}


use crate::config::Config;
use anyhow::Result;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize)]
struct BundlePlan {
    plans: Vec<TokenPlan>,
}

#[derive(Debug, Deserialize)]
struct TokenPlan {
    name: String,
    symbol: String,
    supply: u64,
    liquidity_sol: f64,
}

pub async fn run(cfg: &Config, plan_path: &str, dry_run: bool) -> Result<()> {
    let raw = std::fs::read_to_string(plan_path)?;
    let plan: BundlePlan = serde_json::from_str(&raw)?;
    log::info!("Loaded bundle plan with {} items", plan.plans.len());

    for (i, p) in plan.plans.iter().enumerate() {
        if dry_run {
            log::info!("[DRY] Would launch {} ({}) supply={} liq={} SOL via bags.fm", p.name, p.symbol, p.supply, p.liquidity_sol);
        } else {
            log::info!("[{}/{}] Launching {} ({}) on bags.fm ...", i+1, plan.plans.len(), p.name, p.symbol);
            // TODO: integrate bags.fm launch flow + Raydium listing
            sleep(Duration::from_millis(500)).await;
            log::info!("Launched {} with initial liquidity {:.2} SOL (simulated)", p.symbol, p.liquidity_sol);
        }
    }

    log::info!("Bundle complete. RPC primary = {}", cfg.rpc_url);
    Ok(())
}

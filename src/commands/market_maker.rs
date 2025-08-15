
use crate::config::Config;
use anyhow::Result;
use tokio::time::{sleep, Duration};

pub async fn run(_cfg: &Config, pair: &str, spread_bps: u32) -> Result<()> {
    log::info!("Starting market-maker for pair={} target_spread={} bps", pair, spread_bps);
    for _ in 0..3 {
        log::debug!("Placing bid/ask walls ... (simulated)");
        sleep(Duration::from_millis(300)).await;
        log::debug!("Rebalancing depth and inventory ... (simulated)");
        sleep(Duration::from_millis(300)).await;
    }
    log::info!("MM loop finished (demo mode)");
    Ok(())
}

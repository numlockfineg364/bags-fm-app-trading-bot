
use crate::config::Config;
use anyhow::Result;
use tokio::time::{sleep, Duration};

pub async fn run(_cfg: &Config, wallet: &str) -> Result<()> {
    log::info!("Tracking wallet={} for swaps/transfers/LP events (simulated)", wallet);
    for i in 1..=5 {
        sleep(Duration::from_millis(150)).await;
        log::info!("Event #{}: mock swap detected (simulated)", i);
    }
    Ok(())
}

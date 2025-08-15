
use crate::config::Config;
use anyhow::{Context, Result};
use chrono::{Utc, SecondsFormat};
use csv::WriterBuilder;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct TradeRow {
    time_utc: String,
    wallet: String,
    token: String,
    side: String,
    qty: f64,
    price: f64,
    pnl: f64,
    roi_pct: f64,
    hold_minutes: u64,
}

pub async fn run(cfg: &Config, out_path: &str) -> Result<()> {
    let out = Path::new(out_path);
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).ok();
    }

    let mut wtr = WriterBuilder::new().from_path(out).context("failed to open CSV")?;
    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    // sample rows
    let rows = vec![
        TradeRow { time_utc: now.clone(), wallet: "DemoWallet".into(), token: "DEMO1".into(), side: "BUY".into(), qty: 1000.0, price: 0.0021, pnl: 0.0, roi_pct: 0.0, hold_minutes: 0 },
        TradeRow { time_utc: now.clone(), wallet: "DemoWallet".into(), token: "DEMO1".into(), side: "SELL".into(), qty: 1000.0, price: 0.0030, pnl: 0.9, roi_pct: 42.85, hold_minutes: 37 },
    ];

    for r in rows {
        wtr.serialize(r)?;
    }
    wtr.flush()?;

    log::info!("Exported CSV to {}", out.display());
    Ok(())
}


use clap::{Parser, Subcommand};
use anyhow::Result;

mod config;
mod commands;

use commands::{bundler, sniper, market_maker, tracker, export};

/// SolGod — bags.fm trading bot scaffold
#[derive(Parser, Debug)]
#[command(author, version, about = "SolGod — bags.fm trading weapon (bundler/sniper/mm/track/export)", long_about = None)]
struct Cli {
    /// Increase verbosity (-v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute multi-launch bundle plan against bags.fm
    Bundler {
        /// Path to bundle plan (JSON)
        #[arg(short, long, default_value = "plans/example.json")]
        plan: String,
        /// Dry-run without execution
        #[arg(long)]
        dry_run: bool,
    },
    /// Snipe a token on listing detection
    Sniper {
        /// Token mint address
        #[arg(short, long)]
        token: String,
        /// Max budget in SOL
        #[arg(long, default_value_t = 1.0)]
        budget_sol: f64,
    },
    /// Market-making utilities (spread, depth simulation)
    MarketMaker {
        /// Token mint / pair id
        #[arg(short, long)]
        pair: String,
        /// Target spread in basis points
        #[arg(long, default_value_t = 100)]
        spread_bps: u32,
    },
    /// Real-time wallet tracking
    Track {
        /// Wallet address to track
        #[arg(short, long)]
        wallet: String,
    },
    /// Export trades/intelligence to CSV
    Export {
        /// Output CSV file path
        #[arg(short, long, default_value = "out/trades.csv")]
        out: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    // init logging
    let mut builder = env_logger::Builder::from_default_env();
    if cli.verbose > 0 {
        builder.filter_level(log::LevelFilter::Debug);
    }
    builder.init();

    let cfg = config::Config::from_env()?;
    log::info!("Loaded config: {:?}", cfg.safe());

    match cli.command {
        Commands::Bundler { plan, dry_run } => bundler::run(&cfg, &plan, dry_run).await?,
        Commands::Sniper { token, budget_sol } => sniper::run(&cfg, &token, budget_sol).await?,
        Commands::MarketMaker { pair, spread_bps } => market_maker::run(&cfg, &pair, spread_bps).await?,
        Commands::Track { wallet } => tracker::run(&cfg, &wallet).await?,
        Commands::Export { out } => export::run(&cfg, &out).await?,
    }

    Ok(())
}

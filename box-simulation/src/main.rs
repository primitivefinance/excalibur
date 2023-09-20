use std::time::Instant;

use anyhow::Result;
use tokio::task::JoinHandle;
use tracing::event;
use tracing_subscriber;
use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange, environment::Environment, middleware::RevmMiddleware,
};
use ethers::types::{Address, U256};

mod settings;
mod setup;
mod utils;
use std::sync::Arc;
mod agents;

/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let start = Instant::now();

    let config = settings::params::SimulationConfig::new()?;

    println!("Config: {:#?}", config);

    Ok(())
}

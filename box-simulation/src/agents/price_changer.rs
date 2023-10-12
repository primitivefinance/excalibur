use anyhow::Result;
use arbiter_core::{
    bindings::liquid_exchange::LiquidExchange,
    environment::Environment,
    math::{float_to_wad, OrnsteinUhlenbeck, StochasticProcess, Trajectories},
    middleware::RevmMiddleware,
};
use ethers::utils::parse_ether;
use params::PriceProcessParameters;
use token_admin::TokenAdmin;
use tracing::info;

use super::*;

/// The `PriceChanger` holds the data and has methods that allow it to update
/// the price of the `LiquidExchange`.
pub struct PriceChanger {
    /// The path the price process takes.
    pub trajectory: Trajectories,

    /// The `LiquidExchange` contract with the admin `Client`.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,

    /// The index of the current price in the trajectory.
    pub index: usize,
}

impl PriceChanger {
    /// Create a new `PriceChanger` with the given `LiquidExchange` contract
    /// bound to the admin `Client`. The `PriceChanger` will use the
    /// `OrnsteinUhlenbeck` process to generate a price trajectory with the
    /// constants defined in `config.rs`.
    /// Ornstein-Uhlenbeck processes are useful for modeling the price of stable
    /// tokens.
    pub async fn new(
        environment: &Environment,
        token_admin: &TokenAdmin,
        price_process_params: PriceProcessParameters,
    ) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "price_changer".into())?;
        let liquid_exchange = LiquidExchange::deploy(
            client,
            (
                token_admin.arbx.address(),
                token_admin.arby.address(),
                float_to_wad(price_process_params.initial_price),
            ),
        )?
        .send()
        .await?;

        token_admin
            .mint(
                liquid_exchange.address(),
                parse_ether(100_000_000_000_u64).unwrap(),
                parse_ether(100_000_000_000_u64).unwrap(),
            )
            .await?;

        let PriceProcessParameters {
            initial_price,
            mean,
            std_dev,
            theta,
            t_0,
            t_n,
            num_steps,
            seed,
        } = price_process_params;
        let process = OrnsteinUhlenbeck::new(mean, std_dev, theta);

        let trajectory = match seed {
            Some(seed) => {
                process.seedable_euler_maruyama(initial_price, t_0, t_n, num_steps, 1, false, seed)
            }
            None => process.euler_maruyama(initial_price, t_0, t_n, num_steps, 1, false),
        };

        Ok(Self {
            trajectory,
            liquid_exchange,
            index: 1, /* start after the initial price since it is already set on contract
                       * deployment */
        })
    }

    /// Update the price of the `LiquidExchange` contract to the next price in
    /// the trajectory and increment the index.
    pub async fn update_price(&mut self) -> Result<()> {
        let price = self.trajectory.paths[0][self.index];
        info!("Updating price of liquid_exchange to: {}", price);
        self.liquid_exchange
            .set_price(arbiter_core::math::float_to_wad(price))
            .send()
            .await?
            .await?;
        self.index += 1;
        Ok(())
    }
}

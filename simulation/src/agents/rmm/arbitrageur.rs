use std::{str::FromStr, sync::atomic};

use arbiter_core::bindings::arbiter_math::ArbiterMath;
use ethers::abi::AbiEncode;
use statrs::distribution::{ContinuousCDF, Normal};

use super::*;
use crate::{
    agents::Agent,
    bindings::rmm_math_like::RMMMathLike,
    strategy::{
        rmm::{RmmStrategy, RmmStrategyData},
        ArbitrageStrategy,
    },
};

#[derive(Debug, Clone)]
pub struct RmmArbitrageur<S: ArbitrageStrategy> {
    pub client: Arc<RevmMiddleware>,
    /// The arbitrageur's client connection to the liquid exchange.
    pub liquid_exchange: LiquidExchange<RevmMiddleware>,
    /// The rmm strategy used by the exchange.
    pub rmm_strategy: S,
    /// The atomic arbitrage contract.
    pub atomic_arbitrage: AtomicArbitrage<RevmMiddleware>,
    pub g3m_math: SD59x18Math<RevmMiddleware>,
    pub rmm_math: ArbiterMath<RevmMiddleware>,
}

impl<S: ArbitrageStrategy> RmmArbitrageur<S> {
    pub async fn new(
        environment: &Environment,
        token_admin: &token_admin::TokenAdmin,
        liquid_exchange_address: Address,
        rmm_address: Address,
    ) -> Result<Self> {
        // Create a client for the arbitrageur.
        let client = RevmMiddleware::new(environment, "arbitrageur".into())?;

        // Get the exchanges and arb contract connected to the arbitrageur client.
        let liquid_exchange = LiquidExchange::new(liquid_exchange_address, client.clone());
        let rmm_strategy = S::new(rmm_address, client.clone());
        let atomic_arbitrage = AtomicArbitrage::deploy(
            client.clone(),
            (
                rmm_address,
                liquid_exchange_address,
                token_admin.arbx.address(),
                token_admin.arby.address(),
            ),
        )?
        .send()
        .await?;

        let arbx = ArbiterToken::new(token_admin.arbx.address(), client.clone());
        let arby = ArbiterToken::new(token_admin.arby.address(), client.clone());

        token_admin
            .mint(
                client.address(),
                parse_ether(100_000_000).unwrap(),
                parse_ether(100_000_000).unwrap(),
            )
            .await?;

        arbx.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;
        arby.approve(atomic_arbitrage.address(), U256::MAX)
            .send()
            .await?
            .await?;

        let arby_allowance = arby
            .allowance(client.address(), atomic_arbitrage.address())
            .call()
            .await?;
        let arbx_allowance = arbx
            .allowance(client.address(), atomic_arbitrage.address())
            .call()
            .await?;

        println!("arbx_allowance: {:?}", arbx_allowance);
        println!("arby_allowance: {:?}", arby_allowance);

        let g3m_math = SD59x18Math::deploy(client.clone(), ())?.send().await?;
        let rmm_math = ArbiterMath::deploy(client.clone(), ())?.send().await?;

        Ok(Self {
            client,
            liquid_exchange,
            atomic_arbitrage,
            rmm_strategy,
            g3m_math,
            rmm_math,
        })
    }

    /// Detects if there is an arbitrage opportunity.
    /// Returns the direction of the swap `XtoY` or `YtoX` if there is an
    /// arbitrage opportunity. Returns `None` if there is no arbitrage
    /// opportunity.
    async fn detect_arbitrage(&self, pool: &S) -> Result<Swap> {
        // Update the prices the for the arbitrageur.
        let liquid_exchange_price_wad = self.liquid_exchange.price().call().await?;
        let price = pool.get_spot_price().await?;
        debug!("liquid_exchange_price_wad: {:?}", liquid_exchange_price_wad);
        debug!("rmm_price_wad: {:?}", price);

        let gamma_wad = WAD - pool.get_swap_fee().await?;

        let upper_arb_bound = WAD * price / (WAD - gamma_wad);
        let lower_arb_bound = price * (WAD - gamma_wad) / WAD;

        // Check if we have an arbitrage opportunity by comparing against the bounds and
        // current price.
        // If these conditions are not satisfied, there cannot be a profitable
        // arbitrage. See: [An Analysis of Uniswap Markets](https://arxiv.org/pdf/1911.03380.pdf) Eq. 3, for example.
        if liquid_exchange_price_wad > upper_arb_bound && liquid_exchange_price_wad > price {
            // Raise the portfolio price by selling asset for quote
            Ok(Swap::RaiseExchangePrice(liquid_exchange_price_wad))
        } else if liquid_exchange_price_wad < lower_arb_bound && liquid_exchange_price_wad < price {
            // Lower the exchange price by selling asset for quote
            Ok(Swap::LowerExchangePrice(liquid_exchange_price_wad))
        } else {
            // Prices are within the no-arbitrage bounds, so we don't have an arbitrage.
            Ok(Swap::None)
        }
    }
}
// TODO: make sure we're swapping on low and high vol strategies
#[async_trait::async_trait]
impl<S: ArbitrageStrategy + std::marker::Sync + std::marker::Send> Agent for RmmArbitrageur<S> {
    #[allow(unused)]
    async fn step(&mut self) -> Result<()> {
        // Detect if there is an arbitrage opportunity.
        let arbx = self.atomic_arbitrage.asset().call().await?;
        let arby = self.atomic_arbitrage.quote().call().await?;
        let arbx = ArbiterToken::new(arbx, self.client.clone());
        let arby = ArbiterToken::new(arby, self.client.clone());
        let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
        let arby_balance = arby.balance_of(self.client.address()).call().await?;
        debug!("arbx_balance: {:?}", arbx_balance);
        debug!("arby_balance: {:?}", arby_balance);

        match self.detect_arbitrage(&self.rmm_strategy).await? {
            Swap::RaiseExchangePrice(target_price) => {
                info!(
                    "Detected the need to raise price to {:?}",
                    format_units(target_price, "ether")?
                );
                let input = self
                    .rmm_strategy
                    .get_y_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;

                info!("got input: {:?}", input);
                if input < 0.into() {
                    return Ok(());
                }

                let rmm_math_like = RMMMathLike::deploy(self.client.clone(), ())?.send().await?;
                let (reserve_x, reserve_y, liquidity) =
                    self.rmm_strategy.get_reserves_and_liquidity().await?;
                let spot_price = self.rmm_strategy.get_spot_price().await?;
                let contract = RMM::new(self.rmm_strategy.get_address(), self.client.clone());
                let strike_price = contract.strike_price().call().await?;
                let sigma = contract.sigma().call().await?;
                let tau = contract.tau().call().await?;
                let delta_y = input;
                let delta_l = rmm_math_like
                    .compute_l_given_x(reserve_x, spot_price, strike_price, sigma)
                    .call()
                    .await?;

                let output_solidity = compute_output_x_given_y_solidity(
                    &rmm_math_like,
                    reserve_x,
                    reserve_y,
                    delta_y,
                    liquidity,
                    delta_l,
                    strike_price,
                    sigma,
                )
                .await?;

                let output_rust = compute_output_x_given_y_rust(
                    to_float(reserve_x),
                    to_float(reserve_y),
                    to_float(delta_y),
                    to_float(liquidity),
                    to_float(0.into()),
                    to_float(strike_price),
                    to_float(sigma),
                );

                let tx = self.atomic_arbitrage.raise_exchange_price(input);

                // todo: breaking here
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                debug!("arbx_balance after: {:?}", arbx_balance);
                debug!("arby_balance after: {:?}", arby_balance);
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("Execution revert: {:?} Gas Used: {:?}", output, gas_used);
                        }
                    }
                }
            }
            Swap::LowerExchangePrice(target_price) => {
                info!(
                    "Detected the need to lower price to {:?}",
                    format_units(target_price, "ether")?
                );
                let input = self
                    .rmm_strategy
                    .get_x_input(target_price, &self.g3m_math, &self.rmm_math)
                    .await?;
                info!("Got input: {:?}", input);
                if input <= 0.into() {
                    return Ok(());
                }
                let tx = self.atomic_arbitrage.lower_exchange_price(input);
                let output = tx.send().await;
                let arbx_balance = arbx.balance_of(self.client.address()).call().await?;
                let arby_balance = arby.balance_of(self.client.address()).call().await?;
                trace!("arbx_balance after: {:?}", arbx_balance);
                trace!("arby_balance after: {:?}", arby_balance);
                match output {
                    Ok(output) => {
                        output.await?;
                    }
                    Err(e) => {
                        if let RevmMiddlewareError::ExecutionRevert { gas_used, output } =
                            e.as_middleware_error().unwrap()
                        {
                            info!("Execution revert: {:?}", output);
                        }
                    }
                }
                info!("Sent arbitrage.");
            }
            Swap::None => {
                info!("No arbitrage opportunity");
            }
        }
        Ok(())
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }
}

enum Swap {
    RaiseExchangePrice(U256),
    LowerExchangePrice(U256),
    None,
}

#[cfg(test)]
mod tests {
    use statrs::distribution::Normal;
    use tracing_subscriber::prelude::*;

    use super::*;
    use crate::{
        agents::{
            price_changer::PriceChanger,
            rmm::{
                liquidity_provider::RmmLiquidityProvider,
                rmm_portfolio_manager::RmmPortfolioManagerType,
            },
            token_admin::TokenAdmin,
        },
        bindings::rmm_math_like::RMMMathLike,
        simulations::import,
        strategy::rmm::{get_strategy_args, RmmStrategy},
    };

    async fn setup(
        environment: &Environment,
    ) -> anyhow::Result<(RmmArbitrageur<RmmStrategy>, RMMMathLike<RevmMiddleware>), anyhow::Error>
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        let cwd = std::env::current_dir().unwrap();
        let path = std::path::Path::new(cwd.to_str().unwrap());
        let config_path = path
            .parent()
            .unwrap()
            .join("configs")
            .join("rmm_vol_targeting")
            .join("static.toml");
        let configuration = import(config_path.to_str().unwrap())?;
        let direct_configs: Vec<SimulationConfig<Single>> = configuration.clone().into();
        let config = &direct_configs.clone()[0];

        let token_admin = TokenAdmin::new(environment, config, "token_admin").await?;
        let mut price_changer =
            PriceChanger::new(environment, config, "price_changer", &token_admin).await?;
        let rmm_portfolio_manager = RmmPortfolioManagerType::new(
            environment,
            config,
            "portfolio_manager",
            price_changer.liquid_exchange.address(),
        )
        .await?;

        let client = RevmMiddleware::new(environment, Some(&"rmm_math_like"))?;
        let rmm_math = RMMMathLike::deploy(client.clone(), ())?.send().await?;

        let mut lp = RmmLiquidityProvider::<RmmStrategy>::new(
            environment,
            config,
            "lp",
            &token_admin,
            rmm_portfolio_manager.0.rmm().address(),
        )
        .await?;

        lp.startup().await?;

        let arbitrageur = RmmArbitrageur::<RmmStrategy>::new(
            environment,
            &token_admin,
            price_changer.liquid_exchange.address(),
            rmm_portfolio_manager.0.rmm().address(),
        )
        .await?;

        Ok((arbitrageur, rmm_math))
    }

    #[tokio::test]
    async fn test_math() -> anyhow::Result<(), anyhow::Error> {
        let environment = EnvironmentBuilder::new().build();
        let (arber, _) = setup(&environment).await?;

        let input = arber
            .rmm_strategy
            .get_y_input(parse_ether(1.0).unwrap(), &arber.g3m_math, &arber.rmm_math)
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_compute_l_given_x() -> anyhow::Result<(), anyhow::Error> {
        let environment = EnvironmentBuilder::new().build();
        let (arber, rmm_math_like) = setup(&environment).await?;

        let reserve_x = parse_ether(1000.0).unwrap();
        let spot_price = parse_ether(1.0).unwrap();
        let (sigma, strike_price, tau) = get_strategy_args(&arber.rmm_strategy).await?;
        let l =
            compute_l_given_x_solidity(&rmm_math_like, reserve_x, spot_price, strike_price, sigma)
                .await?;

        let l_rust = compute_l_given_x_rust(
            to_float(reserve_x),
            to_float(spot_price),
            to_float(strike_price),
            to_float(sigma),
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_compute_output_x_given_y() -> anyhow::Result<(), anyhow::Error> {
        let environment = EnvironmentBuilder::new().build();
        let (arber, rmm_math_like) = setup(&environment).await?;

        let reserve_x = parse_ether(1000.0).unwrap();
        let spot_price = parse_ether(1.0).unwrap();
        let (sigma, strike_price, tau) = get_strategy_args(&arber.rmm_strategy).await?;

        let reserve_y = parse_ether(1000.0).unwrap();
        let liquidity = parse_ether(1000.0).unwrap();
        let delta_l = parse_ether(1000.0).unwrap();
        let delta_y = parse_ether(1000.0).unwrap();
        let delta_x = parse_ether(1000.0).unwrap();

        let x = compute_output_x_given_y_solidity(
            &rmm_math_like,
            reserve_x,
            reserve_y,
            delta_y,
            liquidity,
            delta_l,
            strike_price,
            sigma,
        )
        .await?;

        let x_rust = compute_output_x_given_y_rust(
            to_float(reserve_x),
            to_float(reserve_y),
            to_float(delta_y),
            to_float(liquidity),
            to_float(delta_l),
            to_float(strike_price),
            to_float(sigma),
        );

        Ok(())
    }
}

pub fn to_float(value: U256) -> f64 {
    format_ether(value).parse::<f64>().unwrap()
}

/// L_x(x, S) = x / (1 - cdf(ln(S/K) + sigma^2/2) / sigma)
#[tracing::instrument(ret, skip(instance), level = "info")]
pub async fn compute_l_given_x_solidity(
    instance: &RMMMathLike<RevmMiddleware>,
    reserve_x: U256,
    spot_price: U256,
    strike_price: U256,
    sigma: U256,
) -> Result<(U256)> {
    let l = instance
        .compute_l_given_x(reserve_x, spot_price, strike_price, sigma)
        .call()
        .await?;
    Ok(l)
}

#[tracing::instrument(ret, level = "trace")]
pub fn get_s_k_ln(spot_price: f64, strike_price: f64) -> f64 {
    (spot_price / strike_price).ln()
}

#[tracing::instrument(ret, level = "trace")]
pub fn get_sigma_squared_over_two(sigma: f64) -> f64 {
    sigma.powi(2) / 2.0
}

#[tracing::instrument(ret, level = "trace")]
pub fn get_inner_term(spot_price_float: f64, strike_price_float: f64, sigma_float: f64) -> f64 {
    let s_k_ln = get_s_k_ln(spot_price_float, strike_price_float);
    let sigma_squared_over_two = get_sigma_squared_over_two(sigma_float);
    (s_k_ln + sigma_squared_over_two) / sigma_float
}

#[tracing::instrument(ret, level = "trace")]
pub fn get_cdf_inner_term(spot_price_float: f64, strike_price_float: f64, sigma_float: f64) -> f64 {
    let inner_term = get_inner_term(spot_price_float, strike_price_float, sigma_float);
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(inner_term)
}

#[tracing::instrument(ret, level = "trace")]
pub fn get_one_minus_cdf_inner_term(
    spot_price_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
) -> f64 {
    let inner_term = get_inner_term(spot_price_float, strike_price_float, sigma_float);
    let normal = Normal::new(0.0, 1.0).unwrap();
    1.0 - normal.cdf(inner_term)
}

#[tracing::instrument(ret, level = "trace")]
pub fn compute_l_given_x_rust(
    reserve_x_float: f64,
    spot_price_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
) -> f64 {
    let one_minus_cdf_inner_term =
        get_one_minus_cdf_inner_term(spot_price_float, strike_price_float, sigma_float);
    reserve_x_float / one_minus_cdf_inner_term
}

// I have no idea why this says i need to do this for clippy to pass but sure
#[allow(clippy::too_many_arguments)]
#[tracing::instrument(ret, skip(instance), level = "info")]
pub async fn compute_output_x_given_y_solidity(
    instance: &RMMMathLike<RevmMiddleware>,
    reserve_x: U256,
    reserve_y: U256,
    delta_y: U256,
    liquidity: U256,
    delta_l: U256,
    strike_price: U256,
    sigma: U256,
) -> Result<(I256)> {
    let x = instance
        .compute_output_x_given_y(
            reserve_x,
            reserve_y,
            delta_y,
            liquidity,
            delta_l,
            strike_price,
            sigma,
        )
        .await?;
    Ok(x)
}

/// delta_x =
/// (L + d_l) * cdf(-sigma - ppf((y + d_y) / K(L + d_l))) - x - d_x)
#[tracing::instrument(ret, level = "info")]
pub fn compute_output_x_given_y_rust(
    reserve_x_float: f64,
    reserve_y_float: f64,
    delta_y_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
    strike_price_float: f64,
    sigma_float: f64,
) -> f64 {
    let cdf_term = cdf_negative_sigma_less_ppf_term(
        sigma_float,
        reserve_y_float,
        delta_y_float,
        liquidity_float,
        delta_l_float,
        strike_price_float,
    );

    let adjusted_l = liquidity_float + delta_l_float;
    adjusted_l * cdf_term - reserve_x_float
}

#[tracing::instrument(ret, level = "trace")]
pub fn y_plus_delta_y(reserve_y_float: f64, delta_y_float: f64) -> f64 {
    reserve_y_float + delta_y_float
}

#[tracing::instrument(ret, level = "trace")]
pub fn strike_price_mul_l_plus_delta_l(
    strike_price_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
) -> f64 {
    strike_price_float * (liquidity_float + delta_l_float)
}

#[tracing::instrument(ret, level = "trace")]
pub fn adjusted_y_div_adjusted_l(
    reserve_y_float: f64,
    delta_y_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
    strike_price_float: f64,
) -> f64 {
    let adjusted_y = y_plus_delta_y(reserve_y_float, delta_y_float);
    let adjusted_l =
        strike_price_mul_l_plus_delta_l(strike_price_float, liquidity_float, delta_l_float);
    adjusted_y / adjusted_l
}

#[tracing::instrument(ret, level = "trace")]
pub fn ppf_adjusted_y_over_l(
    reserve_y_float: f64,
    delta_y_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
    strike_price_float: f64,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let adjusted_y_over_l = adjusted_y_div_adjusted_l(
        reserve_y_float,
        delta_y_float,
        liquidity_float,
        delta_l_float,
        strike_price_float,
    );
    normal.inverse_cdf(adjusted_y_over_l)
}

#[tracing::instrument(ret, level = "trace")]
pub fn negative_sigma_less_ppf_term(
    sigma_float: f64,
    reserve_y_float: f64,
    delta_y_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
    strike_price_float: f64,
) -> f64 {
    let ppf_adjusted_y_over_l = ppf_adjusted_y_over_l(
        reserve_y_float,
        delta_y_float,
        liquidity_float,
        delta_l_float,
        strike_price_float,
    );
    -sigma_float - ppf_adjusted_y_over_l
}

#[tracing::instrument(ret, level = "trace")]
pub fn cdf_negative_sigma_less_ppf_term(
    sigma_float: f64,
    reserve_y_float: f64,
    delta_y_float: f64,
    liquidity_float: f64,
    delta_l_float: f64,
    strike_price_float: f64,
) -> f64 {
    let negative_sigma_less_ppf_term = negative_sigma_less_ppf_term(
        sigma_float,
        reserve_y_float,
        delta_y_float,
        liquidity_float,
        delta_l_float,
        strike_price_float,
    );
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(negative_sigma_less_ppf_term)
}
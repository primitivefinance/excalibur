use std::sync::Arc;

use alloy_primitives::Address;
use arbiter_bindings::bindings::liquid_exchange::LiquidExchange;
use itertools::iproduct;
use tracing::{debug, info, trace};

use super::*;
use crate::{bindings::dfmm::DFMM, *};

#[derive(Debug, Clone)]
pub struct VolatilityTargetingSubmitter {
    pub client: Arc<RevmMiddleware>,
    pub lex: LiquidExchange<RevmMiddleware>,
    pub dfmm: DFMM<RevmMiddleware>,
    pub next_update_timestamp: u64,
    pub update_frequency: u64,
    pub target_volatility: f64,
    pub portfolio_prices: Vec<(f64, u64)>,
    pub asset_prices: Vec<(f64, u64)>,
    pub portfolio_rv: Vec<(f64, u64)>,
    pub asset_rv: Vec<(f64, u64)>,
}

#[async_trait::async_trait]
impl Agent for VolatilityTargetingSubmitter {
    async fn step(&mut self) -> Result<()> {
        let timestamp = self.client.get_block_timestamp().await?.as_u64();
        let asset_price =
            ethers::utils::format_ether(self.lex.price().call().await?).parse::<f64>()?;
        let reserve_x =
            ethers::utils::format_ether(self.dfmm.reserve_x_wad().call().await?).parse::<f64>()?;
        let reserve_y =
            ethers::utils::format_ether(self.dfmm.reserve_y_wad().call().await?).parse::<f64>()?;
        let portfolio_price = reserve_x * asset_price + reserve_y;

        if self.portfolio_prices.is_empty() {
            info!("portfolio_price: {}", portfolio_price);
            self.portfolio_prices.push((portfolio_price, 0));
            self.asset_prices.push((asset_price, 0));
        }

        if timestamp >= self.next_update_timestamp {
            self.next_update_timestamp = timestamp + self.update_frequency;
            debug!("portfolio_price: {}", portfolio_price);
            self.asset_prices.push((asset_price, timestamp));
            self.portfolio_prices.push((portfolio_price, timestamp));
            self.calculate_rv()?;
            self.execute_smooth_rebalance().await?;
        }
        Ok(())
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl VolatilityTargetingSubmitter {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let lex = LiquidExchange::new(to_ethers_address(liquid_exchange_address), client.clone());

        tracing::info!("params: {:?}", config.agent_parameters.get(&label));

        if let Some(AgentParameters::VolatilityTargetingSubmitter(params)) =
            config.agent_parameters.get(&label)
        {
            // let dfmm_args = (
            // lex.arbiter_token_x().call().await?,
            // lex.arbiter_token_y().call().await?,
            // ethers::utils::parse_ether(1)?,
            // ethers::utils::parse_ether(0.8)?,
            // ethers::utils::parse_ether(1.0)?,
            // ethers::utils::parse_ether(0.997)?,
            // );

            let args = (
                lex.arbiter_token_x().call().await?,
                lex.arbiter_token_y().call().await?,
                ethers::utils::parse_ether(params.fee.0 / 10_000.0)?,
            );
            tracing::info!("args: {:?}", args);
            match params.specialty {
                Specialty::VolatilityTargeting(parameters) => {
                    let dfmm = DFMM::deploy(client.clone(), args)?.send().await?;
                    trace!("Deployed dfmm at address: {:?}", dfmm.address());
                    let strategist = VolatilityTargetingSubmitter {
                        client,
                        lex,
                        dfmm,
                        update_frequency: parameters.update_frequency.0 as u64,
                        next_update_timestamp: parameters.update_frequency.0 as u64,
                        target_volatility: parameters.target_volatility.0,
                        portfolio_prices: Vec::new(),
                        asset_prices: Vec::new(),
                        portfolio_rv: Vec::new(),
                        asset_rv: Vec::new(),
                    };
                    Ok(strategist)
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for volatility targeting portfolio manager"
            ))
        }
    }

    async fn execute_smooth_rebalance(&mut self) -> Result<()> {
        if self.portfolio_rv.len() < 2 {
            return Ok(());
        }
        // let portfolio_rv = self.portfolio_rv.last().unwrap().0;
        // info!("portfolio_rv: {}", portfolio_rv);
        // let rv_difference = portfolio_rv - self.target_volatility;
        // let current_strike = self.dfmm.strike_price().call().await?;
        // let current_strike_float =
        // ethers::utils::format_ether(current_strike).parse::<f64>().unwrap();
        // let strike_change = self.sensitivity * rv_difference;
        // info!("current strike float: {}", current_strike_float);
        // let mut new_strike = current_strike_float;
        // if portfolio_rv > self.target_volatility {
        // new_strike -= 0.0015;
        // } else {
        // new_strike += 0.0015;
        // }
        // info!("new strike float: {}", new_strike);
        // self.dfmm
        // .set_strike_price(
        // parse_ether(new_strike.to_string()).unwrap(),
        // U256::from(self.next_update_timestamp),
        // )
        // .send()
        // .await?;
        Ok(())
    }

    fn calculate_rv(&mut self) -> Result<()> {
        // if self.asset_prices.len() > 15 then only calculate for the last 15 elements
        if self.asset_prices.len() > 15 {
            let asset_rv = compute_realized_volatility(
                self.asset_prices
                    .iter()
                    .skip(self.asset_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );
            self.asset_rv.push((asset_rv, self.next_update_timestamp));
        }
        if self.portfolio_prices.len() > 15 {
            let portfolio_rv = compute_realized_volatility(
                self.portfolio_prices
                    .iter()
                    .skip(self.portfolio_prices.len() - 15)
                    .map(|(price, _)| *price)
                    .collect::<Vec<f64>>(),
            );

            self.portfolio_rv
                .push((portfolio_rv, self.next_update_timestamp));
        }
        debug!(
            "hypothetical percent asset return: {}",
            (self.asset_prices.last().unwrap().0 - self.asset_prices.first().unwrap().0)
                / self.asset_prices.first().unwrap().0
        );
        debug!(
            "portfolio percent return: {}",
            (self.portfolio_prices.last().unwrap().0 - self.portfolio_prices.first().unwrap().0)
                / self.portfolio_prices.first().unwrap().0
        );
        debug!(
            "initial portfolio price: {}",
            self.portfolio_prices.first().unwrap().0
        );
        debug!(
            "current portfolio price: {}",
            self.portfolio_prices.last().unwrap().0
        );

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmitterParameters<P: Parameterized> {
    sigma: P,
    tau: P,
    strike_price: P,
    pub fee: P,
    pub specialty: Specialty<P>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Specialty<P: Parameterized> {
    VolatilityTargeting(DynamicVolatilityTargetingParameters<P>),
}

impl From<SubmitterParameters<Multiple>> for Vec<SubmitterParameters<Single>> {
    fn from(item: SubmitterParameters<Multiple>) -> Self {
        let specialties: Vec<Specialty<Single>> = item.specialty.into();
        iproduct!(
            item.sigma.parameters(),
            item.tau.parameters(),
            item.strike_price.parameters(),
            item.fee.parameters(),
            specialties
        )
        .map(|(s, tau, sp, f, specialty)| SubmitterParameters {
            sigma: Single(s),
            tau: Single(tau),
            strike_price: Single(sp),
            fee: Single(f),
            specialty,
        })
        .collect()
    }
}

impl From<Specialty<Multiple>> for Vec<Specialty<Single>> {
    fn from(item: Specialty<Multiple>) -> Self {
        match item {
            Specialty::VolatilityTargeting(parameters) => {
                let parameters: Vec<DynamicVolatilityTargetingParameters<Single>> =
                    parameters.into();
                parameters
                    .into_iter()
                    .map(Specialty::VolatilityTargeting)
                    .collect()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicVolatilityTargetingParameters<P: Parameterized> {
    pub target_volatility: P,
    pub update_frequency: P,
}

impl From<DynamicVolatilityTargetingParameters<Multiple>>
    for Vec<DynamicVolatilityTargetingParameters<Single>>
{
    fn from(item: DynamicVolatilityTargetingParameters<Multiple>) -> Self {
        iproduct!(
            item.target_volatility.parameters(),
            item.update_frequency.parameters()
        )
        .map(|(tv, uf)| DynamicVolatilityTargetingParameters {
            target_volatility: Single(tv),
            update_frequency: Single(uf),
        })
        .collect()
    }
}

/// Math functions for portfolio optimization and management.
/// Compute the returns of a series of values.
/// Which is defined as the ratio of the current value to the previous value
/// minus 1.
pub fn compute_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let values = values.into_iter().collect::<Vec<f64>>();
    let mut returns = Vec::new();
    for i in 1..values.len() {
        returns.push(values[i] / values[i - 1] - 1.0);
    }
    returns
}

pub fn compute_log_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut previous_value = 0.0_f64;
    let mut returns = Vec::new();
    for value in values {
        if previous_value != 0.0 {
            returns.push((value / previous_value).ln());
        }
        previous_value = value;
    }
    returns
}

pub fn compute_simple_returns(values: impl IntoIterator<Item = f64>) -> Vec<f64> {
    let mut previous_value = 0.0_f64;
    let mut returns = Vec::new();
    for value in values {
        if previous_value != 0.0 {
            returns.push(value / previous_value - 1.0);
        }
        previous_value = value;
    }
    returns
}

pub fn compute_net_returns(values: impl IntoIterator<Item = f64>) -> f64 {
    let values = values.into_iter().collect::<Vec<f64>>();
    let net_return = values.last().unwrap_or(&0.0) / values.first().unwrap_or(&1.0) - 1.0;
    net_return
}

pub fn compute_variance(values: impl IntoIterator<Item = f64>) -> f64 {
    let values = values.into_iter().collect::<Vec<f64>>();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|&return_| (return_ - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    variance
}

pub fn compute_std_deviation(values: impl IntoIterator<Item = f64>) -> f64 {
    let variance = compute_variance(values);
    variance.sqrt()
}

pub fn compute_realized_volatility(values: impl IntoIterator<Item = f64>) -> f64 {
    let returns = compute_log_returns(values);
    let len = returns.len() + 1;
    compute_std_deviation(returns) / (len as f64 / 365.0)
}
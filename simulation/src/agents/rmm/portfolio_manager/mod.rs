use self::rmm_volatility_targeting::{
    RmmVolatilityTargetingParameters, RmmVolatilityTargetingStrategist,
};
use super::*;

pub mod rmm_volatility_targeting;

#[async_trait::async_trait]
pub trait PortfolioManager: Agent {
    async fn execute_rebalance(&mut self) -> Result<()>;
    fn rmm(&self) -> &RMM<RevmMiddleware>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortfolioManagerParameters<P: Parameterized> {
    sigma: P,
    tau: P,
    strike_price: P,
    pub fee: P,
    pub specialty: PortfolioManagerSpecialty<P>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PortfolioManagerSpecialty<P: Parameterized> {
    VolatilityTargeting(RmmVolatilityTargetingParameters<P>),
}

use itertools::iproduct;

impl From<PortfolioManagerParameters<Multiple>> for Vec<PortfolioManagerParameters<Single>> {
    fn from(item: PortfolioManagerParameters<Multiple>) -> Self {
        let specialties: Vec<PortfolioManagerSpecialty<Single>> = item.specialty.into();
        iproduct!(
            item.sigma.parameters(),
            item.tau.parameters(),
            item.strike_price.parameters(),
            item.fee.parameters(),
            specialties
        )
        .map(|(s, tau, sp, f, specialty)| PortfolioManagerParameters {
            sigma: Single(s),
            tau: Single(tau),
            strike_price: Single(sp),
            fee: Single(f),
            specialty,
        })
        .collect()
    }
}

impl From<PortfolioManagerSpecialty<Multiple>> for Vec<PortfolioManagerSpecialty<Single>> {
    fn from(item: PortfolioManagerSpecialty<Multiple>) -> Self {
        match item {
            PortfolioManagerSpecialty::VolatilityTargeting(parameters) => {
                let parameters: Vec<RmmVolatilityTargetingParameters<Single>> = parameters.into();
                parameters
                    .into_iter()
                    .map(PortfolioManagerSpecialty::VolatilityTargeting)
                    .collect()
            }
        }
    }
}

pub struct PortfolioManagerType(pub Box<dyn PortfolioManager>);

impl PortfolioManagerType {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
        liquid_exchange_address: Address,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;
        let lex = LiquidExchange::new(liquid_exchange_address, client.clone());

        if let Some(AgentParameters::PortfolioManager(params)) = config.agent_parameters.get(&label)
        {
            let rmm_args = (
                lex.arbiter_token_x().call().await?,
                lex.arbiter_token_y().call().await?,
                parse_ether(0.3)?,
                parse_ether(1.0)?,
                parse_ether(1.0)?,
                parse_ether(0.997)?,
            );
            match params.specialty {
                PortfolioManagerSpecialty::VolatilityTargeting(parameters) => {
                    let rmm = RMM::deploy(client.clone(), rmm_args)?.send().await?;
                    trace!("Deployed rmm at address: {:?}", rmm.address());
                    let strategist = RmmVolatilityTargetingStrategist {
                        client,
                        lex,
                        rmm,
                        update_frequency: parameters.update_frequency.0 as u64,
                        next_update_timestamp: parameters.update_frequency.0 as u64,
                        target_volatility: parameters.target_volatility.0,
                        portfolio_prices: Vec::new(),
                        asset_prices: Vec::new(),
                        portfolio_rv: Vec::new(),
                        asset_rv: Vec::new(),
                    };
                    Ok(Self(Box::new(strategist)))
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "No parameters found for volatility targeting portfolio manager"
            ))
        }
    }
}

#[async_trait::async_trait]
impl Agent for PortfolioManagerType {
    async fn step(&mut self) -> Result<()> {
        self.0.step().await
    }

    async fn startup(&mut self) -> Result<()> {
        self.0.startup().await
    }
}

#[async_trait::async_trait]
impl PortfolioManager for PortfolioManagerType {
    async fn execute_rebalance(&mut self) -> Result<()> {
        self.0.execute_rebalance().await
    }

    fn rmm(&self) -> &RMM<RevmMiddleware> {
        self.0.rmm()
    }
}
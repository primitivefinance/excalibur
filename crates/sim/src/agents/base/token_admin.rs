use std::sync::Arc;

use alloy_primitives::{Address, U256};
use arbiter_bindings::bindings::arbiter_token::ArbiterToken;

use super::*;

#[derive(Clone, Debug)]
pub struct TokenAdmin {
    pub client: Arc<RevmMiddleware>,
    pub arbx: ArbiterToken<RevmMiddleware>,
    pub arby: ArbiterToken<RevmMiddleware>,
}

#[async_trait::async_trait]
impl Agent for TokenAdmin {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn client(&self) -> Arc<RevmMiddleware> {
        self.client.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenAdminParameters {
    pub arbx: TokenData,
    pub arby: TokenData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl TokenAdmin {
    pub async fn new(
        environment: &Environment,
        config: &SimulationConfig<Single>,
        label: impl Into<String>,
    ) -> Result<Self> {
        let label: String = label.into();
        let client = RevmMiddleware::new(environment, Some(&label))?;

        if let Some(AgentParameters::TokenAdmin(parameters)) = config.agent_parameters.get(&label) {
            let arbx = ArbiterToken::deploy(
                client.clone(),
                (
                    parameters.arbx.name.clone(),
                    parameters.arbx.symbol.clone(),
                    parameters.arbx.decimals,
                ),
            )?
            .send()
            .await?;
            let arby = ArbiterToken::deploy(
                client.clone(),
                (
                    parameters.arby.name.clone(),
                    parameters.arby.symbol.clone(),
                    parameters.arby.decimals,
                ),
            )?
            .send()
            .await?;
            Ok(Self { client, arbx, arby })
        } else {
            Err(anyhow::anyhow!("No parameters found for token admin"))
        }
    }

    pub async fn mint(&self, to: Address, amount_x: U256, amount_y: U256) -> Result<()> {
        let to = ethers::types::Address::from(to.into_array());
        self.arbx
            .mint(to, to_ethers_u256(amount_x))
            .send()
            .await?
            .await?;
        self.arby
            .mint(to, to_ethers_u256(amount_y))
            .send()
            .await?
            .await?;
        Ok(())
    }
}
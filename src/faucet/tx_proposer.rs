use crate::faucet::storage::models;
use crate::faucet::Settings;
use crate::storage::PoolType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TxProposer {
    connpool: PoolType,
    fundings: HashMap<String, String>, // TODO: use decimals
}

impl TxProposer {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        Self {
            connpool,
            fundings: config.fundings.clone(),
        }
    }

    pub async fn run(&self) {
        unimplemented!()
    }

    async fn propose_fundings(&self, user_id: i32) -> Result<(), anyhow::Error> {
        for (asset, amount) in &self.fundings {
            self.propose_funding(user_id, asset, amount).await?;
        }

        Ok(())
    }

    async fn propose_funding(&self, user_id: i32, asset: &str, amount: &str) -> Result<(), anyhow::Error> {
        let stmt = format!(
            "insert into {} (to_user, asset, amount) values ($1, $2, $3)",
            models::tablenames::FAUCET_TX
        );
        sqlx::query(&stmt)
            .bind(user_id)
            .bind(asset)
            .bind(amount)
            .execute(&self.connpool)
            .await?;

        Ok(())
    }
}

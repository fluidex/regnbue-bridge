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

    // TODO: fix types
    async fn propose_fundings(&self, user_id: i32) -> Result<(), anyhow::Error> {
        for (asset, amount) in &self.fundings {
            let stmt = format!(
                "insert into {} (to_user, asset, amount) values ($1, $2, $3)",
                models::tablenames::FAUCET_TX
            );
            match sqlx::query(&stmt)
                .bind(user_id)
                .bind(asset)
                .bind(amount)
                .execute(&self.connpool)
                .await
            {
                Err(e) => {
                    log::error!(
                        "propose funding for user {:?}, asset: {:?}, amount: {:?} , error: {:?}",
                        user_id,
                        asset,
                        amount,
                        e
                    )
                }
                _ => {}
            }
        }

        Ok(())
    }
}

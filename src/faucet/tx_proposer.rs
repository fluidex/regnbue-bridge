use super::msg::load_msgs_from_mq;
use crate::faucet::storage::models;
use crate::faucet::Settings;
use crate::mq::messages::WrappedMessage;
use crate::storage::{DecimalDbType, PoolType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TxProposer {
    brokers: String,
    connpool: PoolType,
    fundings: HashMap<String, DecimalDbType>,
}

impl TxProposer {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        Self {
            brokers: config.brokers.to_owned(),
            connpool,
            fundings: config.fundings.clone(),
        }
    }

    pub async fn run(&self) {
        let (msg_sender, msg_receiver) = crossbeam_channel::unbounded();
        let loader_thread = load_msgs_from_mq(&self.brokers, msg_sender);

        for msg in msg_receiver.iter() {
            match msg {
                WrappedMessage::User(user) => {
                    self.propose_fundings(user.user_id).await.unwrap();
                }
            }
        }

        loader_thread.map(|h| h.join().expect("loader thread failed"));
    }

    async fn propose_fundings(&self, user_id: i32) -> Result<(), anyhow::Error> {
        for (asset, amount) in &self.fundings {
            let stmt = format!(
                "insert into {} (to_user, asset, amount) values ($1, $2, $3)",
                models::tablenames::FAUCET_TX
            );
            if let Err(e) = sqlx::query(&stmt)
                .bind(user_id)
                .bind(asset)
                .bind(amount) // TODO: to_string?
                .execute(&self.connpool)
                .await
            {
                log::error!(
                    "propose funding for user {:?}, asset: {:?}, amount: {:?} , error: {:?}",
                    user_id,
                    asset,
                    amount,
                    e
                )
            }
        }

        Ok(())
    }
}

use super::types::{ContractCall, SubmitBlockArgs};
use crate::storage::PoolType;
use crate::tele_out::Settings;
use crossbeam_channel::Sender;
use ethers::types::U256;
use fluidex_common::db::models;
use std::time::Duration;

#[derive(Debug)]
pub struct TaskFetcher {
    connpool: PoolType,
}

impl TaskFetcher {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    pub async fn run(&self, tx: Sender<ContractCall>) {
        let mut timer = tokio::time::interval(Duration::from_secs(1));

        // TODO: reset subttimg

        loop {
            timer.tick().await;
            log::debug!("ticktock!");

            if let Err(e) = self.run_inner(&tx).await {
                log::error!("{}", e);
            };
        }
    }

    // TODO: this only support commitBlock. we will also need to support proveBlock
    // TODO: marking as submitting
    async fn run_inner(&self, tx: &Sender<ContractCall>) -> Result<(), anyhow::Error> {
        let query: &'static str = const_format::formatcp!(
            r#"
            select * from {} t
            inner join {} b on t.block_id = b.block_id
            where block.status = $1
            ORDER_BY t.block_id ASC
            LIMIT 1"#,
            models::tablenames::TASK,
            models::tablenames::L2_BLOCK,
        );
        let task: Option<models::task::Task> = sqlx::query_as(&query)
            .bind(models::l2_block::BlockStatus::Uncommited)
            .fetch_optional(&self.connpool)
            .await?;

        if task.is_some() {
            let task = task.unwrap();
            let public_inputs: Vec<U256> = serde_json::de::from_slice(&task.public_input.unwrap())?;
            let serialized_proof: Vec<U256> = serde_json::de::from_slice(&task.proof.unwrap())?;
            tx.try_send(ContractCall::SubmitBlock(SubmitBlockArgs {
                block_id: task.block_id.into(),
                public_inputs,
                serialized_proof,
            }))?;
        }

        Ok(())
    }
}

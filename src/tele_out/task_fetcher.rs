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
        loop {
            timer.tick().await;
            log::debug!("ticktock!");

            if let Err(e) = self.run_inner(&tx).await {
                log::error!("{}", e);
            };
        }
    }

    async fn run_inner(&self, tx: &Sender<ContractCall>) -> Result<(), anyhow::Error> {
        let mut db_tx = self.connpool.begin().await?;

        #[derive(sqlx::FromRow, Debug, Clone)]
        struct Task {
            block_id: i64,
            public_input: Vec<u8>,
            proof: Vec<u8>,
        }

        let query: &'static str = const_format::formatcp!(
            r#"
            select t.block_id     as block_id,
                   t.public_input as public_input,
                   t.proof        as proof
            from task t
                     inner join l2block l2b
                                on t.block_id = l2b.block_id
            where t.block_id < coalesce((select block_id
                                         from task
                                         where status <> 'proved'
                                         order by block_id
                                         limit 1), 0)
              and t.status = 'proved' -- defense filter
              and l2b.status = 'uncommited'
            order by t.block_id
            limit 1"#,
            models::tablenames::TASK,
            models::tablenames::L2_BLOCK,
        );
        let task: Option<Task> = sqlx::query_as(&query)
            .fetch_optional(&mut db_tx)
            .await?;

        if let Some(task) = task {
            let public_inputs: Vec<U256> = serde_json::de::from_slice(&task.public_input)?;
            let serialized_proof: Vec<U256> = serde_json::de::from_slice(&task.proof)?;
            tx.try_send(ContractCall::SubmitBlock(SubmitBlockArgs {
                block_id: task.block_id.into(),
                public_inputs,
                serialized_proof,
            }))?;

            let stmt = format!("update {} set status = $1 where block_id = $2", models::tablenames::L2_BLOCK);
            sqlx::query(&stmt)
                .bind(models::l2_block::BlockStatus::Commited)
                .bind(task.block_id)
                .execute(&mut db_tx)
                .await?;
        }

        db_tx.commit().await?;
        Ok(())
    }
}

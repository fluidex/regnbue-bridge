use super::types::{ContractCall, SubmitBlockArgs};
use crate::block_submitter::Settings;
use crate::storage::{DbType, PoolType};
use crossbeam_channel::Sender;
use ethers::types::U256;
use fluidex_common::db::models;
use std::time::Duration;

#[derive(Debug)]
pub struct TaskFetcher {
    connpool: PoolType,
    last_block_id: Option<i64>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct Task {
    block_id: i64,
    public_input: Vec<u8>,
    proof: Vec<u8>,
    public_data: Vec<u8>,
}

impl TryFrom<Task> for SubmitBlockArgs {
    type Error = anyhow::Error;

    fn try_from(t: Task) -> Result<Self, Self::Error> {

        let public_inputs: Vec<U256> = serde_json::de::from_slice(&t.public_input)?;
        let serialized_proof: Vec<U256> = serde_json::de::from_slice(&t.proof)?;

        Ok(SubmitBlockArgs {
            block_id: U256::from(t.block_id),
            public_inputs,
            serialized_proof,
            public_data: t.public_data,
        })
    }
}

impl SubmitBlockArgs {

    pub async fn fetch_by_blockid<'c>(block_id: i64, conn: impl sqlx::Executor<'c, Database = DbType>) -> Result<Option<Self>, anyhow::Error>{
        let query: &'static str = const_format::formatcp!(
            r#"
            select t.block_id     as block_id,
                   t.public_input as public_input,
                   t.proof        as proof,
                   l2b.raw_public_data as public_data
            from {} t
                     inner join {} l2b
                                on t.block_id = l2b.block_id
            where t.block_id = $1
            limit 1"#,
            models::tablenames::TASK,
            models::tablenames::L2_BLOCK,
        );

        let task: Option<Task> = sqlx::query_as(query)
            .bind(block_id)
            .fetch_optional(conn)
            .await?;
        
        match task {
            Some(task) => Self::try_from(task).map(|t|Some(t)),
            None => Ok(None),
        }
    }

    pub async fn fetch_latest<'c>(start_id: Option<i64>, conn: impl sqlx::Executor<'c, Database = DbType>) -> Result<Option<Self>, anyhow::Error>{
        let query: &'static str = const_format::formatcp!(
            r#"
            select t.block_id     as block_id,
                   t.public_input as public_input,
                   t.proof        as proof,
                   l2b.raw_public_data as public_data
            from {} t
                     inner join {} l2b
                                on t.block_id = l2b.block_id
            where t.block_id < coalesce((select block_id
                                         from task
                                         where status <> 'proved'
                                         order by block_id
                                         limit 1), 9223372036854775807)
              and t.block_id > $1
              and t.status = 'proved' -- defense filter
              and l2b.status = 'uncommited'
            order by t.block_id
            limit 1"#,
            models::tablenames::TASK,
            models::tablenames::L2_BLOCK,
        );

        let task: Option<Task> = sqlx::query_as(query)
            .bind(start_id.unwrap_or(-1))
            .fetch_optional(conn)
            .await?;
        
        match task {
            Some(task) => Self::try_from(task).map(|t|Some(t)),
            None => Ok(None),
        }
    
    }

}

impl TaskFetcher {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self {
            connpool,
            last_block_id: None,
        }
    }

    pub async fn run(&mut self, tx: Sender<ContractCall>) {
        let mut timer = tokio::time::interval(Duration::from_secs(1));
        loop {
            timer.tick().await;
            log::debug!("ticktock!");

            if let Err(e) = self.run_inner(&tx).await {
                log::error!("{}", e);
            };
        }
    }

    async fn run_inner(&mut self, tx: &Sender<ContractCall>) -> Result<(), anyhow::Error> {
        let mut db_tx = self.connpool.begin().await?;

        if let Some(args) = SubmitBlockArgs::fetch_latest(self.last_block_id, &mut db_tx).await? {
            let last_id = args.block_id.as_u64() as i64;
            tx.try_send(ContractCall::SubmitBlock(args))?;
            self.last_block_id = Some(last_id);
        }

        db_tx.commit().await?;
        Ok(())
    }
}

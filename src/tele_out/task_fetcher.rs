use super::types::{ContractCall, models};
use crate::storage::PoolType;
use crate::tele_out::Settings;
use crossbeam_channel::Sender;

#[derive(Debug)]
pub struct TaskFetcher {
    connpool: PoolType,
}

impl TaskFetcher {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    // TODO: better type binding
    pub async fn run(&self, tx: Sender<ContractCall>) {
        // TODO: ticker loop

        // TOOD: can we use super::types directly?
        let query = "select * from task where status = 'proved' LIMIT 1";
        // TODO: error handling
        let task: Option<models::Task> = sqlx::query_as(&query).fetch_optional(&self.connpool).await.unwrap();

        unimplemented!()
    }
}

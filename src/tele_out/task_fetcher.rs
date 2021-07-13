use super::types::ContractCall;
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

    pub async fn run(&self, _tx: Sender<ContractCall>) {
        unimplemented!()
    }
}

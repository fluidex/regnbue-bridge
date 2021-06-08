use crate::tele_out::storage::PoolType;
use crate::tele_out::Settings;

#[derive(Debug)]
pub struct TaskFetcher {
    connpool: PoolType,
}

impl TaskFetcher {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    pub fn run(&self) -> tokio::task::JoinHandle<()> {
        unimplemented!()
    }
}

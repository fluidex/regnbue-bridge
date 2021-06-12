use crate::storage::PoolType;
use crate::tele_in::Settings;

#[derive(Debug)]
pub struct TxProposer {
    connpool: PoolType,
}

impl TxProposer {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    pub fn run(&self) -> tokio::task::JoinHandle<()> {
        unimplemented!()
    }
}

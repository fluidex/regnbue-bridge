use crate::storage::PoolType;
use crate::tele_in::Settings;

#[derive(Debug)]
pub struct TxSender {
    connpool: PoolType,
}

impl TxSender {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    pub fn run(&self) -> tokio::task::JoinHandle<()> {
        unimplemented!()
    }
}

use crate::tele_out::Settings;
use crate::tele_out::storage::PoolType;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
}

impl EthSender {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        Self {
            connpool
        }
    }

    pub fn run(&self) -> tokio::task::JoinHandle<()> {
        unimplemented!()
    }
}

use crate::storage::PoolType;
use crate::tele_out::Settings;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
}

impl EthSender {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    pub async fn run(&self) {
        unimplemented!()
    }
}

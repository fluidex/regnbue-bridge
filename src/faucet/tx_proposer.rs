use crate::faucet::Settings;
use crate::storage::PoolType;

#[derive(Debug)]
pub struct TxProposer {
    connpool: PoolType,
}

impl TxProposer {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    pub async fn run(&self) {
        unimplemented!()
    }
}

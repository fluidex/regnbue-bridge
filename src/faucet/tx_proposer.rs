use crate::faucet::Settings;
use crate::storage::PoolType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TxProposer {
    connpool: PoolType,
    fundings: HashMap<String, String>, // TODO: use decimals
}

impl TxProposer {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        Self {
            connpool,
            fundings: config.fundings,
        }
    }

    pub async fn run(&self) {
        unimplemented!()
    }
}

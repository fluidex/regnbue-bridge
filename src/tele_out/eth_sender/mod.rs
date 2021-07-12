use crate::storage::PoolType;
use crate::tele_out::Settings;
use crossbeam_channel::Receiver;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
}

impl EthSender {
    pub fn from_config_with_pool(_config: &Settings, connpool: PoolType) -> Self {
        Self { connpool }
    }

    // TODO: use eth_tx type
    pub async fn run(&self, rx: Receiver<String>) {
        unimplemented!()
    }
}

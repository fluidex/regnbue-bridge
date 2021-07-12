use crate::storage::PoolType;
use crate::tele_out::Settings;
use crossbeam_channel::Receiver;
use web3::types::Address;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
}

impl EthSender {
    // TODO: error handling
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        let address = config.contract_address.parse::<Address>().unwrap();

        Self { connpool }
    }

    // TODO: use eth_tx type
    pub async fn run(&self, rx: Receiver<String>) {
        unimplemented!()
    }
}

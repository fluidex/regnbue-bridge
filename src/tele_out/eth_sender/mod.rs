use crate::storage::PoolType;
use crate::tele_out::Settings;
use crate::contracts;
use crossbeam_channel::Receiver;
use web3::types::Address;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
}

impl EthSender {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Result<Self, anyhow::Error> {
        let address = config.contract_address.parse::<Address>()?;
        let abi = contracts::get_abi(&config.contract_abi_file_path)?;

        Ok(Self { connpool })
    }

    pub async fn run(&self, rx: Receiver<super::ContractCall>) {
        for call in rx.iter() {
            log::debug!("{:?}", call);
            let action = match call {
                super::ContractCall::SubmitProof(data) => self.submit_proof(data),
            };
            if let Err(e) = action.await {
                log::error!("{:?}", e);
            };

            // TODO: save to db
        }
    }

    pub async fn submit_proof(&self, data: super::ProofData) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

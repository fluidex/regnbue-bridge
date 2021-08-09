use super::types::{ContractCall, SubmitBlockArgs};
use crate::contracts;
use crate::storage::PoolType;
use crate::tele_out::Settings;
use crossbeam_channel::Receiver;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::{Address, H256},
};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
    contract: Contract<Provider<Http>>,
    confirmations: usize,
}

impl EthSender {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Result<Self, anyhow::Error> {
        let address = config.contract_address.parse::<Address>()?;
        let abi: Abi = contracts::get_abi(&config.contract_abi_file_path)?;
        let client = Provider::<Http>::try_from(config.web3_url.as_str())?; // assume wallet inside
        let contract = Contract::new(address, abi, client);

        Ok(Self {
            connpool,
            contract,
            confirmations: config.confirmations,
        })
    }

    pub async fn run(&self, rx: Receiver<ContractCall>) {
        for call in rx.iter() {
            log::debug!("{:?}", call);
            if let Err(e) = match call {
                ContractCall::SubmitBlock(args) => self.submit_block(args).await,
            } {
                log::error!("{:?}", e);
                continue;
            };

            // TODO: mark block.status as commited
        }
    }

    pub async fn submit_block(&self, args: SubmitBlockArgs) -> Result<(), anyhow::Error> {
        let call = self
            .contract
            .method::<_, H256>("submitBlock", (args.block_id, args.public_inputs, args.serialized_proof))?;
        let pending_tx = call.send().await?;
        let receipt = pending_tx.confirmations(self.confirmations).await?;
        log::info!("block {:?} submitted. receipt: {:?}.", args.block_id, receipt);
        Ok(())
    }
}

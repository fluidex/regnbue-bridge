use super::types::{ContractCall, SubmitBlockArgs};
use crate::block_submitter::Settings;
use crate::contracts;
use crate::storage::PoolType;
use crossbeam_channel::Receiver;
use ethers::abi::Abi;
use ethers::prelude::*;
use fluidex_common::db::models;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
    account: Address,
    contract: Contract<Provider<Http>>,
    confirmations: usize,
}

impl EthSender {
    pub async fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Result<Self, anyhow::Error> {
        let address = config.contract_address.parse::<Address>()?;
        let abi: Abi = contracts::get_abi(&config.contract_abi_file_path)?;
        let client = Provider::<Http>::try_from(config.web3_url.as_str())?; // assume wallet inside
        let account = match config.account {
            None => client.get_accounts().await?[0],
            Some(ref addr) => addr.parse::<Address>()?,
        };

        let contract = Contract::new(address, abi, client);

        Ok(Self {
            connpool,
            account,
            contract,
            confirmations: config.confirmations,
        })
    }

    pub async fn run(&self, rx: Receiver<ContractCall>) {
        for call in rx.iter() {
            log::debug!("{:?}", call);
            match call {
                ContractCall::SubmitBlock(args) => {
                    if let Err(e) = self.submit_block(args.clone()).await {
                        log::error!("{:?}", e);
                        continue;
                    }

                    let stmt = format!("update {} set status = $1 where block_id = $2", models::tablenames::L2_BLOCK);
                    if let Err(e) = sqlx::query(&stmt)
                        .bind(models::l2_block::BlockStatus::Verified)
                        .bind(args.block_id.as_u64() as i64)
                        .execute(&self.connpool)
                        .await
                    {
                        log::error!("{:?}", e);
                    };
                }
            }
        }
    }

    pub async fn submit_block(&self, args: SubmitBlockArgs) -> Result<(), anyhow::Error> {
        let call = self
            .contract
            .method::<_, H256>("submitBlock", (args.block_id, args.public_inputs, args.serialized_proof))
            .unwrap()
            .from(self.account);
        // ganache does not support EIP-1559
        #[cfg(feature = "ganache")]
        let call = call.legacy();
        let pending_tx = call.send().await?;
        let receipt = pending_tx.confirmations(self.confirmations).await.unwrap();
        log::info!("block {:?} confirmed. receipt: {:?}.", args.block_id, receipt);
        Ok(())
    }
}

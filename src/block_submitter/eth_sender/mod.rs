use super::types::{ContractCall, SubmitBlockArgs};
use crate::block_submitter::Settings;
use crate::contracts;
use crate::storage::PoolType;
use crossbeam_channel::Receiver;
use ethers::abi::Abi;
use ethers::prelude::*;
use ethers::types::H256;
use fluidex_common::db::models;
use std::convert::TryFrom;

type SignedClient = SignerMiddleware<Provider<Http>, LocalWallet>;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
    client: SignedClient,
    account: Address,
    contract: Contract<SignedClient>,
    confirmations: usize,
}

impl EthSender {
    pub async fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Result<Self, anyhow::Error> {
        let address = config.contract_address.parse::<Address>()?;
        let abi: Abi = contracts::get_abi(&config.contract_abi_file_path)?;

        let client = Provider::<Http>::try_from(config.web3_url.as_str())?;
        let wallet = LocalWallet::decrypt_keystore(config.keystore.as_str(), config.password.as_str())?.with_chain_id(config.chain_id);
        let account = wallet.address();
        let client = SignerMiddleware::new(client, wallet);

        let contract = Contract::new(address, abi, client.clone());

        Ok(Self {
            connpool,
            client,
            account,
            contract,
            confirmations: config.confirmations,
        })
    }

    pub async fn run(&self, rx: Receiver<ContractCall>) {
        for call in rx.iter() {
            log::debug!("{:?}", call);
            if let Err(e) = self.run_inner(call).await {
                log::error!("{:?}", e);
            };
        }
    }

    async fn run_inner(&self, call: ContractCall) -> Result<(), anyhow::Error> {
        match call {
            ContractCall::SubmitBlock(args) => self.submit_block(args).await,
        }
    }

    pub async fn verify_block(&self, args: SubmitBlockArgs) -> Result<bool, anyhow::Error> {
        // println!("block aux {:02x?}", args.deposit_aux);
        let ret = self
            .contract
            .method::<_, bool>("verifyBlock", (args.public_inputs, args.serialized_proof, args.public_data))?
            .call()
            .await?;

        Ok(ret)
    }

    pub async fn submit_block(&self, args: SubmitBlockArgs) -> Result<(), anyhow::Error> {
        let call = self
            .contract
            .method::<_, H256>(
                "submitBlock",
                (args.block_id, args.public_inputs, args.serialized_proof, args.public_data),
            )?
            .from(self.account);
        // ganache does not support EIP-1559
        #[cfg(feature = "ganache")]
        let call = call.legacy();
        let pending_tx = call.send().await?;
        let receipt = pending_tx.confirmations(self.confirmations).await?;
        log::info!("block {:?} confirmed. receipt: {:?}.", args.block_id, receipt);

        // https://stackoverflow.com/questions/57350082/to-convert-a-ethereum-typesh256-to-string-in-rust
        let tx_hash_str = match receipt {
            Some(receipt) => format!("{:#x}", receipt.transaction_hash),
            None => String::default(),
        };

        let stmt = format!(
            "update {} set status = $1, l1_tx_hash = $2 where block_id = $3",
            models::tablenames::L2_BLOCK
        );
        sqlx::query(&stmt)
            .bind(models::l2_block::BlockStatus::Verified)
            .bind(tx_hash_str)
            .bind(args.block_id.as_u64() as i64)
            .execute(&self.connpool)
            .await?;

        Ok(())
    }
}

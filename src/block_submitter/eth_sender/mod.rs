use super::types::{ContractCall, SubmitBlockArgs};
use crate::block_submitter::Settings;
use crate::contracts;
use crate::storage::PoolType;
use async_trait::async_trait;
use crossbeam_channel::Receiver;
use ethers::abi::Abi;
use ethers::prelude::*;
use ethers::types::H256;
use fluidex_common::db::models;
use std::convert::TryFrom;
use std::{boxed, pin};

type SignedClient = SignerMiddleware<Provider<Http>, LocalWallet>;

#[async_trait]
pub trait EthSend: Sync + Send {
    async fn verify_block(&self, args: SubmitBlockArgs) -> Result<u64, anyhow::Error>;
    async fn submit_block(&self, args: SubmitBlockArgs) -> Result<(), anyhow::Error>;
    async fn run(&self, rx: Receiver<ContractCall>);
}

#[derive(Debug)]
pub struct EthSenderConfigure {
    abi: Abi,
    base_cli: Provider<Http>,
    address: Address,
    confirmations: usize,
    account: Address,
    wallet: Option<LocalWallet>,
}

impl EthSenderConfigure {
    pub async fn from_config(config: &Settings) -> Result<Self, anyhow::Error> {
        let address = config.contract_address.parse::<Address>()?;
        let abi: Abi = contracts::get_abi(&config.contract_abi_file_path)?;
        let base_cli = Provider::<Http>::try_from(config.web3_url.as_str())?;

        let wallet = if let Some(keystore) = &config.keystore {
            let psw = config.password.as_ref().map_or_else(Default::default, Clone::clone);
            let wallet = LocalWallet::decrypt_keystore(keystore.as_str(), &psw)?.with_chain_id(config.chain_id.unwrap_or(5u64)); //use goerli's chain as default
            Some(wallet)
        } else {
            None
        };

        let account = if let Some(wallet) = &wallet {
            wallet.address()
        } else {
            base_cli.get_accounts().await?[0]
        };

        Ok(Self {
            abi,
            address,
            base_cli,
            wallet,
            account,
            confirmations: config.confirmations,
        })
    }

    fn build_with_keystore(self, connpool: PoolType) -> EthSender<SignedClient> {
        let wallet = self.wallet.unwrap();
        let client = SignerMiddleware::new(self.base_cli, wallet);

        EthSender {
            connpool,
            account: self.account,
            contract: Contract::new(self.address, self.abi, client),
            confirmations: self.confirmations,
        }
    }

    fn build_with_account(self, connpool: PoolType) -> EthSender<Provider<Http>> {
        EthSender {
            connpool,
            account: self.account,
            contract: Contract::new(self.address, self.abi, self.base_cli),
            confirmations: self.confirmations,
        }
    }

    pub fn build(self, connpool: PoolType) -> pin::Pin<boxed::Box<dyn EthSend>> {
        if self.wallet.is_some() {
            boxed::Box::pin(self.build_with_keystore(connpool))
        } else {
            boxed::Box::pin(self.build_with_account(connpool))
        }
    }
}

#[derive(Debug)]
pub struct EthSender<M> {
    connpool: PoolType,
    //client: SignedClient,
    contract: Contract<M>,
    account: Address,
    confirmations: usize,
}

#[async_trait]
impl<M: Middleware> EthSend for EthSender<M> {
    async fn run(&self, rx: Receiver<ContractCall>) {
        for call in rx.iter() {
            log::debug!("{:?}", call);
            let ret = match call {
                ContractCall::SubmitBlock(args) => self.submit_block(args).await,
            };
            if let Err(e) = ret {
                log::error!("{:?}", e);
            };
        }
    }

    async fn verify_block(&self, args: SubmitBlockArgs) -> Result<u64, anyhow::Error> {
        // println!("block aux {:02x?}", args.deposit_aux);
        let ret = self
            .contract
            .method::<_, u64>(
                "verifyBlock",
                (args.public_inputs, args.serialized_proof, args.public_data, args.deposit_aux),
            )?
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("verify fail: {}", e))?;

        Ok(ret)
    }

    async fn submit_block(&self, args: SubmitBlockArgs) -> Result<(), anyhow::Error> {
        let call = self
            .contract
            .method::<_, H256>(
                "submitBlock",
                (
                    args.block_id,
                    args.public_inputs,
                    args.serialized_proof,
                    args.public_data,
                    args.deposit_aux,
                ),
            )?
            .from(self.account);
        // ganache does not support EIP-1559
        #[cfg(feature = "ganache")]
        let call = call.legacy();
        let pending_tx = call.send().await.map_err(|e| anyhow::anyhow!("submit fail: {}", e))?;
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

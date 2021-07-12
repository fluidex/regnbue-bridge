pub mod config;
pub mod eth_sender;
pub mod storage;
pub mod task_fetcher;

pub use config::Settings;
pub use eth_sender::EthSender;
pub use task_fetcher::TaskFetcher;

#[derive(Debug)]
pub enum ContractCall {
    SubmitProof,
}

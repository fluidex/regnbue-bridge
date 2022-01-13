use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub db: String,
    pub contract_address: String,
    pub contract_abi_file_path: String,
    pub confirmations: usize, // TODO: default
    pub web3_url: String,
    pub keystore: Option<String>,
    pub password: Option<String>,
    pub chain_id: Option<u64>,
}

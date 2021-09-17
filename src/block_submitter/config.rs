use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub db: String,
    pub contract_address: String,
    pub contract_abi_file_path: String,
    pub web3_url: String,     // TODO: default
    pub confirmations: usize, // TODO: default
    #[serde(default)]
    /// default using web3 first account
    pub account: Option<String>,
}

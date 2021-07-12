use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub db: String,
    pub contract_address: String,
    pub contract_abi_file_path: String,
}

#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum WrappedMessage {
    USER(UserMessage),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMessage {
    pub user_id: i32,
    pub l1_address: String,
    pub l2_pubkey: String,
}

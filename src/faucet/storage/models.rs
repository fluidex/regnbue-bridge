use crate::storage::{DecimalDbType, TimestampDbType};
use serde::Serialize;

pub mod tablenames {
    pub const FAUCET_TX: &str = "faucet_tx";
}

#[derive(sqlx::Type, Debug, Clone, Serialize)]
#[sqlx(type_name = "tx_status", rename_all = "snake_case")]
pub enum TxStatus {
    Proposed,
    Claimed,
    Sent,
    Confirmed,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct FaucetTx {
    pub id: i32,
    pub to_user: i32,
    pub asset: String,
    pub amount: DecimalDbType,
    pub status: TxStatus,
    pub created_time: TimestampDbType,
    pub updated_time: TimestampDbType,
}

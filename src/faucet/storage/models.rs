use crate::storage::{DecimalDbType, TimestampDbType};
use serde::Serialize;

pub mod tablenames {
    pub const INTERNAL_TX: &str = "internal_tx";
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
pub struct InternalTx {
    pub id: i32,
    pub to_user: i32,
    pub asset: String,
    pub amount: DecimalDbType,
    pub created_time: TimestampDbType,
    pub updated_time: TimestampDbType,
}

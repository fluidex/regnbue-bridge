use serde::{Deserialize, Serialize};
use crate::storage::{DecimalDbType, TimestampDbType};

pub mod tablenames {
    pub const InternalTx: &str = "internal_tx";
}

#[derive(sqlx::Type, Debug, Clone, Serialize)]
#[sqlx(type_name = "tx_status", rename_all = "snake_case")]
pub enum TxStatus {
    Proposed,
    Sent,
    Confirmed,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct InternalTx {
    pub id: TimestampDbType,
    pub to_user: i32,
    pub asset: String,
    pub amount: DecimalDbType,
    pub created_time: TimestampDbType,
    pub updated_time: TimestampDbType,
}

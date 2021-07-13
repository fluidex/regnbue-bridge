use ethers::types::U256;

#[derive(Debug)]
pub enum ContractCall {
    SubmitProof(ProofData),
}

#[derive(Debug)]
pub struct ProofData {
    pub block_id: U256,
    pub public_inputs: Vec<U256>,
    pub serialized_proof: Vec<U256>,
}

pub mod models {
    use serde::{Deserialize, Serialize};
    pub type TimestampDbType = chrono::NaiveDateTime;

    #[derive(sqlx::Type, Debug, Clone, Serialize)]
    #[sqlx(type_name = "task_status", rename_all = "snake_case")]
    pub enum TaskStatus {
        Inited,
        Witgening,
        Ready,
        Assigned,
        Proved,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, sqlx::Type)]
    #[sqlx(type_name = "varchar")]
    #[sqlx(rename_all = "lowercase")]
    pub enum CircuitType {
        BLOCK,
    }

    #[derive(sqlx::FromRow, Debug, Clone, Serialize)]
    pub struct Task {
        // pub id: i64,
        pub task_id: String,
        pub circuit: CircuitType,
        pub block_id: i64,
        pub input: serde_json::Value,
        pub output: Option<serde_json::Value>,
        pub witness: Option<Vec<u8>>,
        pub public_input: Option<Vec<u8>>,
        pub proof: Option<Vec<u8>>,
        pub status: TaskStatus,
        pub prover_id: Option<String>,
        pub created_time: TimestampDbType,
        pub updated_time: TimestampDbType,
    }
}

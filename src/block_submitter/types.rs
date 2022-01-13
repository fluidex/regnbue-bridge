use ethers::types::U256;

#[derive(Debug)]
pub enum ContractCall {
    SubmitBlock(SubmitBlockArgs),
}

#[derive(Debug, Clone)]
pub struct SubmitBlockArgs {
    pub block_id: U256,
    pub public_inputs: Vec<U256>,
    pub serialized_proof: Vec<U256>,
    pub public_data: Vec<u8>,
    pub deposit_aux: Vec<u8>,
}

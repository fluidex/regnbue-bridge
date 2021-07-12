use web3::types::U256;

#[derive(Debug)]
pub enum ContractCall {
    SubmitProof(ProofData),
}

#[derive(Debug)]
pub struct ProofData {
    block_id: U256,
    public_inputs: U256,
    serialized_proof: U256,
}

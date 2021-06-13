use crate::faucet::storage::models;
use crate::pb::matchengine_client::MatchengineClient;
use crate::pb::*;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct GrpcClient {
    pub upstream: String,
}

impl GrpcClient {
    pub async fn sent(&self, wallet_id: u32, tx: &models::InternalTx) -> Result<TransferResponse, anyhow::Error> {
        let mut client = MatchengineClient::connect(self.upstream.clone()).await?;

        let request = tonic::Request::new(TransferRequest {
            asset: tx.asset.clone(),
            delta: tx.amount.to_string(),
            from: wallet_id,
            to: tx.to_user as u32,
            memo: "fast_deposit".to_string(),
        });
        log::debug!("grpc_client sending tx (id: {:?})", tx.id);
        match client.transfer(request).await {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

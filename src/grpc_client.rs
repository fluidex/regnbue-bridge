use crate::faucet::storage::models;
use crate::pb::matchengine_client::MatchengineClient;
use crate::pb::*;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct GrpcClient {
    pub upstream: String,
}

impl GrpcClient {
    pub async fn fund(&self, tx: &models::InternalTx) -> Result<BalanceUpdateResponse, anyhow::Error> {
        let mut client = MatchengineClient::connect(self.upstream.clone()).await?;

        let request = tonic::Request::new(BalanceUpdateRequest {
            user_id: tx.to_user as u32,
            asset: tx.asset.clone(),
            business: "heimdallr_faucet".to_string(),
            business_id: tx.id as u64,
            delta: tx.amount.to_string(),
            detail: tx.created_time.to_string(),
        });
        log::debug!("grpc_client sending faucet_tx (id: {:?})", tx.id);
        match client.balance_update(request).await {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

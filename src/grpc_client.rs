use crate::faucet::storage::models;
use anyhow::anyhow;
use orchestra::rpc::exchange::*;

#[derive(Debug, Clone)]
pub struct GrpcClient {
    pub upstream: String,
}

impl GrpcClient {
    pub async fn fund(&self, tx: &models::FaucetTx) -> Result<BalanceUpdateResponse, anyhow::Error> {
        let mut client = matchengine_client::MatchengineClient::connect(self.upstream.clone()).await?;

        let request = tonic::Request::new(BalanceUpdateRequest {
            user_id: tx.to_user as u32,
            asset: tx.asset.clone(),
            business: "regnbue_bridge_faucet".to_string(),
            business_id: tx.id as u64,
            delta: tx.amount.to_string(),
            detail: serde_json::json!({"id": "", "faucet_tx time": tx.created_time}).to_string(),
        });
        log::debug!("grpc_client sending faucet_tx (id: {:?})", tx.id);
        match client.balance_update(request).await {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub async fn mock_transfer(&self, tx: &models::FaucetTx) -> Result<TransferResponse, anyhow::Error> {
        assert!(tx.to_user > 1);

        let mut client = matchengine_client::MatchengineClient::connect(self.upstream.clone()).await?;

        let request = tonic::Request::new(TransferRequest {
            from: tx.to_user as u32,
            to: (tx.to_user - 1) as u32,
            asset: tx.asset.clone(),
            delta: "0.1".to_string(),
            memo: serde_json::json!({"id": "", "mock_transfer_tx time": tx.created_time}).to_string(),
        });
        log::debug!("grpc_client sending transfer_tx (id: {:?})", tx.id);
        match client.transfer(request).await {
            Ok(resp) => Ok(resp.into_inner()),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

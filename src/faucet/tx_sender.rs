use crate::faucet::{storage::models, Settings};
use crate::grpc_client::GrpcClient;
use crate::storage::PoolType;
use anyhow::anyhow;
use std::time::Duration;

#[derive(Debug)]
pub struct TxSender {
    connpool: PoolType,
    send_interval: Duration,
    grpc_client: GrpcClient,
}

impl TxSender {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        Self {
            connpool,
            send_interval: config.send_interval(),
            grpc_client: GrpcClient {
                upstream: config.grpc_upstream.clone(),
            },
        }
    }

    pub async fn run(&self) {
        let mut timer = tokio::time::interval(self.send_interval);

        // TODO: use worker_pool for multiple workers
        loop {
            timer.tick().await;
            log::debug!("ticktock!");

            if let Err(e) = self.run_inner().await {
                log::error!("{}", e);
            };
        }
    }

    async fn run_inner(&self) -> Result<(), anyhow::Error> {
        let task = self.claim_one_task().await.map_err(|e| anyhow!("claim_one_task: {:?}", e))?;
        if task.is_none() {
            return Ok(());
        }
        let task = task.unwrap();

        self.grpc_client
            .fund(&task)
            .await
            .map_err(|e| anyhow!("grpc_client send tx: {:?}", e))?;

        self.mark_fund_sent(task.clone().id)
            .await
            .map_err(|_| anyhow!("mark_fund_sent in db"))?;

        log::info!(
            "({:?}) amout of asset ({:?}) funded to {:?} successfully!",
            task.amount,
            task.asset,
            task.to_user
        );

        Ok(())
    }

    async fn claim_one_task(&self) -> Result<Option<models::FaucetTx>, anyhow::Error> {
        let mut tx = self.connpool.begin().await?;

        let query = format!(
            "select id, to_user, asset, amount, status, created_time, updated_time
            from {}
            where status = $1 limit 1",
            models::tablenames::FAUCET_TX
        );

        let fetch_res = sqlx::query_as::<_, models::FaucetTx>(&query)
            .bind(models::TxStatus::Proposed)
            .fetch_optional(&mut tx)
            .await?;

        if let Some(ref t) = fetch_res {
            let stmt = format!("update {} set status = $1 where id = $2", models::tablenames::FAUCET_TX);
            sqlx::query(&stmt)
                .bind(models::TxStatus::Claimed)
                .bind(t.clone().id)
                .execute(&mut tx)
                .await?;
        };

        tx.commit().await?;
        Ok(fetch_res)
    }

    async fn mark_fund_sent(&self, id: i32) -> Result<(), anyhow::Error> {
        let stmt = format!("update {} set status = $1 where id = $2", models::tablenames::FAUCET_TX);
        sqlx::query(&stmt)
            .bind(models::TxStatus::Sent)
            .bind(id)
            .execute(&self.connpool)
            .await?;
        Ok(())
    }
}

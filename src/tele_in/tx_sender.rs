use crate::grpc_client::GrpcClient;
use crate::storage::PoolType;
use crate::tele_in::{storage::models, Settings};
use anyhow::anyhow;
use std::time::Duration;

#[derive(Debug, Clone)]
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

            if let Err(e) = self.clone().run_inner().await {
                log::error!("{}", e);
            };
        }
    }

    async fn run_inner(&self) -> Result<(), anyhow::Error> {
        let task = self.claim_one_task().await.map_err(|_| anyhow!("claim_one_task"))?;
        if task.is_none() {
            return Ok(());
        }
        let task = task.unwrap();

        // TODO: grpc send

        self.mask_tx_sent(task.clone().id)
            .await
            .map_err(|_| anyhow!("mask_tx_sent in db"))?;

        log::info!(
            "({:?}) amout of asset ({:?}) sent to {:?} successfully!",
            task.amount,
            task.asset,
            task.to_user
        );

        Ok(())
    }

    async fn claim_one_task(&self) -> Result<Option<models::InternalTx>, anyhow::Error> {
        let mut tx = self.connpool.begin().await?;

        let query = format!(
            "select id, to_user, asset, amount, created_time, updated_time
            from {}
            where status = $1 limit 1",
            models::tablenames::INTERNAL_TX
        );

        let fetch_res = sqlx::query_as::<_, models::InternalTx>(&query)
            .bind(models::TxStatus::Proposed)
            .fetch_optional(&mut tx)
            .await?;

        if let Some(ref t) = fetch_res {
            let stmt = format!("update {} set status = $1 where id = $2", models::tablenames::INTERNAL_TX);
            sqlx::query(&stmt)
                .bind(models::TxStatus::Claimed)
                .bind(t.clone().id)
                .execute(&mut tx)
                .await?;
        };

        tx.commit().await?;
        Ok(fetch_res)
    }

    async fn mask_tx_sent(&self, id: i32) -> Result<(), anyhow::Error> {
        let stmt = format!("update {} set status = $1 where id = $2", models::tablenames::INTERNAL_TX);
        sqlx::query(&stmt)
            .bind(models::TxStatus::Sent)
            .bind(id)
            .execute(&self.connpool)
            .await?;
        Ok(())
    }
}

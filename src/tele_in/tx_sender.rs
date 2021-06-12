use crate::grpc_client::GrpcClient;
use crate::storage::PoolType;
use crate::tele_in::Settings;
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
        unimplemented!()
    }
}

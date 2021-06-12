use crate::storage::PoolType;
use crate::tele_in::Settings;
use std::time::Duration;

#[derive(Debug)]
pub struct TxSender {
    connpool: PoolType,
    send_interval: Duration,
}

impl TxSender {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Self {
        Self {
            connpool,
            send_interval: config.send_interval(),
        }
    }

    pub fn run(&self) -> tokio::task::JoinHandle<()> {
        let mut timer = tokio::time::interval(self.send_interval);

        // TODO: use worker_pool for multiple workers
        tokio::spawn(async move {
            loop {
                timer.tick().await;
                log::debug!("ticktock!");
                if let Err(e) = self.clone().run_inner().await {
                    log::error!("{}", e);
                };
            }
        })
    }

    async fn run_inner(&self) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

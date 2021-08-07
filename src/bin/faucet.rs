use fluidex_common::non_blocking_tracing;
use futures::{channel::mpsc, executor::block_on, SinkExt, StreamExt};
use heimdallr::faucet::{storage, Settings, TxProposer, TxSender};
use std::cell::RefCell;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _guard = non_blocking_tracing::setup();
    log::info!("heimdallr faucet started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("FAUCET_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    // handle ctrl+c
    let (stop_signal_sender, mut stop_signal_receiver) = mpsc::channel(256);
    {
        let stop_signal_sender = RefCell::new(stop_signal_sender.clone());
        ctrlc::set_handler(move || {
            let mut sender = stop_signal_sender.borrow_mut();
            block_on(sender.send(true)).expect("crtlc signal send");
        })
        .expect("Error setting Ctrl-C handler");
    }

    let dbpool = storage::from_config(&settings).await?;

    let tx_proposer = TxProposer::from_config_with_pool(&settings, dbpool.clone());
    let tx_proposer_task_handle = tokio::spawn(async move { tx_proposer.run().await });

    let tx_sender = TxSender::from_config_with_pool(&settings, dbpool);
    let tx_sender_task_handle = tokio::spawn(async move { tx_sender.run().await });

    tokio::select! {
        _ = async { tx_proposer_task_handle.await } => {
            panic!("Tx Proposer actor is not supposed to finish its execution")
        },
        _ = async { tx_sender_task_handle.await } => {
            panic!("FaucetTx Sender actor is not supposed to finish its execution")
        },
        _ = async { stop_signal_receiver.next().await } => {
            log::warn!("Stop signal received, shutting down");
        }
    };

    Ok(())
}

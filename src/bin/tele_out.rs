use futures::{channel::mpsc, executor::block_on, SinkExt, StreamExt};
use heimdallr::tele_out::{storage, EthSender, Settings, TaskFetcher};
use std::cell::RefCell;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("heimdallr tele_out started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("TELE_OUT_CONFIG").unwrap();
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

    // TODO: maybe separate and have: 1. consumer 2. producer 3. sender

    let fetcher = TaskFetcher::from_config_with_pool(&settings, dbpool.clone());
    let fetcher_task_handle = fetcher.run();

    let eth_sender = EthSender::from_config_with_pool(&settings, dbpool);
    let eth_sender_task_handle = eth_sender.run();

    tokio::select! {
        _ = async { fetcher_task_handle.await } => {
            panic!("Tele_out task fetcher actor is not supposed to finish its execution")
        },
        _ = async { eth_sender_task_handle.await } => {
            panic!("Ethereum Sender actor is not supposed to finish its execution")
        },
        _ = async { stop_signal_receiver.next().await } => {
            log::warn!("Stop signal received, shutting down");
        }
    };

    Ok(())
}

use heimdallr::tele_out::storage;
use futures::{channel::mpsc, executor::block_on, SinkExt, StreamExt};
use std::cell::RefCell;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
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

    // TODO: init storage
    let _dbpool = storage::from_config().await?;

    tokio::select! {
        _ = async { stop_signal_receiver.next().await } => {
            log::warn!("Stop signal received, shutting down");
        }
    };

    Ok(())
}

use clap::Parser;
use fluidex_common::non_blocking_tracing;
use futures::{channel::mpsc, executor::block_on, SinkExt, StreamExt};
use regnbue_bridge::block_submitter::{storage, EthSender, Settings, TaskFetcher, types};
use std::cell::RefCell;

#[derive(Parser, Debug)]
#[clap(version = "0.1")]
struct Opts {
    #[clap(subcommand)]
    command: Option<SubCommand>
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Verify a block with specified block id
    Verify(VerifyBlock),
    /// manual submit a block
    Manual(ManualSubmit)
}

#[derive(Parser, Debug)]
struct VerifyBlock {
    block_id: i64
}

#[derive(Parser, Debug)]
struct ManualSubmit {
    /// Dry-run mode just verify the next submittion without really submit it
    #[clap(long = "dry-run")]
    dry_run: bool
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _guard = non_blocking_tracing::setup();
    log::info!("regnbue-bridge Block Submitter started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("BLOCK_SUBMITTER_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    let opts: Opts = Opts::parse();
    log::debug!("{:?}", opts);

    // TODO: maybe separate and have: 1. consumer 2. producer 3. sender
    let dbpool = storage::from_config(&settings).await?;
    let eth_sender = EthSender::from_config_with_pool(&settings, dbpool.clone()).await?;

    // one-block mode
    if let Some(sub_cmd) = opts.command {
        match sub_cmd {
            SubCommand::Verify(opts) => {
                let block_id = opts.block_id;
                let block = types::SubmitBlockArgs::fetch_by_blockid(block_id, &dbpool).await?;
                let ret = eth_sender.verify_block(block.ok_or_else(||anyhow::anyhow!("block {} not existed", block_id))?).await?;
                println!("verify block {} result: {}", block_id, ret);
            },
            SubCommand::Manual(opts) => {
                let block = types::SubmitBlockArgs::fetch_latest(None, &dbpool).await?;
                let block = block.ok_or_else(||anyhow::anyhow!("no pending block for submitting"))?;
                let block_id = block.block_id;
                if opts.dry_run {
                    let ret = eth_sender.verify_submitting(block).await?;
                    println!("verify submitting {} result: {}", block_id, ret);                    
                }else {
                    eth_sender.submit_block(block).await?;
                }
            },
        };

        return Ok(());
    }

    // continuous mode

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

    let (tx, rx) = crossbeam_channel::unbounded();
    let mut fetcher = TaskFetcher::from_config_with_pool(&settings, dbpool.clone());
    let fetcher_task_handle = tokio::spawn(async move { fetcher.run(tx).await });
    let eth_sender_task_handle = tokio::spawn(async move { eth_sender.run(rx).await });

    tokio::select! {
        _ = async { fetcher_task_handle.await } => {
            panic!("Block Submitter task fetcher actor is not supposed to finish its execution")
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

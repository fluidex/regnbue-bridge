use clap::Parser;
use ethers::abi::Abi;
use ethers::prelude::*;
use fluidex_common::non_blocking_tracing;
use sqlx::Connection;
use regnbue_bridge::storage::ConnectionType;
use regnbue_bridge::block_submitter::{Settings, types::SubmitBlockArgs};

#[derive(Parser, Debug)]
#[clap(version = "0.1")]
struct Opts {
    /// Specified block id to be verified, or use the latest, uncommited block
    #[clap(long = "block")]
    block_number: Option<u64>,
    verifier_addr: String,

    //TODO: add options for db, web3 and abi_file_path
}

//verifier has a stable interface so we can hard-code it here
static VERIFIERABI: &str = r#"
[
    {
      "inputs": [
        {
          "internalType": "uint256[]",
          "name": "public_inputs",
          "type": "uint256[]"
        },
        {
          "internalType": "uint256[]",
          "name": "serialized_proof",
          "type": "uint256[]"
        }
      ],
      "name": "verify_serialized_proof",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    }
]
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _guard = non_blocking_tracing::setup();
    log::info!("regnbue-bridge Block Verifier started");

    let opts: Opts = Opts::parse();
    log::debug!("{:?}", opts);

    let abi: Abi = serde_json::from_str(VERIFIERABI)?;
    let address = opts.verifier_addr.parse::<Address>()?;

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("BLOCK_SUBMITTER_CONFIG").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    let client = Provider::<Http>::try_from(settings.web3_url.as_str())?; // assume wallet inside
    let contract = Contract::new(address, abi, client);

    let mut conn = ConnectionType::connect(&settings.db).await?;

    let args = if let Some(block_id) = opts.block_number {
        SubmitBlockArgs::fetch_by_blockid(block_id as i64, &mut conn).await?
    }else {
        SubmitBlockArgs::fetch_latest(None, &mut conn).await?
    };

    if let Some(args) = args {
        log::debug!("public input of block is {:02x?}", args.public_data);
        let ret = contract.method::<_, bool>("verify_serialized_proof", (args.public_inputs, args.serialized_proof))?.call().await?;
        println!("verify block {} result: {}", args.block_id, ret);
        
    }else {
        return Err(anyhow::anyhow!("No matched block"));
    }
    

    Ok(())
}

use crate::block_submitter::Settings;
use crate::storage::{PoolOptions, PoolType};

pub async fn from_config(config: &Settings) -> anyhow::Result<PoolType> {
    let db_pool = PoolOptions::new().connect(&config.db).await?;

    Ok(db_pool)
}

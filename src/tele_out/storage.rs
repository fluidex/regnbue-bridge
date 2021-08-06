use crate::storage::{PoolOptions, PoolType};
use crate::tele_out::Settings;

pub async fn from_config(config: &Settings) -> anyhow::Result<PoolType> {
    let db_pool = PoolOptions::new().connect(&config.db).await?;

    Ok(db_pool)
}

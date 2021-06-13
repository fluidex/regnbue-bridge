use crate::faucet::Settings;
use crate::storage::{PoolOptions, PoolType};

pub mod models;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/faucet");

pub async fn from_config(config: &Settings) -> anyhow::Result<PoolType> {
    let db_pool = PoolOptions::new().connect(&config.db).await?;

    MIGRATOR.run(&db_pool).await?;

    Ok(db_pool)
}

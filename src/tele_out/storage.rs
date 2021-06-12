use crate::storage::{PoolOptions, PoolType};
use crate::tele_out::Settings;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/tele_out");

pub async fn from_config(config: &Settings) -> anyhow::Result<PoolType> {
    let db_pool = PoolOptions::new().connect(&config.db).await?;

    MIGRATOR.run(&db_pool).await?;

    Ok(db_pool)
}

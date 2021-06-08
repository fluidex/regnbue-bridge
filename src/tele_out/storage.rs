use crate::tele_out::Settings;

pub type DbType = sqlx::Postgres;
pub type ConnectionType = sqlx::postgres::PgConnection;
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
pub type DBErrType = sqlx::Error;

// TODO: config this filepath
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/tele_out");

pub async fn from_config(config: &Settings) -> anyhow::Result<sqlx::Pool<DbType>> {
    let db_pool = PoolOptions::new().connect(&config.db).await?;

    MIGRATOR.run(&db_pool).await?;

    Ok(db_pool)
}

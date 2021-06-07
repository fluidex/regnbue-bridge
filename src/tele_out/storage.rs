pub type DbType = sqlx::Postgres;
pub type ConnectionType = sqlx::postgres::PgConnection;
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
pub type DBErrType = sqlx::Error;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!(); // defaults to "./migrations"

pub async fn from_config() -> anyhow::Result<sqlx::Pool<DbType>> {
    unimplemented!();

    // MIGRATOR.run();
}

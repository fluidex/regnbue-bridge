pub type DbType = sqlx::Postgres;
pub type ConnectionType = sqlx::postgres::PgConnection;
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
pub type DBErrType = sqlx::Error;

pub async fn from_config() -> anyhow::Result<sqlx::Pool<DbType>> {
    unimplemented!();
}

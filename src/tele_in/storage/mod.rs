use crate::storage::{PoolOptions, PoolType};

pub mod models;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/tele_in");

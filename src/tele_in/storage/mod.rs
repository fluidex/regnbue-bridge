use crate::storage::{PoolOptions, PoolType};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations/tele_in");

use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub send_interval: u64,
    pub db: String,
    pub grpc_upstream: String,
}

impl Settings {
    /// Converts `self.interval` into `Duration`.
    pub fn send_interval(&self) -> Duration {
        Duration::from_millis(self.send_interval)
    }
}

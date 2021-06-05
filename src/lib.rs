#![allow(clippy::rc_buffer)]

pub use crate::subdomain::Runner;
use async_trait::async_trait;
use error::Result;
pub use postprocessor::{CleanExt, PostProcessor, PostProcessorIter};
use std::sync::Arc;
use tokio::sync::mpsc;

pub mod error;
pub mod postprocessor;
pub mod sources;
pub mod subdomain;

pub (crate) const QUEUE_SIZE: usize = 1024;

#[async_trait]
trait DataSource: Send + Sync {
    async fn run(&self, host: Arc<String>, mut tx: mpsc::Sender<Vec<String>>) -> Result<()>;
}

#[macro_export]
macro_rules! client {
    ($timeout:expr, $ptimeout:expr) => {
        reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs($timeout))
            .pool_idle_timeout(std::time::Duration::from_secs($ptimeout))
            .build()
            .unwrap()
    };
    () => {
        reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(20))
            .pool_idle_timeout(std::time::Duration::from_secs(20))
            .build()
            .unwrap()
    };
}
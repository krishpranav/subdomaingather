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
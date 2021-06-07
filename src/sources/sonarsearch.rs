use crate::error::{Result, SubError};
use crate::{DataSource, QUEUE_SIZE};
use async_trait::async_trait;
use crobat::Crobat;
use futures::StreamExt;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::{debug, info};

#[derive(Default, Clone)]
pub struct SonarSearch {
    client: Client,
}

impl SonarSearch {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
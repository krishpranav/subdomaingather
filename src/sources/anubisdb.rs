use crate::error::{Result, SubError};
use crate::{DataSource, IntoSubdomain};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::value::Value;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::{info, trace, warn};

struct AnubisResult {
    results: Value,
}

impl AnubisResult {
    fn new(results: Value) -> Self {
        Self { results }
    }
}

impl IntoSubdomain for AnubisResult {
    fn subdomains(&self) -> Sub<String> {
        match self.results.as_array() {
            Some(array) => array.iter().map(|s| s.to_string()).collect(),
            None => Sub::new(),
        }
    }
}

#[derive(Default, Clone)]
pub struct AnubisDB {
    client: Client,
}

impl AnubisDB {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    fn build_url(&self, host: &str) -> String {
        format!("https://jldc.me/anubis/subdomains/{}", host)
    }
}
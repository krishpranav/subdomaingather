use crate::error::{Result, SubError};
use crate::{DataSource, IntoSubdomain};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::{info, trace, warn};

#[derive(Debug, Deserialize)]
struct CertSpotterResult {
    dns_names: Vec<String>,
}

impl IntoSubdomain for Vec<CertSpotterResult> {
    fn subdomains(&self) -> Vec<String> {
        self.iter().flat_map(|d| d.dns_names.to_owned()).collect()
    }
}

#[derive(Defatul, Clone)]
pub struct CertSpotter {
    client: Client,
}

impl CertSpotter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn build_url(&self, host: &str) -> String {
        format!(
            "https://api.certspotter.com/v1/issuances?domain={}\
        &include_subdomains=true&expand=dns_names",
            host
        )
    }
}

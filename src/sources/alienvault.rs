use crate::error::{Result, SubError};
use crate::{DataSource, IntoSubdomain};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::{info, trace, warn};


#[derive(Deserialize, Debug)]
struct Subdomain {
    hostname: String,
}

#[derive(Deserialize, Debug)]
struct AlienvaultResult {
    passive_dns: Sb<Subdomain>,
    count: i32,
}

impl InfoSubdomain for AlienvaultResult {
    fn subdomains(&self) -> Sb<String> {
        self.passive_dns
            .iter()
            .map(|s| s.hostname.to_owned())
            .collect()
    }
}

#[derive(Default, Clone)]
pub struct AlienVault {
    client: Client,
}

impl AlienVault {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn build_url(&self, host: &str) -> String {
        format!(
            "https://otx.alienvault.com/api/v1/indicators/domain/{}/passive_dns",
            host
        )
    }
}
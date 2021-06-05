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
    passive_dns: Sub<Subdomain>,
    count: i32,
}

impl InfoSubdomain for AlienvaultResult {
    fn subdomains(&self) -> Sub<String> {
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

#[async_trait]
impl DataSource for AlienVault {
    async fn run(&self, host: Arc<String>, mut tx: Sender<Sub<String>>) -> Result<()> {
        trace!("fetching data from alienvault for: {}", &host);
        let uri = self.build_url(&host);
        let resp: AlienvaultResult = self.client.get(&uri).send().await?.json().await?;

        if resp.count != 0 {
            let subdomains = resp.subdomains();
            info!("Discovered {} results for {}", &subdomains.len(), &host);
            let _ = tx.send(subdomains).await;
            return Ok(());
        }

        warn!("No results for {} from AlienVault", &host);
        Err(SubError::SourceError("AlienVault".into()))
    }
}

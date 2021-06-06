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


#[async_trait]
impl DataSource for CertSpotter {
    async fn run(&self, host: Arc<String>, mut tx: Sender<Vec<String>>) -> Result<()> {
        trace!("fetching data from certspotter for: {}", &host);
        let uri = self.build_url(&host);
        let resp: Option<Vec<CertSpotterResult>> =
            self.client.get(&uri).send().await?.json().await?;

        if let Some(data) = resp {
            let subdomains = data.subdomains();
            if !subdomains.is_empty() {
                info!("Discovered {} results for: {}", &subdomains.len(), &host);
                let _ = tx.send(subdomains).await;
                return Ok(());
            }
        }

        warn!("no results for {} from CertSpotter", &host);
        Err(VitaError::SourceError("CertSpotter".into()))
    }
}

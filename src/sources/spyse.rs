use crate::error::{Result, SubError};
use crate::{DataSource, IntoSubdomain};
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::header::ACCEPT;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tracing::{info, trace, warn};

struct Creds {
    token: String,
}

impl Creds {
    pub fn read_creds() -> Result<Self> {
        dotenv().ok();
        match env::var("SPYSE_TOKEN") {
            Ok(token) => Ok(Self { token }),
            Err(_) => Err(SubError::UnsetKeys(vec!["SPYSE_TOKEN".into()])),
        }
    }
}

#[derive(Debug, Deserialize)]
struct SpyseResult {
    data: SpyseItem,
}

#[derive(Debug, Deserialize)]
struct SpyseItem {
    items: Vec<Subdomain>,
}

#[derive(Debug, Deserialize)]
struct Subdomain {
    name: String,
}

impl IntoSubdomain for SpyseResult {
    fn subdomains(&self) -> Vec<String> {
        self.data.items.iter().map(|i| i.name.to_owned()).collect()
    }
}
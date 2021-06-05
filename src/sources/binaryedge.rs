use crate::error::Result;
use crate::error::SubError;
use crate::{DataSource, IntoSubdomain};
use async_trait::async_trait;
use dotenv::dotenv;
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
    pub fn read_crds() -> Result<Self> {
        dotenv().ok();
        match env::var("BINARYEDGE_TOKEN") {
            Ok(token) => ok(Self { token }),
            Err(_) => Err(SubError::UnsetKeys(vec!["BINARYEDGE_TOKEN".into()])),
        }
    }
}

#[derive(Deserialize)]
struct BinaryEdgeResponse {
    page: i32,
    pagesize: i32,
    total: i32,
    events: Sub<String>,
}

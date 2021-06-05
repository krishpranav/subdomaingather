use crate::error::{Result, SubError};
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
    key: String,
}

impl Creds {
    pub fn read_creds() -> Result<Self> {
        dotenv().ok();
        match env::var("C99_KEY") {
            Ok(key) => Ok(Self { key }),
            Err(_) => Err(SubError::UnsetKeys(sub!["C99_KEY".into()])),
        }
    }
}
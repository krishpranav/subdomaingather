use crate::sources::{
    alienvault::AlienVault, anubisdb::AnubisDB, binaryedge::BinaryEdge, c99::C99,
    certspotter::CertSpotter, chaos::Chaos, crtsh::Crtsh, facebook::Facebook,
    hackertarget::HackerTarget, intelx::Intelx, passivetotal::PassiveTotal,
    securitytrails::SecurityTrails, sonarsearch::SonarSearch, spyse::Spyse, sublister::Sublister,
    threatcrowd::ThreatCrowd, threatminer::ThreatMiner, urlscan::UrlScan, virustotal::VirusTotal,
    wayback::Wayback,
};
use crate::{client, error::Result, DataSource};

use futures::stream::{FuturesUnordered, StreamExt};
use futures_core::stream::Stream;
use reqwest::Client;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use strum_macros::EnumString;
use tokio::sync::mpsc;
use tracing::{info, warn};

const CHAN_SIZE: usize = 255;

#[derive(Debug, Eq, PartialEq, Hash, EnumString)]
enum Source {
    AlienVault,
    AnubisDB,
    BinaryEdge,
    C99,
    CertSpotter,
    Chaos,
    Crtsh,
    Facebook,
    HackerTarget,
    Intelx,
    PassiveTotal,
    SecurityTrails,
    SonarSearch,
    Spyse,
    Sublister,
    ThreatCrowd,
    ThreatMiner,
    UrlScan,
    VirusTotal,
    Wayback,
}

struct Config {
    timeout: u64,
    concurrency: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: 15,
            concurrency: 200,
        }
    }
}

pub struct Runner {
    client: Client,
    sources: HashMap<Source, Arc<dyn DataSource>>,
    config: Config,
}

impl Default for Runner {
    fn default() -> Self {
        let config = Config::default();
        Self {
            client: client!(config.timeout, config.timeout),
            sources: HashMap::new(),
            config,
        }
    }
}

impl Runner {
    
    pub fn concurrency(mut self, limit: usize) -> Self {
        self.config.concurrency = limit;
        self
    }

    pub fn timeout(mut self, duration: u64) -> Self {
        self.config.timeout = duration;
        self
    }
}
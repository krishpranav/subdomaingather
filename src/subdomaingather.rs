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

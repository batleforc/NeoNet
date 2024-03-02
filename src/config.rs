use serde::{Deserialize, Serialize};

use crate::peers::peers_kind::Peer;

const PORT: &str = "PORT";
const HOSTNAME: &str = "HOSTNAME";
const RUST_ENV: &str = "RUST_ENV";

const OTEL_ENDPOINT: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub port: u16,
    pub hostname: String,
    pub env: String,
    pub peers: Vec<Peer>,
    pub tracing_sub: Vec<Peer>,
}

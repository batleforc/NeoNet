use serde::{Deserialize, Serialize};
use tracing::Level;

use super::level::VerboseLevel;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TracingKind {
    File,
    Console,
    Otel,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Tracing {
    pub kind: TracingKind,
    pub name: String,
    pub level: VerboseLevel,
}

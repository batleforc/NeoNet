use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Debug;
use tracing::Level;

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum VerboseLevel {
    ERROR = 4,
    WARN = 3,
    INFO = 2,
    DEBUG = 1,
    TRACE = 0,
}

impl From<VerboseLevel> for Level {
    fn from(level: VerboseLevel) -> Self {
        match level {
            VerboseLevel::ERROR => Level::ERROR,
            VerboseLevel::WARN => Level::WARN,
            VerboseLevel::INFO => Level::INFO,
            VerboseLevel::DEBUG => Level::DEBUG,
            VerboseLevel::TRACE => Level::TRACE,
        }
    }
}

impl Debug for VerboseLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ERROR => write!(f, "ERROR"),
            Self::WARN => write!(f, "WARN"),
            Self::INFO => write!(f, "INFO"),
            Self::DEBUG => write!(f, "DEBUG"),
            Self::TRACE => write!(f, "TRACE"),
        }
    }
}

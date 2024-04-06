use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PeerKind {
    Friend,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Peer {
    pub kind: PeerKind,
    pub hostname: String,
}

impl Peer {
    pub fn get_hostname_without_protocol(&self) -> String {
        match self.is_https() {
            true => self.hostname.replace("https://", ""),
            false => self.hostname.replace("http://", ""),
        }
        .replace('/', "")
    }
    pub fn is_https(&self) -> bool {
        self.hostname.starts_with("https://")
    }
}

use serde::{Deserialize, Serialize};

use crate::net::NetIdentity;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PeerId([u8; 32]);
impl PeerId {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8; 32]> for PeerId {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<&str> for PeerId {
    fn from(value: &str) -> Self {
        Self(blake3::hash(value.as_bytes()).into())
    }
}

impl From<[u8; 32]> for PeerId {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

#[derive(Clone)]
pub struct Peer {
    pub id: PeerId,
    pub identity: NetIdentity,
    pub address: String,
}

impl Peer {
    pub fn new(identity: NetIdentity, address: String) -> Self {
        Self { id: identity.peer_id(), identity, address }
    }
}
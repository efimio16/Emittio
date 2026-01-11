use serde::{Deserialize, Serialize};

use crate::net::types::NetIdentity;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PeerId(pub [u8; 32]);
impl PeerId {
    pub fn new(id: &str) -> Self {
        Self(*blake3::hash(id.as_bytes()).as_bytes())
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
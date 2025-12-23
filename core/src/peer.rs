// use tokio::sync::mpsc;
// use bytes::{Bytes};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PeerId(pub String);
impl PeerId {
    pub fn new(id: &str) -> Self {
        PeerId(id.into())
    }
}

// pub struct Peer {
//     pub id: PeerId,
//     pub address: String,
// }

// impl Peer {
//     pub fn new(id: PeerId, address: String) -> Self {
//         Peer { id, address }
//     }
// }
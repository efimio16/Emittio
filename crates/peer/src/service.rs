use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{peer::{Peer, PeerId, PeerTableCmd, PeerTableDispatcher, PeerTableError}, service::Service, utils::reply};

const CHAN_SIZE: usize = 100;

pub struct PeerTable {
    rx: mpsc::Receiver<PeerTableCmd>,
    peers: HashMap<PeerId, Peer>,
    peer_ids: Vec<PeerId>,
}
impl PeerTable {
    pub fn new() -> (Self, PeerTableDispatcher) {
        let (tx, rx) = mpsc::channel(CHAN_SIZE);
        (Self { rx, peers: HashMap::new(), peer_ids: Vec::new() }, PeerTableDispatcher { tx })
    }
}

impl Service for PeerTable {
    type Error = PeerTableError;

    async fn run(mut self, token: CancellationToken) -> Result<(), PeerTableError> {
        println!("Running peer table");

        loop {
            tokio::select! {
                _ = token.cancelled() => { return Ok(()); }
                Some(cmd) = self.rx.recv() => {
                    match cmd {
                        PeerTableCmd::GetAllIds { tx } => reply(tx, self.peer_ids.clone())?,
                        PeerTableCmd::GetPeer { peer_id, tx } => reply(tx, self.peers.get(&peer_id).cloned())?,
                        PeerTableCmd::AddPeer { peer } => { self.peer_ids.push(peer.id); self.peers.insert(peer.id, peer); },
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio_util::sync::CancellationToken;

    use crate::{net::NetIdentity, peer::{Peer, PeerTable}, service::Service};

    #[tokio::test]
    async fn test() {
        let (table, dispatcher) = PeerTable::new();

        tokio::spawn(table.run(CancellationToken::new()));

        let peer_id = "test".into();
        let peer = Peer { id: peer_id, address: "0.0.0.0".into(), identity: NetIdentity { x_pk: [1u8; 32], kb_pk: [1u8; 800] } };

        dispatcher.add_peer(peer).await.expect("add peer failed");

        assert_eq!(dispatcher.get_peer(peer_id).await.expect("get peer failed").expect("peer not found").id, peer_id);

        assert_eq!(dispatcher.get_all_ids().await.expect("get all ids failed"), vec![peer_id]);
    }
}
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

use crate::{channels::ChannelError, peer::{Peer, PeerId}, service::Service};

#[derive(Debug, Error)]
pub enum PeerTableError {
    #[error(transparent)]
    Channel(#[from] ChannelError),
}

enum PeerTableCmd {
    GetAllIds { reply_tx: oneshot::Sender<Vec<PeerId>> },
    GetPeer { peer_id: PeerId, reply_tx: oneshot::Sender<Option<Peer>> },
    AddPeer { peer: Peer },
}

#[derive(Clone)]
pub struct PeerTableDispatcher {
    tx: mpsc::Sender<PeerTableCmd>,
}

impl PeerTableDispatcher {
    pub async fn get_all_ids(&self) -> Result<Vec<PeerId>, ChannelError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(PeerTableCmd::GetAllIds { reply_tx: tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)
    }
    pub async fn get_peer(&self, peer_id: PeerId) -> Result<Option<Peer>, ChannelError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(PeerTableCmd::GetPeer { peer_id, reply_tx: tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)
    }
    pub async fn add_peer(&self, peer: Peer) -> Result<(), ChannelError> {
        self.tx.send(PeerTableCmd::AddPeer { peer }).await.map_err(|_| ChannelError::Closed)
    }
}

pub struct PeerTable {
    rx: mpsc::Receiver<PeerTableCmd>,
    peers: HashMap<PeerId, Peer>,
    peer_ids: Vec<PeerId>,
}
impl PeerTable {
    pub fn new() -> (Self, PeerTableDispatcher) {
        let (tx, rx) = mpsc::channel(100);
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
                        PeerTableCmd::GetAllIds { reply_tx } => reply_tx.send(self.peer_ids.clone()).map_err(|_| ChannelError::Closed)?,
                        PeerTableCmd::GetPeer { peer_id, reply_tx } => reply_tx.send(self.peers.get(&peer_id).cloned()).map_err(|_| ChannelError::Closed)?,
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

    use crate::{net::types::NetIdentity, peer::{Peer, PeerId}, peer_table::PeerTable, service::Service};

    #[tokio::test]
    async fn test() {
        let (table, dispatcher) = PeerTable::new();

        tokio::spawn(async move { table.run(CancellationToken::new()).await.expect("Something happened")});

        let peer_id = PeerId::new("test");
        let peer = Peer { id: peer_id, address: "0.0.0.0".into(), identity: NetIdentity { x_pk: [1u8; 32], kb_pk: [1u8; 800] } };

        dispatcher.add_peer(peer).await.expect("add peer failed");

        assert_eq!(dispatcher.get_peer(peer_id).await.expect("get peer failed").expect("peer not found").id, peer_id);

        assert_eq!(dispatcher.get_all_ids().await.expect("get all ids failed"), vec![peer_id]);
    }
}
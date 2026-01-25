use tokio::sync::{mpsc, oneshot};

use crate::{peer::{Peer, PeerId}, utils::ChannelError};

pub(super) enum PeerTableCmd {
    GetAllIds { reply_tx: oneshot::Sender<Vec<PeerId>> },
    GetPeer { peer_id: PeerId, reply_tx: oneshot::Sender<Option<Peer>> },
    AddPeer { peer: Peer },
}

#[derive(Clone)]
pub struct PeerTableDispatcher {
    pub(super) tx: mpsc::Sender<PeerTableCmd>,
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
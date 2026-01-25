use tokio::sync::{mpsc, oneshot};
use heapless::Vec;

use crate::{dht::CID, peer::PeerId, utils::ChannelError};

pub(super) const MAX_PEERS: usize = 8;

pub(super) enum DhtRoutingCmd {
    ClosestPeers {
        cid: CID,
        reply: oneshot::Sender<Vec<PeerId, MAX_PEERS>>,
    }
}

#[derive(Clone)]
pub struct DhtRoutingDispatcher {
    pub(super) tx: mpsc::Sender<DhtRoutingCmd>,
}

impl DhtRoutingDispatcher {
    pub async fn closest_peers(&mut self, cid: CID) -> Result<Vec<PeerId, MAX_PEERS>, ChannelError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(DhtRoutingCmd::ClosestPeers { cid, reply: tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)
    }
}
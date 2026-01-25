use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

use crate::{dht::CID, utils::ChannelError};

pub enum DhtStorageCmd {
    Get {
        cid: CID,
        reply: oneshot::Sender<Option<Bytes>>,
    },
    Put {
        content: Bytes,
        cid: CID,
    },
}

#[derive(Clone)]
pub struct DhtStorageDispatcher {
    pub(super) tx: mpsc::Sender<DhtStorageCmd>,
}

impl DhtStorageDispatcher {
    pub async fn put(&self, cid: CID, content: Bytes) -> Result<(), ChannelError> {
        self.tx.send(DhtStorageCmd::Put { cid, content }).await.map_err(|_| ChannelError::Closed)
    }
    pub async fn get(&mut self, cid: CID) -> Result<Option<Bytes>, ChannelError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(DhtStorageCmd::Get { cid, reply: tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)
    }
}
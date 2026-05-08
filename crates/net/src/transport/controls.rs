use tokio::sync::mpsc;

use crate::{message::{Message}, utils::ChannelError};

const CHAN_SIZE: usize = 128;

pub struct TransportDispatcher {
    pub(super) rx: mpsc::Receiver<Message>,
    pub(super) tx: mpsc::Sender<Message>,
}

pub struct TransportHandler {
    pub(super) rx: mpsc::Receiver<Message>,
    pub(super) tx: mpsc::Sender<Message>,
}

impl TransportDispatcher {
    pub async fn send(&self, msg: Message) -> Result<(), ChannelError> {
        self.tx.send(msg).await.map_err(|_| ChannelError::Closed)
    }
    pub async fn recv(&mut self) -> Option<Message> {
        self.rx.recv().await
    }
}

impl TransportHandler {
    pub fn new() -> (Self, TransportDispatcher) {
        let (send_tx, send_rx) = mpsc::channel(CHAN_SIZE);
        let (recv_tx, recv_rx) = mpsc::channel(CHAN_SIZE);
        (Self { rx: send_rx, tx: recv_tx }, TransportDispatcher { rx: recv_rx, tx: send_tx })
    }
    pub async fn send(&self, msg: Message) -> Result<(), ChannelError> {
        self.tx.send(msg).await.map_err(|_| ChannelError::Closed)
    }
    pub async fn recv(&mut self) -> Option<Message> {
        self.rx.recv().await
    }
}
use tokio::sync::mpsc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("channel is closed")]
    Closed,
}

pub struct Participant<R, T> {
    pub rx: mpsc::Receiver<R>,
    pub tx: mpsc::Sender<T>,
}

impl<R, T> Participant<R, T> {
    pub async fn send(&self, value: T) -> Result<(), ChannelError> {
        self.tx.send(value).await.map_err(|_| ChannelError::Closed)
    }
    pub async fn recv(&mut self) -> Option<R> {
        self.rx.recv().await
    }
}

pub fn new<R, T>(size: usize) -> (Participant<R, T>, Participant<T, R>) {
    let (send_tx, send_rx) = mpsc::channel(size);
    let (recv_tx, recv_rx) = mpsc::channel(size);

    (Participant { rx: send_rx, tx: recv_tx }, Participant { rx: recv_rx, tx: send_tx })
}
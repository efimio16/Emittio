use futures::{TryFutureExt, future::MapErr};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("Channel closed unexpectedly")]
    Closed,
}

#[inline]
pub async fn send<T>(chan: &mpsc::Sender<T>, data: T) -> Result<(), ChannelError> {
    chan.send(data).await.map_err(|_| ChannelError::Closed)
}

#[inline]
pub fn create_oneshot<T>() -> (oneshot::Sender<T>, MapErr<oneshot::Receiver<T>, impl FnOnce(oneshot::error::RecvError) -> ChannelError>) {
    let (tx, rx) = oneshot::channel();
    (tx, rx.map_err(|_| ChannelError::Closed))
}

#[inline]
pub fn reply<T>(chan: oneshot::Sender<T>, data: T) -> Result<(), ChannelError> {
    chan.send(data).map_err(|_| ChannelError::Closed)
}

#[macro_export]
macro_rules! ok_or_reply {
    ($err:ty, $tx:expr, $block:block) => {
        let mut inner = async || -> Result<(), $err> { $block };

        match inner().await {
            Ok(success) => success,
            Err(err) => {
                $crate::channel::reply($tx, Err(err)).ok();
                return;
            }
        }
    };
}
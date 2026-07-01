use tokio::{sync::{mpsc, oneshot}, task::JoinSet};
use tokio_util::sync::CancellationToken;
use futures::{TryFutureExt, future::MapErr};
use thiserror::Error;
use tokio_util::task::JoinMap;

pub use tokio;
pub use tokio_util;
pub use actorify_macro::actor;

pub type ActorJoinMap<K> = JoinMap<K, ()>;
pub type ActorJoinSet = JoinSet<()>;

pub trait Actor {
    type Handle;

    fn run(self, token: CancellationToken) -> (Self::Handle, impl Future<Output = ()> + Send);
    fn spawn(self) -> Self::Handle
    where
        Self: Sized + Send + 'static
    {
        let (handle, future) = self.run(CancellationToken::new());
        
        tokio::spawn(future);

        handle
    }
}

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("Channel closed unexpectedly")]
    Closed,
}

pub struct Callback<T>(oneshot::Sender<T>);

impl<T> Callback<T> {
    #[inline]
    pub fn new() -> (Self, MapErr<oneshot::Receiver<T>, impl FnOnce(oneshot::error::RecvError) -> ChannelError>) {
        let (tx, rx) = oneshot::channel();
        (Self(tx), rx.map_err(|_| ChannelError::Closed))
    }

    #[inline]
    pub fn send(self, data: T) -> Result<(), ChannelError> {
        self.0.send(data).map_err(|_| ChannelError::Closed)
    }
}

pub struct Channel<T>(mpsc::Sender<T>);

impl<T> Channel<T> {
    #[inline]
    pub fn new(size: usize) -> (Self, mpsc::Receiver<T>) {
        let (tx, rx) = mpsc::channel(size);

        (Self(tx), rx)
    }

    #[inline]
    pub async fn send(&self, data: T) -> Result<(), ChannelError> {
        self.0.send(data).await.map_err(|_| ChannelError::Closed)
    }
}

impl<T> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[macro_export]
macro_rules! ok_or_reply {
    ($cb:expr, $block:block) => {
        let mut inner = async || { $block };

        match inner().await {
            Ok(success) => success,
            Err(err) => {
                $cb.send(Err(err)).ok();
                return;
            }
        }
    };
}
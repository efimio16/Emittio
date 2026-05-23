pub use tokio_util::sync::CancellationToken;
pub use tokio::{sync::{mpsc, oneshot}, select};
pub use actor_macro::actor;

use futures::{TryFutureExt, future::MapErr};
use thiserror::Error;

// I don't think we'll need a dynamic orchestration of services,
// so let's try without this enum but keeping it just in case

// use crate::{client::ClientServiceError, dht::{DhtRoutingError, DhtStorageError}, net::SessionManagerServiceError, node::NodeError, peer::PeerTableError, tag::TagServiceError, transport::MockTransportError, utils::ChannelError};

// #[derive(Debug, Error)]
// pub enum ServiceError {
//     #[error(transparent)]
//     Join(#[from] JoinError),
    
//     #[error(transparent)]
//     MockTransport(#[from] MockTransportError),
    
//     #[error(transparent)]
//     TagService(#[from] TagServiceError),

//     #[error(transparent)]
//     PeerTable(#[from] PeerTableError),

//     #[error(transparent)]
//     ClientService(#[from] ClientServiceError),

//     #[error(transparent)]
//     Channel(#[from] ChannelError),

//     #[error(transparent)]
//     DhtStorage(#[from] DhtStorageError),

//     #[error(transparent)]
//     DhtRouting(#[from] DhtRoutingError),

//     #[error(transparent)]
//     Node(#[from] NodeError),

//     #[error(transparent)]
//     NetSession(#[from] SessionManagerServiceError),
// }

// pub struct ServiceManager {
//     services: JoinSet<Result<(), ServiceError>>,
//     token: CancellationToken,
// }

// impl ServiceManager {
//     pub fn new() -> Self {
//         Self { services: JoinSet::new(), token: CancellationToken::new() }
//     }

//     pub fn spawn<S: Service + 'static + Send>(&mut self, service: S)
//     where ServiceError: From<<S as Service>::Error> {
//         let token = self.token.clone();
//         self.services.spawn(async move { service.run(token).await?; Ok(()) });
//     }

//     pub fn run(mut self) -> (JoinHandle<Result<(), ServiceError>>, CancellationToken) {
//         println!("Running all services");

//         let token = self.token.clone();

//         (tokio::spawn(async move {
//             let Some(res) = self.services.join_next().await else { return Ok(()); };

//             token.cancel();
//             let _ = self.services.join_all().await;

//             res?
//         }), self.token)
//     }
// }

pub trait Service {
    fn run(self, token: CancellationToken) -> impl Future<Output = Result<(), ChannelError>> + Send;
}

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("Channel closed unexpectedly")]
    Closed,
}

pub struct Callback<T, E>(oneshot::Sender<Result<T, E>>);

impl<T, E> Callback<T, E> {
    #[inline]
    pub fn new() -> (Self, MapErr<oneshot::Receiver<Result<T, E>>, impl FnOnce(oneshot::error::RecvError) -> ChannelError>) {
        let (tx, rx) = oneshot::channel();
        (Self(tx), rx.map_err(|_| ChannelError::Closed))
    }

    #[inline]
    pub fn ok(self, data: T) -> Result<(), ChannelError> {
        self.0.send(Ok(data)).map_err(|_| ChannelError::Closed)
    }

    #[inline]
    pub fn err(self, err: E) -> Result<(), ChannelError> {
        self.0.send(Err(err)).map_err(|_| ChannelError::Closed)
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

pub struct ResultChannel<T, E>(mpsc::Sender<Result<T, E>>);

impl<T, E> ResultChannel<T, E> {
    #[inline]
    pub fn new(size: usize) -> (Self, mpsc::Receiver<Result<T, E>>) {
        let (tx, rx) = mpsc::channel(size);

        (Self(tx), rx)
    }

    #[inline]
    pub async fn ok(&self, data: T) -> Result<(), ChannelError> {
        self.0.send(Ok(data)).await.map_err(|_| ChannelError::Closed)
    }

    #[inline]
    pub async fn err(&self, err: E) -> Result<(), ChannelError> {
        self.0.send(Err(err)).await.map_err(|_| ChannelError::Closed)
    }
}

#[macro_export]
macro_rules! ok_or_reply {
    ($err:ty, $cb:expr, $block:block) => {
        let mut inner = async || -> Result<(), $err> { $block };

        match inner().await {
            Ok(success) => success,
            Err(err) => {
                $cb.err(err).ok();
                return;
            }
        }
    };
}
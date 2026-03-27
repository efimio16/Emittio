use thiserror::Error;
use tokio::{task::JoinError,sync::mpsc::error::{SendError},net::tcp::OwnedReadHalf};

use crate::{net::{SessionManagerDispatcherError,CryptoError,NetError,Message}, utils::ChannelError,peer::{PeerId}};

#[derive(Debug, Error)]
pub enum MockTransportError {
    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    NetSession(#[from] SessionManagerDispatcherError),

    #[error("address not found")]
    AddressNotFound,

    #[error("peer not found")]
    PeerNotFound,
}

#[derive(Debug, Error)]
pub enum TransportError {
    // Make PeerId optional so that callers can add information without having to pass PeerId down to every function
    #[error("session's read task has been cancelled")]
    ReadCancelled(OwnedReadHalf,Option<PeerId>),
    
    #[error("request was cancelled by token")]
    Cancelled,

    // Consider returning peer ID as data in this error
    #[error("session not found")]
    SessionNotFound(Option<PeerId>),

    // Consider returning peer ID as data in this error
    #[error("connection not found in active map")]
    ConnectionNotInMap(Option<PeerId>),

    // Consider returning peer ID as data in this error
    #[error("no sessions available")]
    NoSessions,

    // Consider returning peer ID as data in this error
    #[error("message channel has been closed")]
    MessageChannelClosed,

    #[error("peer already connected")]
    PeerAlreadyConnected(Option<PeerId>),

    #[error("connection closed")]
    ConnectionClosed(Option<PeerId>),

    #[error(transparent)]
    Serialization(#[from] postcard::Error),

    #[error(transparent)]
    Reading(#[from] SendError::<(PeerId,Message)>),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Join(#[from] JoinError),

    #[error(transparent)]
    Encrypt(#[from] CryptoError),

    #[error(transparent)]
    Decrypt(#[from] NetError),
}
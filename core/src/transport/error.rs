use thiserror::Error;
use tokio::task::JoinError;

use crate::{net::{SessionManagerDispatcherError,CryptoError,NetError}, utils::ChannelError};

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
    #[error("request was cancelled by token")]
    Cancelled,

    // Consider returning peer ID as data in this error
    #[error("session not found")]
    SessionNotFound,

    // Consider returning peer ID as data in this error
    #[error("no sessions available")]
    NoSessions,

    #[error("peer already connected")]
    PeerAlreadyConnected,

    #[error("connection closed")]
    ConnectionClosed,

    #[error(transparent)]
    Serialization(#[from] postcard::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Join(#[from] JoinError),

    #[error(transparent)]
    Encrypt(#[from] CryptoError),

    #[error(transparent)]
    Decrypt(#[from] NetError),
}
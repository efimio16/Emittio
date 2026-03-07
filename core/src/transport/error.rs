use thiserror::Error;
use tokio::task::JoinError;

use crate::{net::{CryptoError, NetError}, utils::{ChannelError, SerdeError},peer::{PeerId}};

#[derive(Debug, Error)]
pub enum MockTransportError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Net(#[from] NetError),

    #[error("client not found")]
    ClientNotFound,

    #[error("sessions not found")]
    SessionsNotFound,

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

    #[error("peer already connected")]
    PeerAlreadyConnected,

    #[error(transparent)]
    Serialization(#[from] postcard::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Join(#[from] JoinError),
}
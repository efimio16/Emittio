use thiserror::Error;

use crate::{net::{CryptoError, NetError}, utils::{ChannelError, SerdeError}};

#[derive(Debug, Error)]
pub enum SessionManagerServiceError {
    #[error(transparent)]
    Channel(#[from] ChannelError),
}

#[derive(Debug, Error)]
pub enum SessionManagerDispatcherError {
    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Net(#[from] NetError),

    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error("session not found")]
    SessionNotFound,

    #[error("peer id not found")]
    PeerIdNotFound,

    #[error("missing conn id")]
    MissingConnId,
}
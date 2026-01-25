use thiserror::Error;

use crate::{net::{CryptoError, NetError}, utils::{ChannelError, SerdeError}};

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
use thiserror::Error;

use crate::{net::SessionManagerDispatcherError, utils::ChannelError};

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
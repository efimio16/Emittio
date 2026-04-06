use thiserror::Error;

use crate::{peer::PeerTableError, utils::ChannelError};

#[derive(Debug, Error)]
pub enum BucketsError {
    #[error("Bucket overflow")]
    Overflow,
}

#[derive(Debug, Error)]
pub enum DhtRoutingError {
     #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    PeerTable(#[from] PeerTableError),

    #[error(transparent)]
    Buckets(#[from] BucketsError),
}
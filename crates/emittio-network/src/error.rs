use emittio_crypto::error::CryptoError;
use actorify::ChannelError;
use thiserror::Error;

use crate::peer::PeerId;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error("invalid seq")]
    InvalidSeq,

    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    Postcard(#[from] postcard::Error),

    #[error("peer not found: {0:?}")]
    PeerNotFound(PeerId),
}
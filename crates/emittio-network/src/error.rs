use emittio_crypto::error::CryptoError;
use actorify::ChannelError;
use thiserror::Error;

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
}
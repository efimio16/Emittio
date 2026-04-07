use crypto::error::CryptoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetError {
    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error("invalid seq")]
    InvalidSeq,

    #[error("ephemeral client cannot accept handshakes or have static identity")]
    EphemeralClient,
}
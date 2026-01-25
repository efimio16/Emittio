use thiserror::Error;
use pqc_kyber::KyberError;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error(transparent)]
    Kyber(#[from] KyberError),

    #[error("invalid shared key")]
    InvalidSharedKey,

    #[error("encryption failed")]
    AesGcmEncryption(aes_gcm::Error),

    #[error("decryption failed")]
    AesGcmDecryption(aes_gcm::Error),

    #[error("ephemeral client cannot accept handshakes or have static identity")]
    EphemeralClient,
}

#[derive(Debug, Error)]
pub enum NetError {
    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error("invalid seq")]
    InvalidSeq,
}
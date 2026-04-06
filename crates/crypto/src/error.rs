use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error(transparent)]
    Kyber(#[from] pqc_kyber::KyberError),

    #[error(transparent)]
    Ed25519(#[from] ed25519_dalek::ed25519::Error),

    #[error("invalid shared key")]
    InvalidSharedKey,

    #[error("encryption/decryption failed")]
    AesGcm(#[from] aes_gcm::Error),

    #[error(transparent)]
    Rand(#[from] rand::Error),
}
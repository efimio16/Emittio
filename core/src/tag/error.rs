use thiserror::Error;

use crate::utils::{SerdeError, ChannelError};

#[derive(Debug, Error)]
pub enum TagServiceError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Io(#[from] tokio::io::Error),

    #[error(transparent)]
    Channel(#[from] ChannelError),
}

#[derive(Debug, Error)]
pub enum TagError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error("encryption failed")]
    AesGcmEncryption(aes_gcm::Error),

    #[error("decryption failed")]
    AesGcmDecryption(aes_gcm::Error),
}
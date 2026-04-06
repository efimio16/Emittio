use thiserror::Error;

use crate::utils::{ChannelError, SerdeError};

#[derive(Debug, Error)]
pub enum DhtStorageError {
     #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    Io(#[from] tokio::io::Error),

    #[error(transparent)]
    Serde(#[from] SerdeError),
}

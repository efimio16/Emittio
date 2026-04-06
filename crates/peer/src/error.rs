use thiserror::Error;

use crate::utils::ChannelError;

#[derive(Debug, Error)]
pub enum PeerTableError {
    #[error(transparent)]
    Channel(#[from] ChannelError),
}
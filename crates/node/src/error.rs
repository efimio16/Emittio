use thiserror::Error;
use tokio::sync::oneshot;

use crate::utils::ChannelError;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error("Timeout")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    OheshotRecv(#[from] oneshot::error::RecvError),
}
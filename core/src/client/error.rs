use thiserror::Error;

use crate::{payload::ReplyError, utils::{ChannelError, SerdeError}};

#[derive(Debug, Error)]
pub enum ClientServiceError {
    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error("Too many pending")]
    TooManyPending,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error("Timeout")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    Reply(#[from] ReplyError),

    #[error("Invalid reply")]
    InvalidReply,

    #[error(transparent)]
    Serde(#[from] SerdeError),
}
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DhtGetError {
    #[error("Internal error")]
    Internal,
}

#[derive(Debug, Error)]
pub enum DhtPutError {
    #[error("File too large")]
    TooLarge,

    #[error("Internal error")]
    Internal,
}

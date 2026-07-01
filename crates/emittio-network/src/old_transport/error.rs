use crypto::{error::CryptoError, id::Id};
use thiserror::Error;
use tokio::{net::tcp::OwnedReadHalf, sync::mpsc::error::SendError, task::JoinError};

use crate::{error::NetError, packet::Message};

// use crate::{net::SessionManagerDispatcherError, utils::ChannelError};

// #[derive(Debug, Error)]
// pub enum MockTransportError {
//     #[error(transparent)]
//     Channel(#[from] ChannelError),

//     #[error(transparent)]
//     NetSession(#[from] SessionManagerDispatcherError),

//     #[error("address not found")]
//     AddressNotFound,

//     #[error("peer not found")]
//     PeerNotFound,
// }

#[derive(Debug, Error)]
pub enum TransportError {
    // Make Id optional so that callers can add information without having to pass Id down to every function
    #[error("session's read task has been cancelled")]
    ReadCancelled(OwnedReadHalf,Option<Id>),
    
    #[error("request was cancelled by token")]
    Cancelled,

    // Consider returning peer ID as data in this error
    #[error("session not found")]
    SessionNotFound(Option<Id>),

    // Consider returning peer ID as data in this error
    #[error("connection not found in active map")]
    ConnectionNotInMap(Option<Id>),

    // Consider returning peer ID as data in this error
    #[error("no sessions available")]
    NoSessions,

    // Consider returning peer ID as data in this error
    #[error("message channel has been closed")]
    MessageChannelClosed,

    #[error("peer already connected")]
    PeerAlreadyConnected(Option<Id>),

    #[error("connection closed")]
    ConnectionClosed(Option<Id>),

    #[error(transparent)]
    Serialization(#[from] postcard::Error),

    #[error(transparent)]
    Reading(#[from] SendError::<(Id,Message)>),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Join(#[from] JoinError),

    #[error(transparent)]
    Encrypt(#[from] CryptoError),

    #[error(transparent)]
    Decrypt(#[from] NetError),
}
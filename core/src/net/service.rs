use tokio_util::sync::CancellationToken;
use tokio::task::JoinError;
use thiserror::Error;
use crate::{net::{session::NetSession,error::{NetError,CryptoError}},message::{IncomingMessage,OutgoingMessage},payload::Payload,peer::{Peer,PeerId},utils::{SerdeError,ChannelError}, transport::{TransportError}};

#[derive(Debug,Error)]
pub enum NetServError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Net(#[from] NetError),

    #[error(transparent)]
    Join(#[from] JoinError),

    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error("client not found")]
    ClientNotFound,

    #[error("sessions not found")]
    SessionsNotFound,

    #[error("Session already exists")]
    SessionAlreadyExists,

    #[error("peer not found")]
    PeerNotFound,

}



pub trait NetService{
    // Interface for handling connections out from a single peer/client

    type Error: Into<NetServError>;

    // Add fully formed sessions to the service
    // Deal with handshakes prior to adding into service
    // Fail on adding a session if the peer already exists
    fn add_session(&mut self, client: (Peer,NetSession)) -> impl Future<Output = Result<(), Self::Error>> + Send;
    // It is possible that you'd want multiple sessions between two peers. Currently this would be an error. 
    // If not changed now, it will be hard to fix in the future.

    // Close a session and drop it from the table
    fn drop_session(&mut self, peer: &PeerId) -> Result<(), Self::Error>;

    // Listen for incoming messages from all peers
    fn listen(&self, token: CancellationToken) -> impl Future<Output = Result<IncomingMessage, Self::Error>> + Send;

    // Broadcast messages to all sessions
    // Responsible for encrypting for each peer
    fn broadcast(&self, msg: Payload, token: CancellationToken) -> impl Future<Output = Result<(), Self::Error>> + Send;

    // Transmit messages to a specific session
    // OutgoingMessage has its own PeerID
    fn transmit(&self, msg: Payload,target: Peer, token: CancellationToken) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
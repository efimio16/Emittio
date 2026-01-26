use tokio_util::sync::CancellationToken;
use tokio::task::JoinError;
use thiserror::Error;
use crate::{net::{session::NetSession,error::{NetError,CryptoError}},message::{IncomingMessage,OutgoingMessage},payload::Payload,peer::{PeerId},utils::{SerdeError,ChannelError}};

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
    fn add_session(&mut self, client: (PeerId,NetSession)) -> Result<(), Self::Error>;

    // Drop a session and return it
    // Don't close it, hand it back to the caller to let them handle it
    fn drop_session(&mut self, peer: &PeerId) -> Result<NetSession, Self::Error>;

    // Listen for incoming messages from all peers
    fn listen(&self, token: CancellationToken) -> impl Future<Output = Result<IncomingMessage, Self::Error>> + Send;

    // Broadcast messages to all sessions
    // Responsible for encrypting for each peer
    fn broadcast(&self, msg: Payload, token: CancellationToken) -> impl Future<Output = Result<(), Self::Error>> + Send;

    // Transmit messages to a specific session
    // OutgoingMessage has its own PeerID
    fn transmit(&self, msg: OutgoingMessage, token: CancellationToken) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
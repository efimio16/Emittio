use std::collections::HashMap;
use thiserror::Error;
use tokio_stream::{StreamExt, StreamMap, wrappers::ReceiverStream};
use tokio_util::sync::CancellationToken;

use crate::{channels::{self, ChannelError}, message::{IncomingMessage, OutgoingMessage}, net::{client::NetClient, error::{CryptoError, NetError}, packet::Packet, session_store::SessionStore}, peer::PeerId, peer_table::PeerTableDispatcher, service::Service, utils::{SerdeError, deserialize, serialize}};

pub type TransportDispatcher = channels::Participant<IncomingMessage, OutgoingMessage>;
pub type TransportHandler = channels::Participant<OutgoingMessage, IncomingMessage>;

#[derive(Debug, Error)]
pub enum MockTransportError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Net(#[from] NetError),

    #[error("client not found")]
    ClientNotFound,

    #[error("sessions not found")]
    SessionsNotFound,

    #[error("peer not found")]
    PeerNotFound,
}

pub struct MockTransport {
    transport_handlers: HashMap<PeerId, TransportHandler>,
    peer_table: PeerTableDispatcher,
    clients: HashMap<PeerId, NetClient>,
    session_stores: HashMap<PeerId, SessionStore>,
}

impl MockTransport {
    pub fn new(peer_table: PeerTableDispatcher) -> Self {
        Self {
            transport_handlers: HashMap::new(),
            peer_table,
            clients: HashMap::new(),
            session_stores: HashMap::new(),
        }
    }

    pub fn add_peer(&mut self, peer_id: PeerId, client: NetClient, handler: TransportHandler) {
        self.clients.insert(peer_id, client);
        self.transport_handlers.insert(peer_id, handler);
        self.session_stores.insert(peer_id, SessionStore::new());
    }
}

impl Service for MockTransport {
    type Error = MockTransportError;
    
    async fn run(mut self, token: CancellationToken) -> Result<(), MockTransportError> {
        println!("Running mock transport");

        let mut streams = StreamMap::new();

        let mut transport_txs = HashMap::new();

        for (id, handler) in self.transport_handlers {
            streams.insert(id, ReceiverStream::new(handler.rx));
            transport_txs.insert(id, handler.tx);
        }

        loop {
            tokio::select! {
                _ = token.cancelled() => { return Ok(()); }
                Some((tx_id, tx_msg)) = streams.next() => {
                    let rx_id = tx_msg.to;
                    let tx_bytes = serialize(&tx_msg)?;

                    let Some(tx_client) = self.clients.get(&tx_id) else { return Err(MockTransportError::ClientNotFound); };
                    let Some(rx_client) = self.clients.get(&rx_id) else { return Err(MockTransportError::ClientNotFound); };

                    let [
                        Some(tx_sessions),
                        Some(rx_sessions),
                    ] = self.session_stores.get_disjoint_mut([&tx_id, &rx_id]) else { return Err(MockTransportError::SessionsNotFound); };

                    let tx_session = match tx_sessions.get_by_peer(&rx_id) {
                        Some(v) => v,
                        None => {
                            let Some(rx_peer) = self.peer_table.get_peer(tx_msg.to).await? else { return Err(MockTransportError::PeerNotFound) };

                            let (shared, handshake) = tx_client.handshake(rx_peer.identity)?;
                            let tx_new_conn_id = handshake.created_conn_id;
                            let (rx_session, ack) = rx_client.accept(handshake)?;
                            let rx_new_conn_id = ack.created_conn_id;
                            let tx_session = tx_client.session(shared, ack);

                            rx_sessions.insert(rx_new_conn_id, tx_id, rx_session);
                            tx_sessions.insert(tx_new_conn_id, rx_id, tx_session)
                        },
                    };

                    let msg = tx_session.send(&tx_bytes)?;
                    let Packet::Message(msg) = deserialize(&serialize(&Packet::Message(msg))?)? else { panic!("It should be Packet::Message") };

                    let Some(rx_session) = rx_sessions.get_by_conn(&msg.conn_id) else { continue; };

                    let rx_bytes = rx_session.receive(msg)?;

                    let Some(rx_handler) = transport_txs.get(&tx_msg.to) else { continue; };
                    
                    rx_handler.send(IncomingMessage::receive(tx_id, deserialize(&rx_bytes)?)).await.map_err(|_| ChannelError::Closed)?;
                }
            }
        }
    }
}
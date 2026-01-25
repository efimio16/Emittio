use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_stream::{StreamExt, StreamMap, wrappers::ReceiverStream};
use tokio_util::sync::CancellationToken;

use crate::{message::IncomingMessage, net::{ConnId, NetClient, NetSession, Packet, SessionStore}, peer::{Peer, PeerId, PeerTableDispatcher}, service::Service, transport::{MockTransportError, TransportHandler, TransportParticipant}, utils::{ChannelError, deserialize, serialize}};

pub struct MockTransport {
    transport_handlers: Vec<TransportHandler>,
    peer_table: PeerTableDispatcher,
    clients: Vec<NetClient>,
    static_clients: HashMap<PeerId, NetClient>,
    session_stores: Vec<SessionStore>,
}

impl MockTransport {
    pub fn new(peer_table: PeerTableDispatcher) -> Self {
        Self {
            transport_handlers: Vec::new(),
            peer_table,
            static_clients: HashMap::new(),
            clients: Vec::new(),
            session_stores: Vec::new(),
        }
    }

    pub async fn add_peer(&mut self, participant: &impl TransportParticipant, handler: TransportHandler) -> Result<(), ChannelError> {
        let client = participant.net_client();

        if let Some(identity) = client.identity() {
            let peer_id = identity.peer_id();
            self.peer_table.add_peer(Peer::new(identity, Default::default())).await?;
            self.static_clients.insert(peer_id, client.clone());
        }
        self.transport_handlers.push(handler);
        self.clients.push(client);
        self.session_stores.push(SessionStore::new());

        Ok(())
    }
}

impl Service for MockTransport {
    type Error = MockTransportError;
    
    async fn run(mut self, token: CancellationToken) -> Result<(), MockTransportError> {
        println!("Running mock transport");

        let mut streams = StreamMap::new();
        let mut rx_handlers = Vec::new();
        let mut rx_listeners: HashMap<ConnId, (PeerId, NetSession, &mpsc::Sender<IncomingMessage>)> = HashMap::new();

        let mut i = 0usize;
        for handler in self.transport_handlers {
            streams.insert(i, ReceiverStream::new(handler.rx));
            rx_handlers[i] = handler.tx;
            i += 1;
        }

        loop {
            tokio::select! {
                _ = token.cancelled() => { return Ok(()); }
                Some((i, tx_msg)) = streams.next() => {
                    let rx_id = tx_msg.to;
                    let tx_bytes = serialize(&tx_msg)?;

                    let Some(tx_sessions) = self.session_stores.get_mut(i) else { return Err(MockTransportError::SessionsNotFound); };

                    let tx_session = match tx_sessions.get_by_peer(&rx_id) {
                        Some(v) => v,
                        None => {
                            let tx_client = &self.clients[i];
                            let Some(rx_client) = self.static_clients.get(&rx_id) else { return Err(MockTransportError::ClientNotFound); };
                            let Some(rx_peer) = self.peer_table.get_peer(tx_msg.to).await? else { return Err(MockTransportError::PeerNotFound) };

                            let (shared, handshake) = tx_client.handshake(rx_peer.identity)?;
                            let (tx_new_conn_id, tx_id) = (handshake.created_conn_id, handshake.from.peer_id());
                            let (rx_session, ack) = rx_client.accept(handshake)?;
                            let rx_new_conn_id = ack.created_conn_id;
                            let tx_session = tx_client.session(shared, ack);

                            rx_listeners.insert(rx_new_conn_id, (tx_id, rx_session, &rx_handlers[i]));
                            tx_sessions.insert(tx_new_conn_id, rx_id, tx_session)
                        },
                    };

                    let msg = tx_session.send(&tx_bytes)?;
                    let Packet::Message(msg) = deserialize(&serialize(&Packet::Message(msg))?)? else { panic!("It should be Packet::Message") };

                    let Some((tx_id, rx_session, rx_handler)) = rx_listeners.get_mut(&msg.conn_id) else { continue; };

                    let rx_bytes = rx_session.receive(msg)?;
                    
                    rx_handler.send(IncomingMessage::receive(*tx_id, deserialize(&rx_bytes)?)).await.map_err(|_| ChannelError::Closed)?;
                }
            }
        }
    }
}
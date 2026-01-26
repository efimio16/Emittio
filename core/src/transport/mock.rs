use std::collections::HashMap;
use tokio_stream::{StreamExt, StreamMap, wrappers::ReceiverStream};
use tokio_util::sync::CancellationToken;

use crate::{message::{IncomingMessage, OutgoingMessage}, net::{ConnId, NetClient, NetSession}, peer::{Peer, PeerId, PeerTableDispatcher}, service::Service, transport::{MockTransportError, TransportHandler, TransportParticipant}, utils::{ChannelError, deserialize, mock_peer_addr, serialize}};

pub struct MockTransport {
    peer_table: PeerTableDispatcher,
    clients: HashMap<String, NetClient>,
    handlers: HashMap<String, TransportHandler>,
    sessions: HashMap<ConnId, (String, NetSession, PeerId)>,
    connections: HashMap<String, HashMap<PeerId, ConnId>>,
}

impl MockTransport {
    pub fn new(peer_table: PeerTableDispatcher) -> Self {
        Self {
            peer_table,
            clients: HashMap::new(),
            handlers: HashMap::new(),
            sessions: HashMap::new(),
            connections: HashMap::new(),
        }
    }

    pub async fn add_participant(&mut self, participant: &impl TransportParticipant, handler: TransportHandler) -> Result<(), ChannelError> {
        let client = participant.net_client();
        let address = mock_peer_addr();

        if let Some(identity) = client.identity() {
            self.peer_table.add_peer(Peer::new(identity, address.clone())).await?;
        }
        self.clients.insert(address.clone(), client);
        self.handlers.insert(address.clone(), handler);
        self.connections.insert(address, HashMap::new());

        Ok(())
    }
}

impl Service for MockTransport {
    type Error = MockTransportError;
    async fn run(mut self, token: CancellationToken) -> Result<(), MockTransportError> {
        println!("Running mock transport");

        let mut streams = StreamMap::new();
        let mut rx_handlers = HashMap::new();

        for (addr, handler) in self.handlers.into_iter() {
            streams.insert(addr.clone(), ReceiverStream::new(handler.rx));
            rx_handlers.insert(addr, handler.tx);
        }

        loop {
            tokio::select! {
                _ = token.cancelled() => { return Ok(()); }
                Some((tx_addr, tx_msg)) = streams.next() => {
                    let rx_id = tx_msg.to;

                    let Some(tx_connections) = self.connections.get(&tx_addr) else { return Err(MockTransportError::SessionsNotFound); };

                    let tx_conn = match tx_connections.get(&rx_id) {
                        Some(v) => *v,
                        None => {
                            let tx_client = &self.clients[&tx_addr];
                            let Some(rx_peer) = self.peer_table.get_peer(rx_id).await? else { return Err(MockTransportError::PeerNotFound);};
                            let rx_addr = rx_peer.address;

                            let rx_client = &self.clients[&rx_addr];

                            let (shared, handshake) = tx_client.handshake(rx_peer.identity)?;
                            let (tx_new_conn_id, tx_id) = (handshake.created_conn_id, handshake.from.peer_id());
                            let (rx_session, ack) = rx_client.accept(handshake)?;
                            let rx_new_conn_id = ack.created_conn_id;
                            let tx_session = tx_client.session(shared, ack);

                            {
                                let Some(tx_connections) = self.connections.get_mut(&tx_addr) else { return Err(MockTransportError::SessionsNotFound); };
                                tx_connections.insert(rx_id, tx_new_conn_id);
                            }
                            {
                                let Some(rx_connections) = self.connections.get_mut(&rx_addr) else { return Err(MockTransportError::SessionsNotFound); };
                                rx_connections.insert(tx_id, rx_new_conn_id);
                            }

                            self.sessions.insert(tx_new_conn_id, (tx_addr.clone(), tx_session, rx_id));
                            self.sessions.insert(rx_new_conn_id, (rx_addr, rx_session, tx_id));

                            tx_new_conn_id
                        },
                    };

                    
                    let Some((_, tx_session, _)) = self.sessions.get_mut(&tx_conn) else { return Err(MockTransportError::SessionsNotFound); };
                    let msg = tx_session.send(&serialize(&tx_msg)?)?;
                    
                    let Some((rx_addr, rx_session, tx_id)) = self.sessions.get_mut(&msg.conn_id) else { panic!("continue"); };
                    let rx_bytes = rx_session.receive(msg)?;

                    let Some(rx_handler) = rx_handlers.get(rx_addr) else { panic!("continue"); };
                    let rx_msg: OutgoingMessage = deserialize(&rx_bytes)?;

                    println!("{} -> {}: {:#?}", tx_addr, rx_addr, rx_msg.payload);

                    rx_handler.send(IncomingMessage::receive(*tx_id, rx_msg)).await.map_err(|_| ChannelError::Closed)?;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::timeout;
    use tokio_util::sync::CancellationToken;

    use crate::{message::OutgoingMessage, net::NetClient, payload::{Payload, Reply, TagQuery}, peer::{PeerTable, PeerTableDispatcher}, service::Service, transport::{MockTransport, TransportHandler, TransportParticipant}, utils::random_bytes};

    struct MockParticipant(pub NetClient);

    impl TransportParticipant for MockParticipant {
        fn net_client(&self) -> NetClient {
            self.0.clone()
        }
    }

    fn setup_peer_table() -> PeerTableDispatcher {
        let (service, dispatcher) = PeerTable::new();
        tokio::spawn(async { service.run(CancellationToken::new()).await.unwrap() });

        dispatcher
    }

    #[tokio::test]
    async fn test() {
        const TIMEOUT: Duration = Duration::from_secs(5);

        let peer_table = setup_peer_table();

        let mut transport = MockTransport::new(peer_table);

        let (alice_handler, mut alice) = TransportHandler::new();
        let (bob_handler, mut bob) = TransportHandler::new();

        let bob_cl = NetClient::from_seed(random_bytes());
        let bob_id = bob_cl.identity().expect("bob should be static").peer_id();

        transport.add_participant(&MockParticipant(NetClient::Ephemeral), alice_handler).await.expect("add participant failed");
        transport.add_participant(&MockParticipant(bob_cl), bob_handler).await.expect("add participant failed");

        tokio::spawn(async { transport.run(CancellationToken::new()).await.unwrap() });

        alice.send(OutgoingMessage::query(&bob_id, TagQuery::Get)).await.expect("alice query failed");

        let incoming = timeout(TIMEOUT, bob.recv()).await.expect("timeout").expect("channel should not be closed");

        bob.send(incoming.reply(Reply::Ok)).await.expect("bob reply failed");

        let reply = timeout(TIMEOUT, alice.recv()).await.expect("timeout").unwrap();

        assert!(matches!(reply.payload, Payload::Reply(Reply::Ok)));
    }
}
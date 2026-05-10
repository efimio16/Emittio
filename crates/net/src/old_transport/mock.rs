use std::collections::HashMap;
use tokio_stream::{StreamExt, StreamMap, wrappers::ReceiverStream};
use tokio_util::sync::CancellationToken;

use crate::{net::{NetIdentity, SessionManagerDispatcher}, peer::{Peer, PeerTableDispatcher}, service::Service, transport::{MockTransportError, TransportHandler}, utils::{ChannelError, mock_peer_addr}};

pub struct MockTransport {
    peer_table: PeerTableDispatcher,
    handlers: HashMap<String, TransportHandler>,
    clients: HashMap<String, SessionManagerDispatcher>,
}

impl MockTransport {
    pub fn new(peer_table: PeerTableDispatcher) -> Self {
        Self {
            peer_table,
            clients: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    pub async fn add_participant(&mut self, client: SessionManagerDispatcher, identity: Option<NetIdentity>, handler: TransportHandler) -> Result<(), ChannelError> {
        let address = mock_peer_addr();

        if let Some(identity) = identity {
            self.peer_table.add_peer(Peer::new(identity, address.clone())).await?;
        }
        self.clients.insert(address.clone(), client);
        self.handlers.insert(address.clone(), handler);

        Ok(())
    }
}

impl Service for MockTransport {
    type Error = MockTransportError;
    async fn run(self, token: CancellationToken) -> Result<(), MockTransportError> {
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
                    let tx_client = &self.clients[&tx_addr];

                    let conn_id = match tx_client.connections(tx_msg.to).await?.get(0) {
                        Some(&v) => v,
                        None => {
                            let Some(rx_peer) = self.peer_table.get_peer(rx_id).await? else { return Err(MockTransportError::PeerNotFound); };
                            let rx_addr = rx_peer.address;

                            let handshake = tx_client.handshake(rx_peer.identity, rx_addr.clone()).await?;

                            let rx_client = &self.clients[&rx_addr];

                            let ack = rx_client.accept(handshake, tx_addr.clone()).await?;

                            let conn_id = tx_client.confirm(ack).await?;
                            conn_id
                        }
                    };
                    
                    let msg = tx_client.send(conn_id, tx_msg).await?;

                    let Some(rx_addr) = tx_client.addr(rx_id).await? else { return Err(MockTransportError::AddressNotFound); };

                    let rx_client = &self.clients[&rx_addr];

                    let rx_msg = rx_client.recv(msg).await?;

                    println!("{} -> {}: {:#?}", tx_addr, rx_addr, rx_msg.payload);

                    rx_handlers[&rx_addr].send(rx_msg).await.map_err(|_| ChannelError::Closed)?;
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

    use crate::{message::OutgoingMessage, net::{NetClient, SessionManager, SessionManagerDispatcher}, payload::{Payload, Reply, TagQuery}, peer::{PeerTable, PeerTableDispatcher}, service::Service, transport::{MockTransport, TransportHandler}, utils::random_bytes};

    fn setup_peer_table() -> PeerTableDispatcher {
        let (service, dispatcher) = PeerTable::new();
        tokio::spawn(async { service.run(CancellationToken::new()).await.unwrap() });

        dispatcher
    }

    fn setup_session_manager(client: NetClient) -> SessionManagerDispatcher {
        let (dispatcher, service) = SessionManager::new(client);
        tokio::spawn(service.run(CancellationToken::new()));

        dispatcher
    }

    #[tokio::test]
    async fn test() {
        const TIMEOUT: Duration = Duration::from_secs(5);

        let peer_table = setup_peer_table();

        let bob_cl = NetClient::from_seed(random_bytes());
        let bob_identity = bob_cl.identity().expect("bob should be static");
        let bob_id = bob_identity.peer_id();

        let alice_sessions = setup_session_manager(NetClient::Ephemeral);
        let bob_sessions = setup_session_manager(bob_cl);

        let mut transport = MockTransport::new(peer_table);

        let (alice_handler, mut alice) = TransportHandler::new();
        let (bob_handler, mut bob) = TransportHandler::new();

        transport.add_participant(alice_sessions, None, alice_handler).await.expect("add participant failed");
        transport.add_participant(bob_sessions, Some(bob_identity), bob_handler).await.expect("add participant failed");

        tokio::spawn(async { transport.run(CancellationToken::new()).await.unwrap() });

        alice.send(OutgoingMessage::query(&bob_id, TagQuery::Get)).await.expect("alice query failed");

        let incoming = timeout(TIMEOUT, bob.recv()).await.expect("timeout").expect("channel should not be closed");

        bob.send(incoming.reply(Reply::Ok)).await.expect("bob reply failed");

        let reply = timeout(TIMEOUT, alice.recv()).await.expect("timeout").unwrap();

        assert!(matches!(reply.payload, Payload::Reply(Reply::Ok)));
    }
}
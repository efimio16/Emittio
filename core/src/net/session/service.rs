use std::collections::{BTreeMap, HashMap, HashSet};
use tokio::{sync::mpsc, time::{Duration, interval}};
use tokio_util::sync::CancellationToken;
use std::mem;

use crate::{message::IncomingMessage, net::{ActiveSession, ConnId, NetClient, SessionManagerDispatcherError, SessionManagerServiceError, PendingSession}, peer::PeerId, service::Service, utils::{deserialize, get_timestamp, reply, reply_err, reply_ok, serialize}};

use super::SessionManagerCmd;

const TTL_PENDING_SESSION: u64 = 10;
const TTL_ACTIVE_SESSION: u64 = 60;
const TICK_INTERVAL: Duration = Duration::from_secs(10);

pub struct SessionManagerService {
    client: NetClient,
    rx: mpsc::Receiver<SessionManagerCmd>,
    ttl_active: BTreeMap<u64, ConnId>,
    ttl_pending: BTreeMap<u64, ConnId>,
    active_sessions: HashMap<ConnId, ActiveSession>,
    pending_sessions: HashMap<ConnId, PendingSession>,
    conns_by_peer: HashMap<PeerId, HashSet<ConnId>>,
    peer_by_conn: HashMap<ConnId, PeerId>,
    addr_by_peer: HashMap<PeerId, String>,
}

impl SessionManagerService {
    pub(super) fn new(client: NetClient, rx: mpsc::Receiver<SessionManagerCmd>) -> Self {
        Self {
            client,
            rx,
            ttl_active: BTreeMap::new(),
            ttl_pending: BTreeMap::new(),
            active_sessions: HashMap::new(),
            pending_sessions: HashMap::new(),
            conns_by_peer: HashMap::new(),
            peer_by_conn: HashMap::new(),
            addr_by_peer: HashMap::new(),
        }
    }

    fn add_pending_session(&mut self, conn_id: ConnId, peer_id: PeerId, addr: String, session: PendingSession) {
        self.pending_sessions.insert(conn_id, session);
        self.peer_by_conn.insert(conn_id, peer_id);
        self.ttl_pending.insert(get_timestamp() + TTL_PENDING_SESSION, conn_id);
        self.addr_by_peer.insert(peer_id, addr);
    }

    fn activate(&mut self, conn_id: ConnId, peer_id: PeerId, created_conn_id: Option<ConnId>) -> Result<Result<&mut ActiveSession, SessionManagerDispatcherError>, SessionManagerServiceError> {
        let Some(pending_session) = self.pending_sessions.remove(&conn_id) else {
            return Ok(Err(SessionManagerDispatcherError::SessionNotFound));
        };

        self.conns_by_peer.entry(peer_id).or_insert_with(|| HashSet::new()).insert(conn_id);

        let Some(activated) = pending_session.activate(created_conn_id) else {
            return Ok(Err(SessionManagerDispatcherError::MissingConnId));
        };

        self.ttl_pending.remove(&conn_id);
        self.ttl_active.insert(get_timestamp() + TTL_ACTIVE_SESSION, conn_id);

        Ok(Ok(self.active_sessions.entry(conn_id).or_insert(activated)))
    }
}

impl Service for SessionManagerService {
    type Error = SessionManagerServiceError;
    
    async fn run(mut self, token: CancellationToken) -> Result<(), SessionManagerServiceError> {
        println!("Running net session service");

        let mut ticker = interval(TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    return Ok(());
                }
                _ = ticker.tick() => {
                    let then = get_timestamp() + 1;

                    let mut pending_expired = self.ttl_pending.split_off(&then);
                    mem::swap(&mut self.ttl_pending, &mut pending_expired);

                    let mut active_expired = self.ttl_active.split_off(&then);
                    mem::swap(&mut self.ttl_active, &mut active_expired);

                    for (_, conn_id) in pending_expired {
                        self.pending_sessions.remove(&conn_id);
                        let Some(peer_id) = self.peer_by_conn.remove(&conn_id) else { continue; };
                        self.addr_by_peer.remove(&peer_id);
                    }

                    for (_, conn_id) in active_expired {
                        self.active_sessions.remove(&conn_id);
                        let Some(peer_id) = self.peer_by_conn.remove(&conn_id) else { continue; };
                        self.addr_by_peer.remove(&peer_id);
                        let Some(conns) = self.conns_by_peer.get_mut(&peer_id) else { continue; };
                        conns.remove(&conn_id);
                        if conns.len() == 0 {
                            self.conns_by_peer.remove(&peer_id);
                        }
                    }
                }
                Some(cmd) = self.rx.recv() => {
                    match cmd {
                        SessionManagerCmd::Handshake { with, tx, addr } => {
                            let peer_id = with.peer_id();
                            let (pending_session, handshake) = match self.client.handshake(with) {
                                Ok(res) => res,
                                Err(err) => {
                                    reply_err(tx, err)?;
                                    continue;
                                }
                            };

                            self.add_pending_session(handshake.created_conn_id, peer_id, addr, pending_session);

                            reply_ok(tx, handshake)?;
                        },
                        SessionManagerCmd::Accept { handshake, tx, addr } => {
                            let peer_id = handshake.from.peer_id();
                            let (pending_session, ack) = match self.client.accept(handshake) {
                                Ok(res) => res,
                                Err(err) => {
                                    reply_err(tx, err)?;
                                    continue;
                                }
                            };

                            self.add_pending_session(ack.created_conn_id, peer_id, addr, pending_session);

                            reply_ok(tx, ack)?;
                        },
                        SessionManagerCmd::Confirm { ack, tx } => {
                            let &peer_id = match self.peer_by_conn.get(&ack.conn_id) {
                                Some(res) => res,
                                None => {
                                    reply_err(tx, SessionManagerDispatcherError::PeerIdNotFound)?;
                                    continue;
                                }
                            };

                            match self.activate(ack.conn_id, peer_id, Some(ack.created_conn_id))? {
                                Ok(_) => reply_ok(tx, ack.conn_id)?,
                                Err(err) => reply_err(tx, err)?,
                            };
                        },
                        SessionManagerCmd::Conns { of, tx } => {
                            reply(tx, self.conns_by_peer
                                .get(&of)
                                .and_then(|conns| Some(conns.iter().copied().collect()))
                                .unwrap_or_else(|| Vec::new())
                            )?;
                        },
                        SessionManagerCmd::Addr { of, tx } => {
                            reply(tx, self.addr_by_peer.get(&of).cloned())?;
                        }
                        SessionManagerCmd::Send { conn_id, msg, tx } => {
                            let session = match self.active_sessions.get_mut(&conn_id) {
                                Some(res) => res,
                                None => {
                                    reply_err(tx, SessionManagerDispatcherError::SessionNotFound)?;
                                    continue;
                                }
                            };

                            let bytes = match serialize(&msg) {
                                Ok(res) => res,
                                Err(err) => {
                                    reply_err(tx, err)?;
                                    continue;
                                }
                            };

                            let message = match session.send(&bytes) {
                                Ok(res) => res,
                                Err(err) => {
                                    reply_err(tx, err)?;
                                    continue;
                                }
                            };
                            
                            reply_ok(tx, message)?;
                        },
                        SessionManagerCmd::Recv { msg, tx } => {
                            let conn_id = msg.conn_id;

                            let &peer_id = match self.peer_by_conn.get(&conn_id) {
                                Some(res) => res,
                                None => {
                                    reply_err(tx, SessionManagerDispatcherError::PeerIdNotFound)?;
                                    continue;
                                }
                            };

                            let session = match self.active_sessions.get_mut(&conn_id) {
                                Some(session) => session,
                                None => {
                                    match self.activate(conn_id, peer_id, None)? {
                                        Ok(res) => res,
                                        Err(err) => {
                                            reply_err(tx, err)?;
                                            continue;
                                        }
                                    }
                                }
                            };

                            let bytes = match session.receive(msg) {
                                Ok(res) => res,
                                Err(err) => {
                                    reply_err(tx, err)?;
                                    continue;
                                }
                            };

                            let outgoing = match deserialize(&bytes) {
                                Ok(res) => res,
                                Err(err) => {
                                    reply_err(tx, err)?;
                                    continue;
                                }
                            };

                            reply_ok(tx, IncomingMessage::receive(peer_id, outgoing))?;
                        },
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::{task::JoinHandle, time::sleep};
    use tokio_util::sync::CancellationToken;

    use crate::{message::OutgoingMessage, net::{NetClient, SessionManager, SessionManagerServiceError}, payload::{Payload, Query, Reply, TagQuery, TagReply}, service::Service, utils::random_bytes};
    use super::super::SessionManagerDispatcher;

    fn setup_service(client: NetClient, token: CancellationToken) -> (SessionManagerDispatcher, JoinHandle<Result<(), SessionManagerServiceError>>) {
        let (dispatcher, service) = SessionManager::new(client);
        let handle = tokio::spawn(service.run(token));

        (dispatcher, handle)
    }

    #[tokio::test]
    async fn test_net_session() {
        let alice = NetClient::Ephemeral;
        let bob = NetClient::from_seed(random_bytes());
        let bob_identity = bob.identity().expect("bob should have static identity");
        let bob_id = bob_identity.peer_id();

        let (alice_dispatcher, _) = setup_service(alice, CancellationToken::new());
        let (bob_dispatcher, _) = setup_service(bob, CancellationToken::new());

        let handshake = alice_dispatcher.handshake(bob_identity, "test://bob".into()).await.expect("handshake failed");

        let ack = bob_dispatcher.accept(handshake, "test://alice".into()).await.expect("handshake ack failed");
        
        let to_bob_conn = alice_dispatcher.confirm(ack).await.expect("confirm failed");

        let msg = alice_dispatcher.send(to_bob_conn, OutgoingMessage::query(&bob_id, TagQuery::Get)).await.expect("alice send failed");

        let incoming = bob_dispatcher.recv(msg).await.expect("bob receive failed");

        assert!(matches!(incoming.payload, Payload::Query(Query::Tag(TagQuery::Get))));

        let &to_alice_conn = bob_dispatcher.connections(incoming.from).await.expect("bob connections lookup failed").get(0).expect("bob should have a connecton");

        let msg = bob_dispatcher.send(to_alice_conn, OutgoingMessage::reply(&bob_id, TagReply::Return(vec![]))).await.expect("bob send failed");

        let incoming = alice_dispatcher.recv(msg).await.expect("alice receive failed");

        assert!(matches!(incoming.payload, Payload::Reply(Reply::Tag(TagReply::Return(_)))));
    }

    #[tokio::test]
    async fn test_ttl() {
        let alice = NetClient::Ephemeral;
        let bob = NetClient::from_seed(random_bytes());
        let bob_identity = bob.identity().expect("bob should have static identity");
        let bob_id = bob_identity.peer_id();

        let (alice_dispatcher, _) = setup_service(alice, CancellationToken::new());
        let (bob_dispatcher, _) = setup_service(bob, CancellationToken::new());

        let handshake = alice_dispatcher.handshake(bob_identity, "test://bob".into()).await.expect("handshake failed");
        let ack = bob_dispatcher.accept(handshake, "test://alice".into()).await.expect("handshake ack failed");
        let to_bob_conn = alice_dispatcher.confirm(ack).await.expect("confirm failed");

        println!("Waiting 20 seconds...");
        sleep(Duration::from_secs(20)).await;

        let msg = alice_dispatcher.send(to_bob_conn, OutgoingMessage::query(&bob_id, TagQuery::Get)).await.expect("alice send failed");

        bob_dispatcher.recv(msg).await.expect_err("bob should delete pending session");
    }
}
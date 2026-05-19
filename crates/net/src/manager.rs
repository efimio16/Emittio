use std::collections::{HashMap, HashSet};
use crypto::kem::{Kem, SecretKey, SharedSecret};
use tokio_util::task::JoinMap;
use actor::{actor, channel, mpsc, oneshot, ok_or_reply};

use crate::{error::NetworkError, message::{MsgId, OutgoingMessage}, packet::{ConnId, Handshake, Packet}, payload::Payload, peer::{Peer, PeerId}, session::Session};

const CHAN_SIZE: usize = 256;

type ConnHandle = mpsc::Sender<Packet>;
type PayloadCallback = oneshot::Sender<Result<Payload, NetworkError>>;

#[actor]
mod network {
    #[commands]
    pub enum NetworkCmd {
        #[reply(Result<Payload, NetworkError>)]
        Send { peer: Peer, payload: OutgoingMessage },
    }

    #[handle]
    pub struct NetworkHandle;

    #[service]
    pub struct NetworkService {
        zero_rtt_resp_states: HashMap<PeerId, SharedSecret>,
        one_rtt_init_states: HashMap<PeerId, SecretKey>,
        one_rtt_resp_states: HashMap<PeerId, SharedSecret>,

        zero_rtt_sessions: HashMap<PeerId, Session>,
        one_rtt_sessions: HashMap<PeerId, Session>,

        callbacks: HashMap<(PeerId, MsgId), PayloadCallback>,
        ttl_sessions: HashMap<u64, PeerId>,

        conns_by_peer: HashMap<PeerId, HashSet<ConnId>>,
        peer_by_conn: HashMap<ConnId, PeerId>,
        pending_conns: HashSet<ConnId>,
        active_conns: HashMap<ConnId, ConnHandle>,
        connections: JoinMap<ConnId, NetworkError>,
        ttl_connections: HashMap<u64, ConnId>,
    }

    #[service]
    impl NetworkService {
        // TODO: change OutgoingMessage to Payload
        // TODO: change &Peer to PeerId when node manager will be implemented

        #[command(Send)]
        async fn send(&mut self, peer: Peer, payload: OutgoingMessage, callback: PayloadCallback) {
            ok_or_reply!(NetworkError, callback, {
                let session = self.select_session(&peer).await?;

                let wire_payload = session.send(&postcard::to_stdvec(&payload)?)?;

                let conn = self.select_connection(&peer).await?;
                channel::send(&conn, Packet::Message(wire_payload)).await?;

                Ok(())
            });
        }
    }

    impl NetworkService {
        fn new() -> (Self, NetworkHandle) {
            let (tx, rx) = mpsc::channel(CHAN_SIZE);

            (Self {
                rx,

                zero_rtt_resp_states: HashMap::new(),
                one_rtt_init_states: HashMap::new(),
                one_rtt_resp_states: HashMap::new(),

                zero_rtt_sessions: HashMap::new(),
                one_rtt_sessions: HashMap::new(),
                callbacks: HashMap::new(),
                ttl_sessions: HashMap::new(),

                conns_by_peer: HashMap::new(),
                peer_by_conn: HashMap::new(),
                pending_conns: HashSet::new(),
                active_conns: HashMap::new(),
                connections: JoinMap::new(),
                ttl_connections: HashMap::new(),
            }, NetworkHandle { tx })
        }

        // TODO: change &Peer to PeerId when node manager will be implemented
        async fn select_session(&mut self, peer: &Peer) -> Result<&mut Session, NetworkError> {
            let session = if self.one_rtt_sessions.contains_key(&peer.id) {
                self.one_rtt_sessions.get_mut(&peer.id).unwrap()
            } else if let Some(state) = self.one_rtt_resp_states.remove(&peer.id) {
                let session = Session::new(state, false);

                self.one_rtt_sessions.insert(peer.id.clone(), session);
                self.one_rtt_sessions.get_mut(&peer.id).unwrap()
            } else if self.zero_rtt_sessions.contains_key(&peer.id) {
                self.zero_rtt_sessions.get_mut(&peer.id).unwrap()
            } else {
                let keypair = Kem::random();

                let (capsule, shared) = keypair.sk.shared(&peer.pk)?;

                self.one_rtt_init_states.insert(peer.id.clone(), keypair.sk);

                let handshake = Handshake {
                    pk: keypair.pk,
                    capsule,
                };

                let conn = self.select_connection(peer).await?;
                channel::send(&conn, Packet::Handshake(handshake)).await?;

                let session = Session::new(shared, true);

                self.zero_rtt_sessions.insert(peer.id.clone(), session);
                self.zero_rtt_sessions.get_mut(&peer.id).unwrap()
            };

            Ok(session)
        }

        // TODO: change &Peer to PeerId when node manager will be implemented
        async fn select_connection(&mut self, peer: &Peer) -> Result<ConnHandle, NetworkError> {
            todo!("Get or init connection")
        }
    }
}
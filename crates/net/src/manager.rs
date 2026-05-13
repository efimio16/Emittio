use std::collections::{HashMap, HashSet};
use crypto::kem::{Kem, SecretKey, SharedSecret};
use service::{Service, channel::{self, ChannelError}, commands};
use tokio::sync::{mpsc, oneshot};
use tokio_util::{sync::CancellationToken, task::JoinMap};

use crate::{error::NetworkError, message::{MsgId, OutgoingMessage}, packet::{ConnId, Handshake, Packet}, payload::Payload, peer::{Peer, PeerId}, session::Session};

const CHAN_SIZE: usize = 256;

type ConnHandle = mpsc::Sender<Packet>;
type PayloadCallback = oneshot::Sender<Result<Payload, NetworkError>>;

commands!(
    NetworkCmd, NetworkDispatcher,
    // TODO: change OutgoingMessage to Payload
    // TODO: change Peer to PeerId when node manager will be implemented
    send => Send { peer: Peer, payload: OutgoingMessage } -> Result<Payload, NetworkError>,
);

pub struct NetworkManager;

impl NetworkManager {
    pub fn new() -> (NetworkDispatcher, NetworkService) {
        let (tx, rx) = mpsc::channel(CHAN_SIZE);
        (NetworkDispatcher { tx }, NetworkService::new(rx))
    }
}

pub struct NetworkService {
    rx: mpsc::Receiver<NetworkCmd>,

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

impl NetworkService {
    fn new(rx: mpsc::Receiver<NetworkCmd>) -> Self {
        Self {
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
        }
    }

    // TODO: change OutgoingMessage to Payload
    // TODO: change &Peer to PeerId when node manager will be implemented
    async fn send(&mut self, peer: &Peer, payload: OutgoingMessage) -> Result<(), NetworkError> {
        let session = self.select_session(peer).await?;

        let wire_payload = session.send(&postcard::to_stdvec(&payload)?)?;

        let conn = self.select_connection(peer).await?;
        channel::send(&conn, Packet::Message(wire_payload)).await?;

        Ok(())
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

impl Service for NetworkService {
    type Error = ChannelError;

    async fn run(mut self, token: CancellationToken) -> Result<(), Self::Error> {
        loop {
            tokio::select! {
                _ = token.cancelled() => { return Ok(()); },
                Some(cmd) = self.rx.recv() => {
                    match cmd {
                        NetworkCmd::Send { peer, payload, reply_tx } => {
                            let payload_id = payload.id.clone();
                            match self.send(&peer, payload).await {
                                Ok(()) => {
                                    self.callbacks.insert((peer.id.clone(), payload_id), reply_tx);
                                }
                                Err(err) => {
                                    channel::reply(reply_tx, Err(err))?;
                                },
                            }
                        },
                    }
                }
            }
        }
    }
}
use std::collections::{HashMap, HashSet};
use crypto::id::Id;
use service::{Service, channel::ChannelError, commands};
use tokio::sync::{mpsc, oneshot};
use tokio_util::{sync::CancellationToken, task::JoinMap};

use crate::{error::NetworkError, message::MsgId, packet::{ConnId, Packet}, payload::Payload, peer::PeerId, session::{ActiveSession, EphemeralState, PendingSession, SessionId}};

const CHAN_SIZE: usize = 256;

type ConnHandle = mpsc::Sender<Packet>;
type PayloadCallback = oneshot::Sender<Payload>;

commands!(
    NetworkCmd, NetworkDispatcher,
    send => Send { peer_id: PeerId, payload: Payload } -> Payload,
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

    sessions_by_peer: HashMap<PeerId, HashSet<SessionId>>,
    peer_by_session: HashMap<SessionId, PeerId>,
    active_sessions: HashMap<SessionId, ActiveSession>,
    pending_sessions: HashMap<SessionId, PendingSession>,
    ephemeral_states: HashMap<PeerId, EphemeralState>,
    message_callbacks: HashMap<(SessionId, MsgId), PayloadCallback>,
    ttl_sessions: HashMap<u64, SessionId>,

    conns_by_peer: HashMap<PeerId, HashSet<ConnId>>,
    peer_by_conn: HashMap<ConnId, PeerId>,
    pending_conns: HashSet<ConnId>,
    active_conns: HashMap<ConnId, ConnHandle>,
    connections: JoinMap<ConnId, NetworkError>,
    ttl_connections: HashMap<u64, SessionId>,
}

impl NetworkService {
    fn new(rx: mpsc::Receiver<NetworkCmd>) -> Self {
        Self {
            rx,

            sessions_by_peer: HashMap::new(),
            peer_by_session: HashMap::new(),
            active_sessions: HashMap::new(),
            pending_sessions: HashMap::new(),
            ephemeral_states: HashMap::new(),
            message_callbacks: HashMap::new(),
            ttl_sessions: HashMap::new(),

            conns_by_peer: HashMap::new(),
            peer_by_conn: HashMap::new(),
            pending_conns: HashSet::new(),
            active_conns: HashMap::new(),
            connections: JoinMap::new(),
            ttl_connections: HashMap::new(),
        }
    }

    pub async fn send(&mut self, peer_id: Id, payload: Payload, callback: PayloadCallback) {
        todo!("Handle sessions, encrypt, serialize, find a connection and send it to an active connection")
    }

    fn select_session(&mut self, peer_id: Id) -> ActiveSession {
        todo!("Get or init session")
    }

    fn select_connection(&mut self, peer_id: Id) -> ConnHandle {
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
                        NetworkCmd::Send { peer_id, payload, reply_tx } => self.send(peer_id, payload, reply_tx).await,
                    }
                }
            }
        }
    }
}
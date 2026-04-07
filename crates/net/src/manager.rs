use std::collections::{HashMap, HashSet};
use tokio::sync::mpsc;
use tokio_util::task::JoinMap;

use crate::{error::NetError, message::MsgId, packet::{ConnId, Message, Packet}, peer::PeerId, session::{ActiveSession, EphemeralState, PendingSession, SessionId}};

struct NetworkManager {
    sessions_by_peer: HashMap<PeerId, HashSet<SessionId>>,
    peer_by_session: HashMap<SessionId, PeerId>,
    active_sessions: HashMap<SessionId, ActiveSession>,
    pending_sessions: HashMap<SessionId, PendingSession>,
    ephemeral_states: HashMap<PeerId, EphemeralState>,
    message_callbacks: HashMap<(SessionId, MsgId), mpsc::Sender<Message>>,
    ttl_sessions: HashMap<u64, SessionId>,

    conns_by_peer: HashMap<PeerId, HashSet<ConnId>>,
    peer_by_conn: HashMap<ConnId, PeerId>,
    pending_conns: HashSet<ConnId>,
    active_conns: HashMap<ConnId, mpsc::Sender<Packet>>,
    connections: JoinMap<ConnId, NetError>,
    ttl_connections: HashMap<u64, SessionId>,
}
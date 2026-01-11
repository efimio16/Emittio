use std::collections::HashMap;

use crate::{net::{packet::ConnId, session::NetSession}, peer::PeerId};

pub struct SessionStore {
    by_conn: HashMap<ConnId, NetSession>,
    by_peer: HashMap<PeerId, ConnId>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self { by_conn: HashMap::new(), by_peer: HashMap::new() }
    }
    pub fn insert(&mut self, conn: ConnId, peer: PeerId, session: NetSession) -> &mut NetSession {
        self.by_peer.insert(peer, conn);
        self.by_conn.entry(conn).or_insert(session)
    }
    pub fn get_by_conn(&mut self, conn: &ConnId) -> Option<&mut NetSession> {
        self.by_conn.get_mut(conn)
    }
    pub fn get_by_peer(&mut self, peer: &PeerId) -> Option<&mut NetSession> {
        self.by_peer
            .get(peer)
            .and_then(|conn| self.by_conn.get_mut(conn))
    }
    pub fn remove(&mut self, conn: &ConnId, peer: &PeerId) {
        self.by_conn.remove(conn);
        self.by_peer.remove(peer);
    }
}
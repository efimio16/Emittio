use std::collections::{HashMap, HashSet};
use bytes::Bytes;
use emittio_crypto::{derivable::Derivable, kem::{Kem, SecretKey, SharedSecret}};
use tokio_util::task::JoinMap;
use actorify::{Callback, Channel, actor, ok_or_reply};
use rand::{RngCore, rngs::OsRng};

use crate::{error::NetworkError, peer::{Peer, PeerId}, query::{PeerSelection, Query}, reply::Reply, session::Session, types::{Handshake, Packet, PayloadId}};

use crate::types::FrameData;

type ConnId = u64;

pub struct NetworkActor {
    zero_rtt_resp_states: HashMap<PeerId, SharedSecret>,
    one_rtt_init_states: HashMap<PeerId, SecretKey>,
    one_rtt_resp_states: HashMap<PeerId, SharedSecret>,

    zero_rtt_sessions: HashMap<PeerId, Session>,
    one_rtt_sessions: HashMap<PeerId, Session>,

    callbacks: HashMap<(PeerId, PayloadId), Callback<Result<Bytes, NetworkError>>>,
    ttl_sessions: HashMap<u64, PeerId>,

    conns_by_peer: HashMap<PeerId, HashSet<ConnId>>,
    peer_by_conn: HashMap<ConnId, PeerId>,
    pending_conns: HashSet<ConnId>,
    active_conns: HashMap<ConnId, Channel<Packet>>,
    connections: JoinMap<ConnId, NetworkError>,
    ttl_connections: HashMap<u64, ConnId>,
}

#[actor]
impl NetworkActor {
    /// Sends query to peers selected by `peer_selection` returning their replies
    #[command]
    async fn query(&mut self, peer_selection: PeerSelection, mut query: Query, #[callback] callback: Result<Vec<(PeerId, Reply)>, NetworkError>) {
        // TODO: change to a counter
        let query_id = OsRng.next_u32() as u16;
        query.query_id = query_id;
        let peers = self.select_peers(peer_selection);

        ok_or_reply!(callback, {
            for peer_id in peers {
                self.send(&peer_id, &FrameData::Query(query.clone())).await?;
            }

            Ok(())
        });
        todo!("Store callback into a reply accumulator");
        // self.callbacks.insert((*peer_id, payload_id), callback);
    }

    async fn send(&mut self, peer_id: &PeerId, data: &FrameData) -> Result<(), NetworkError> {
        let session = self.select_session(&peer_id).await?;

        let frame = session.send(data)?;

        let conn = self.select_connection(&peer_id).await?;
        conn.send(Packet::Frame(frame)).await?;

        Ok(())
    }

    /// Updates peer score based on reply verification results
    #[command]
    async fn verifications(&mut self, results: Vec<(PeerId, bool)>, #[callback] callback: ()) {
        todo!("Update peer score");
    }

    pub fn new() -> Self {
        Self {
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

    async fn select_session(&mut self, peer_id: &PeerId) -> Result<&mut Session, NetworkError> {
        let session = if self.one_rtt_sessions.contains_key(peer_id) {
            self.one_rtt_sessions.get_mut(&peer_id).unwrap()
        } else if let Some(state) = self.one_rtt_resp_states.remove(peer_id) {
            let session = Session::new(state, false);

            self.one_rtt_sessions.insert(*peer_id, session);
            self.one_rtt_sessions.get_mut(peer_id).unwrap()
        } else if self.zero_rtt_sessions.contains_key(peer_id) {
            self.zero_rtt_sessions.get_mut(peer_id).unwrap()
        } else {
            let Some(peer) = self.get_peer(peer_id) else {
                return Err(NetworkError::PeerNotFound(*peer_id));
            };

            let keypair = Kem::random();

            let (capsule, shared) = keypair.sk.shared(&peer.pk)?;

            self.one_rtt_init_states.insert(peer.id.clone(), keypair.sk);

            let handshake = Handshake {
                pk: keypair.pk,
                capsule,
            };

            let conn = self.select_connection(peer_id).await?;
            conn.send(Packet::Handshake(handshake)).await?;

            let session = Session::new(shared, true);

            self.zero_rtt_sessions.insert(*peer_id, session);
            self.zero_rtt_sessions.get_mut(peer_id).unwrap()
        };

        Ok(session)
    }

    async fn select_connection(&mut self, peer_id: &PeerId) -> Result<Channel<Packet>, NetworkError> {
        todo!("Get or init connection")
    }

    fn get_peer(&self, peer_id: &PeerId) -> Option<Peer> {
        todo!("Get peer by peer_id")
    }
    fn select_peers(&self, peer_selection: PeerSelection) -> Vec<PeerId> {
        todo!("Select peers")
    }
}
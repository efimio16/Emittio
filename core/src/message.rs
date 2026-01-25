use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};

use crate::{payload::{Payload, Query, Reply}, peer::PeerId};

pub type MsgId = u64;

#[derive(Serialize, Deserialize)]
pub struct IncomingMessage {
    pub from: PeerId,
    pub payload: Payload,
    pub id: MsgId,
}

impl IncomingMessage {
    pub fn receive(from: PeerId, msg: OutgoingMessage) -> Self {
        Self { from, payload: msg.payload, id: msg.id }
    }
    pub fn reply(&self, reply: Reply) -> OutgoingMessage {
        OutgoingMessage { to: self.from, payload: Payload::Reply(reply), id: self.id }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OutgoingMessage {
    pub to: PeerId,
    pub payload: Payload,
    pub id: MsgId,
}

impl OutgoingMessage {
    pub fn new(to: &PeerId, payload: Payload) -> Self {
        Self { to: to.clone(), payload, id: OsRng.next_u64() }
    }
    pub fn query(to: &PeerId, q: impl Into<Query>) -> Self {
        Self::new(to, Payload::Query(q.into()))
    }
    pub fn reply(to: &PeerId, r: impl Into<Reply>) -> Self {
        Self::new(to, Payload::Reply(r.into()))
    }
}

pub struct OutgoingMessageBuilder {
    to: PeerId,
    id: MsgId,
}

impl OutgoingMessageBuilder {
    pub fn new(incoming: &IncomingMessage) -> Self {
        Self { to: incoming.from, id: incoming.id }
    }
    pub fn reply(self, reply: Reply) -> OutgoingMessage {
        OutgoingMessage { to: self.to, payload: Payload::Reply(reply), id: self.id }
    }
}
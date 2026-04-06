use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};

use crypto::id::Id;
use crate::{payload::{Payload, Query, Reply}};

pub type MsgId = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessage {
    pub from: Id,
    pub payload: Payload,
    pub id: MsgId,
}

impl IncomingMessage {
    pub fn receive(from: Id, msg: OutgoingMessage) -> Self {
        Self { from, payload: msg.payload, id: msg.id }
    }
    pub fn reply(self, reply: Reply) -> OutgoingMessage {
        OutgoingMessage { to: self.from, payload: Payload::Reply(reply), id: self.id }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutgoingMessage {
    pub to: Id,
    pub payload: Payload,
    pub id: MsgId,
}

impl OutgoingMessage {
    pub fn new(to: &Id, payload: Payload) -> Self {
        Self { to: to.clone(), payload, id: OsRng.next_u64() }
    }
    pub fn query(to: &Id, q: impl Into<Query>) -> Self {
        Self::new(to, Payload::Query(q.into()))
    }
    pub fn reply(to: &Id, r: impl Into<Reply>) -> Self {
        Self::new(to, Payload::Reply(r.into()))
    }
}

pub struct OutgoingMessageBuilder {
    to: Id,
    id: MsgId,
}

impl OutgoingMessageBuilder {
    pub fn new(incoming: &IncomingMessage) -> Self {
        Self { to: incoming.from.clone(), id: incoming.id }
    }
    pub fn reply(self, reply: Reply) -> OutgoingMessage {
        OutgoingMessage { to: self.to, payload: Payload::Reply(reply), id: self.id }
    }
}
use serde::{Deserialize, Serialize};

use crate::{peer::PeerId, pow::Pow, tag::Tag};

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
        OutgoingMessage { to: self.from.clone(), payload: Payload::Reply(reply), id: self.id }
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
        Self { to: to.clone(), payload, id: rand::random() }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Payload {
    Query(Query),
    Reply(Reply),
}

#[derive(Serialize, Deserialize)]
pub enum Query {
    GetTags,
    PublishTag {
        tag: Tag,
        pow: Pow,
        nonce: u64,
    },
    GetPow(Action),
}

#[derive(Serialize, Deserialize)]
pub enum Reply {
    Empty,
    Ok,
    Err(String),
    RequirePow(Pow),
    ReturnTags(Vec<Tag>),
}

impl Reply {
    // pub fn is_err(&self) -> bool {
    //     matches!(self, Self::Err(_))
    // }
    pub fn as_ok(self) -> Result<Self, String> {
        match self {
            Self::Err(msg) => Err(msg.clone()),
            _ => Ok(self),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(u16)]
pub enum Action {
    PublishTag = 1,
}

impl Action {
    pub fn value(&self) -> u16 {
        *self as u16
    }
}
mod pow;
mod tag;
mod dht;

pub use pow::*;
pub use tag::*;
pub use dht::*;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    Query(Query),
    Reply(Reply),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Query {
    Pow(PowQuery),
    Tag(TagQuery),
    Dht(DhtQuery),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Reply {
    Empty,
    Ok,
    Pow(PowReply),
    Tag(TagReply),
    Dht(DhtReply),
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("unexpected query type")]
    UnexpectedQueryType,
}

#[derive(Debug, Error)]
pub enum ReplyError {
    #[error("unexpected reply type")]
    UnexpectedReplyType,
}

pub trait TryFromQuery: TryFrom<Query, Error = QueryError> {}
pub trait TryFromReply: TryFrom<Reply, Error = ReplyError> {}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[repr(u16)]
pub enum Action {
    PublishTag = 1,
}

impl Action {
    pub fn value(&self) -> u16 { *self as u16 }
}
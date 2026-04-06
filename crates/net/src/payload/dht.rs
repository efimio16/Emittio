use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::{dht::CID, payload::{Query, QueryError, Reply, ReplyError, TryFromQuery, TryFromReply}};

#[derive(Serialize, Deserialize, Debug)]
pub enum DhtQuery {
    Get(CID),
    Put(Bytes),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DhtReply {
    Return(Option<Bytes>),
}

impl From<DhtQuery> for Query {
    fn from(value: DhtQuery) -> Self { Self::Dht(value) }
}
impl From<DhtReply> for Reply {
    fn from(value: DhtReply) -> Self { Self::Dht(value) }
}

impl TryFrom<Query> for DhtQuery {
    type Error = QueryError;
    fn try_from(value: Query) -> Result<Self, Self::Error> { match value { Query::Dht(q) => Ok(q), _ => Err(QueryError::UnexpectedQueryType) }}
}
impl TryFromQuery for DhtQuery {}

impl TryFrom<Reply> for DhtReply {
    type Error = ReplyError;
    fn try_from(value: Reply) -> Result<Self, Self::Error> { match value { Reply::Dht(r) => Ok(r), _ => Err(ReplyError::UnexpectedReplyType) }}
}
impl TryFromReply for DhtReply {}
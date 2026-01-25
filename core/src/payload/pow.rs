use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{payload::{Action, Query, QueryError, Reply, ReplyError, TryFromQuery, TryFromReply}, pow::Pow};

#[derive(Serialize, Deserialize)]
pub enum PowQuery {
    Get(Action),
}

#[derive(Serialize, Deserialize)]
pub enum PowReply {
    Require(Pow),
    Err(PowReplyErr),
}

#[derive(Serialize, Deserialize, Debug, Error)]
pub enum PowReplyErr {
    #[error("Incorrect nonce")]
    IncorrectNonce,
}

impl From<PowQuery> for Query {
    fn from(value: PowQuery) -> Self { Self::Pow(value) }
}
impl From<PowReply> for Reply {
    fn from(value: PowReply) -> Self { Self::Pow(value) }
}
impl From<PowReplyErr> for Reply {
    fn from(value: PowReplyErr) -> Self { Self::Pow(PowReply::Err(value)) }
}

impl TryFrom<Query> for PowQuery {
    type Error = QueryError;
    fn try_from(value: Query) -> Result<Self, Self::Error> { match value { Query::Pow(q) => Ok(q), _ => Err(QueryError::UnexpectedQueryType) }}
}
impl TryFromQuery for PowQuery {}

impl TryFrom<Reply> for PowReply {
    type Error = ReplyError;
    fn try_from(value: Reply) -> Result<Self, Self::Error> { match value { Reply::Pow(r) => Ok(r), _ => Err(ReplyError::UnexpectedReplyType) }}
}
impl TryFromReply for PowReply {}
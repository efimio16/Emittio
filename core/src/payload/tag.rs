use serde::{Deserialize, Serialize};

use crate::{payload::{Query, QueryError, Reply, ReplyError, TryFromQuery, TryFromReply}, pow::Pow, tag::Tag};

#[derive(Serialize, Deserialize, Debug)]
pub enum TagQuery {
    Get,
    Publish {
        tag: Tag,
        pow: Pow,
        nonce: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TagReply {
    Return(Vec<Tag>),
}

impl From<TagQuery> for Query {
    fn from(value: TagQuery) -> Self { Self::Tag(value) }
}
impl From<TagReply> for Reply {
    fn from(value: TagReply) -> Self { Self::Tag(value) }
}

impl TryFrom<Query> for TagQuery {
    type Error = QueryError;
    fn try_from(value: Query) -> Result<Self, Self::Error> { match value { Query::Tag(q) => Ok(q), _ => Err(QueryError::UnexpectedQueryType) }}
}
impl TryFromQuery for TagQuery {}

impl TryFrom<Reply> for TagReply {
    type Error = ReplyError;
    fn try_from(value: Reply) -> Result<Self, Self::Error> { match value { Reply::Tag(r) => Ok(r), _ => Err(ReplyError::UnexpectedReplyType) }}
}
impl TryFromReply for TagReply {}
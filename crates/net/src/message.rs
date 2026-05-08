use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};

use crypto::id::Id;
use crate::{payload::{Payload, Query, Reply}};

pub type MsgId = u64;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message {
    pub id: MsgId,
    pub from: Id,
    pub payload: Payload,
    pub to: Id,
}

impl Message {

    pub fn new(from: &Id, to: &Id, payload: Payload) -> Self {
        Message{
            from: from.clone(),
            id: OsRng.next_u64(),
            payload,
            to: to.clone(),
        }
    }
    pub fn query(from: &Id, to: &Id, q: impl Into<Query>) -> Self {
        Self::new(from, to, Payload::Query(q.into()))
    }

    pub fn reply(&self, reply: impl Into<Reply>) -> Self {
        // At the moment I think it only makes sense to reply to an existing message
        // I don't think we need to consume the original message - but that's up for debate
        Self { from: self.to.clone(), to: self.from.clone(), payload: Payload::Reply(reply.into()), id: self.id }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::payload::{TestQuery,TestReply};

    #[test]
    fn new_message() {
        let alice = Id::new([0u8; 32]);
        let bob = Id::new([1u8; 32]);
        let payload = Payload::Query(Query::Mock(TestQuery::Ping));
        let expect = Message{
            from: alice.clone(),
            id: 1234567890,
            payload: payload.clone(),
            to: bob.clone(),
        };

        let got = Message::new(&alice, &bob, payload);

        // ID is assigned randomly so can't compare structs directly
        assert_eq!(expect.from,got.from,"From should match");
        assert_eq!(expect.to,got.to,"To should match");
        assert_eq!(expect.payload,got.payload,"Payload should match");
    }

    #[test]
    fn test_query() {
        let alice = Id::new([0u8; 32]);
        let bob = Id::new([1u8; 32]);
        let payload = Payload::Query(Query::Mock(TestQuery::Ping));
        let expect = Message{
            from: alice.clone(),
            id: 1234567890,
            payload: payload.clone(),
            to: bob.clone(),
        };

        let got = Message::new(&alice, &bob, payload);

        // ID is assigned randomly so can't compare structs directly
        assert_eq!(expect.from,got.from,"From should match");
        assert_eq!(expect.to,got.to,"To should match");
        assert_eq!(expect.payload,got.payload,"Payload should match");
    }

    #[test]
    fn test_reply() {
        let alice = Id::new([0u8; 32]);
        let bob = Id::new([1u8; 32]);
        let payload = Payload::Query(Query::Mock(TestQuery::Ping));
        let message = Message{
            from: alice.clone(),
            id: 1234567890,
            payload,
            to: bob.clone(),
        };

        let expect = Message {
            from: bob.clone(),
            id: 1234567890,
            payload: Payload::Reply(Reply::Mock(TestReply::Pong)),
            to: alice.clone(),
        };

        // ID should match so we can compare directly
        let got = message.reply(Reply::Mock(TestReply::Pong));
        assert_eq!(expect,got,"Reply should match");
    }
}
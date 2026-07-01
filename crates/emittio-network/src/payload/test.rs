use serde::{Deserialize, Serialize};

use super::{Query, Reply};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TestQuery {
    Ping,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum TestReply {
    Pong,
}

impl From<TestQuery> for Query {
    fn from(value: TestQuery) -> Self { Self::Test(value) }
}
impl From<TestReply> for Reply {
    fn from(value: TestReply) -> Self { Self::Test(value) }
}

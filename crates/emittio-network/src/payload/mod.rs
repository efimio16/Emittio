mod test;

pub use test::*;

use serde::{Deserialize, Serialize};
// use crate::actor::T;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Query {
    Test(TestQuery),
    GetPointers()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Reply {
    Test(TestReply),
}
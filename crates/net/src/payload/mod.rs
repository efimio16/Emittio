mod test;

pub use test::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Query {
    Test(TestQuery),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Reply {
    Test(TestReply),
}
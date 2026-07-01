use emittio_crypto::{id::Id, tag::Tag};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pointer {
    tag: Tag,
    cid: Id,
}

pub type BlockTime = u64;
pub const BLOCK_DURATION_IN_SECS: u64 = 15 * 60;
pub const MAX_POINTERS_IN_BLOCK: u64 = 16000;

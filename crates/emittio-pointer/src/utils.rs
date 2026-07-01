use std::time::UNIX_EPOCH;

use crate::types::{BLOCK_DURATION_IN_SECS, BlockTime};

pub fn block_time(current_time: u64) -> BlockTime {
    current_time / BLOCK_DURATION_IN_SECS
}

pub fn current_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_secs()
}
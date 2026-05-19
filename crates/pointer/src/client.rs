use crypto::kem::PublicKey;
use net::manager::NetworkHandle;

use crate::types::PointerBody;

const BLOCK_DURATION: u64 = 15 * 60;

pub type Block = u64;

pub struct PointerClient {
    network: NetworkHandle,
    last_refresh_time: u64,
}

impl PointerClient {
    pub async fn pull(&mut self, address: PublicKey) -> Vec<PointerBody> {
        todo!("Pulls all the pointers, filter, scan...")
    }

    pub async fn push(&mut self) {
        todo!("Pushes a new pointer")
    }

    async fn bucket(&self) {
        todo!("Selects bucket")
    }

    fn current_block(&self) -> Block {
        self.last_refresh_time / BLOCK_DURATION
    }
}
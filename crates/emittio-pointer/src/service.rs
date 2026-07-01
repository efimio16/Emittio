use std::{collections::{BTreeMap, HashMap, HashSet}, path::PathBuf};

use emittio_crypto::id::{Id, Mask};
use emittio_network::types::Handler;
use tokio::io;

use crate::{query::{CountPointers, GetPointers, PutPointer}, types::{BlockTime, MAX_POINTERS_IN_BLOCK, Pointer}, utils::{block_time, current_time}};

struct Block {
    count: u64,
    buckets: HashMap<Id, Vec<Pointer>>,
    changed: HashSet<Id>,
    mask: Mask,
}

pub struct PointerStorage {
    dir: PathBuf,
    blocks: BTreeMap<BlockTime, Block>,
}

impl PointerStorage {
    fn get_bucket(&self, time: &BlockTime, bucket: &Bucket) -> Option<&Vec<Pointer>> {
        self.blocks.get(time)
            .map(|b| b.buckets.get(bucket))
            .flatten()
    }
}

impl Handler<CountPointers> for PointerStorage {
    async fn handle(&self, query: CountPointers) -> u64 {
        self.blocks.get(&query.time).map(|b| b.count).unwrap_or(0)
    }
}

impl Handler<GetPointers> for PointerStorage {
    async fn handle(&self, query: GetPointers) -> Vec<Pointer> {
        let Some(bucket) = self.get_bucket(&query.time, &query.bucket) else {
            return Vec::new()
        };

        let from = bucket.len().min(query.cursor as usize);
        let to = bucket.len().min(from + query.count as usize);

        Vec::from(&bucket[from..to])
    }
}

impl Handler<PutPointer> for PointerStorage {
    async fn handle(&mut self, query: PutPointer) {
        let block_time = block_time(current_time());

        let block = match self.blocks.get_mut(&block_time) {
            Some(block) => block,
            None => {
                let previous_pointer_count = self.blocks.last_entry()
                    .map(|e| e.get())
                    .map(|b| b.count)
                    .unwrap_or(0);

                let block = Block {
                    count: 0,
                    buckets: HashMap::new(),
                    changed: HashSet::new(),
                    mask: Mask::new_hex_mask(MAX_POINTERS_IN_BLOCK, previous_pointer_count),
                };

                self.blocks.insert(block_time, block);
                &mut self.blocks[block_time]
            },
        };

        let bucket_key = query.bucket.bucket(&block.mask); // Normalize bucket to avoid 

        let bucket = match block.buckets.get_mut(&bucket_key) {
            Some(bucket) => bucket,
            None => {
                block.buckets.insert(bucket_key, Vec::new());
                &mut block.buckets[bucket_key]
            },
        };

        bucket.push(query.pointer);
        block.count += 1;
        block.changed.insert(bucket_key);
    }
}
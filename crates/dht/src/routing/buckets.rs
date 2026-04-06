use std::array;
use heapless::{Vec, Deque};

use crate::{dht::routing::BucketsError, id::GenericId, peer::PeerId};

pub(super) struct Buckets<const B: usize, const P: usize> {
    buckets: [Deque<PeerId, P>; B],
    id: PeerId,
}

impl<const B: usize, const P: usize> Buckets<B, P> {
    pub fn new(id: PeerId) -> Self {
        Self { buckets: array::from_fn(|_| Deque::new()), id }
    }
    pub fn fill(&mut self, peers: &[PeerId]) -> Result<(), BucketsError> {
        for peer in peers {
            let Some(index) = self.id.bucket_for(peer) else { continue; };
            let Some(bucket) = self.buckets.get_mut(index) else { continue; };

            bucket.retain(|p| p != peer);
            if bucket.len() == P {
                bucket.pop_front();
            }
            bucket.push_back(*peer).map_err(|_| BucketsError::Overflow)?;
        }
        Ok(())
    }
    pub fn get_closest_peers<const K: usize>(&self, other: &impl AsRef<[u8; 32]>, count: usize) -> Result<Vec<PeerId, K>, BucketsError> {
        let mut peers = Vec::new();
        for i in self.bucket_order(self.id.bucket_for(other).unwrap_or(0)) {
            let Some(bucket) = self.buckets.get(i) else { continue; };
            for &peer in bucket {
                peers.push(peer).map_err(|_| BucketsError::Overflow)?;
                if peers.len() == count {
                    return Ok(peers);
                }
            }
        }
        Ok(peers)
    }
    fn bucket_order(&self, base: usize) -> impl Iterator<Item = usize> {
        std::iter::once(base).chain(
            (1..B).flat_map(move |d| {
                [
                    base.checked_sub(d),
                    base.checked_add(d).filter(|&i| i < B),
                ]
                .into_iter()
                .flatten()
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::GenericId;

    fn pid(n: u8) -> PeerId {
        let mut id = [0u8; 32];
        id[31] = n;
        id.into()
    }

    fn pid_from(mut id: [u8; 32], n: u8) -> PeerId {
        id[31] = n;
        id.into()
    }

    const B: usize = 256;
    const P: usize = 3;

    #[test]
    fn fill_places_peers_into_buckets() {
        let me = pid(0);
        let mut buckets: Buckets<B, P> = Buckets::new(me);

        let peers = [pid(1), pid(2), pid(3)];
        buckets.fill(&peers).unwrap();

        for p in peers {
            let idx = me.bucket_for(&p).unwrap();
            assert!(buckets.buckets[idx].iter().any(|e| e == &p));
        }
    }

    #[test]
    fn fill_deduplicates_peer() {
        let me = pid(0);
        let mut buckets: Buckets<B, P> = Buckets::new(me);

        let peer = pid(1);
        buckets.fill(&[peer, peer]).unwrap();

        let idx = me.bucket_for(&peer).unwrap();
        assert_eq!(buckets.buckets[idx].len(), 1);
    }

    #[test]
    fn fill_evicts_oldest_when_full() {
        let me = pid(0);
        let mut buckets: Buckets<B, P> = Buckets::new(me);

        let p1 = pid_from([1u8; 32], 1);
        let p2 = pid_from([1u8; 32], 2);
        let p3 = pid_from([1u8; 32], 3);
        let p4 = pid_from([1u8; 32], 4);

        buckets.fill(&[p1, p2, p3]).unwrap();
        buckets.fill(&[p4]).unwrap();

        let idx = me.bucket_for(&p1).unwrap();
        let bucket = &buckets.buckets[idx];

        assert!(!bucket.iter().any(|e| e == &p1)); // evicted
        assert!(bucket.iter().any(|e| e == &p4));
    }

    #[test]
    fn bucket_order_is_symmetric() {
        let me = pid(0);
        let buckets: Buckets<B, P> = Buckets::new(me);

        let order: Vec<usize, 10> = buckets.bucket_order(10).take(5).collect();

        assert_eq!(order, Vec::<_, 10>::from_array([10, 9, 11, 8, 12]));
    }

    #[test]
    fn get_closest_peers_collects_from_multiple_buckets() {
        let me = pid(0);
        let mut buckets: Buckets<B, P> = Buckets::new(me);

        let p1 = pid(1);
        let p2 = pid(2);
        let p3 = pid(3);

        buckets.fill(&[p1, p2, p3]).unwrap();

        let closest: Vec<PeerId, 10> = buckets.get_closest_peers::<10>(&pid(1), 3).unwrap();

        assert!(!closest.is_empty());
        assert!(closest.contains(&p1));
    }

    #[test]
    fn get_closest_peers_overflow() {
        const K: usize = 1;
        let me = pid(0);
        let mut buckets: Buckets<B, P> = Buckets::new(me);

        buckets.fill(&[pid(1), pid(2)]).unwrap();

        let res = buckets.get_closest_peers::<K>(&pid(1), 3);
        assert!(matches!(res, Err(BucketsError::Overflow)));
    }
}
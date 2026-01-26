use heapless::Vec;

use crate::{dht::{CID, routing::MAX_PEERS}, id::GenericId, peer::PeerId};

pub fn includes_id(id: &PeerId, cid: &CID, peers: &Vec<PeerId, MAX_PEERS>) -> bool {
    if peers.len() == 0 {
        return true;
    }

    let self_distance = id.distance(cid);

    for peer in peers {
        let distance = peer.distance(cid);
        if distance > self_distance {
            return true;
        }
    }
    false
}
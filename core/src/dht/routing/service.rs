use std::time::Duration;
use tokio::{sync::mpsc, time::interval};
use tokio_util::sync::CancellationToken;

use crate::{dht::{routing::Buckets, DhtRoutingDispatcher, DhtRoutingError, routing::DhtRoutingCmd}, peer::{PeerId, PeerTableDispatcher}, service::Service, utils::ChannelError};

const CHAN_SIZE: usize = 100;
#[cfg(not(test))]
const TICK_INTERVAL: Duration = Duration::from_mins(5);
#[cfg(test)]
const TICK_INTERVAL: Duration = Duration::from_secs(1);

const B: usize = 256;
const P: usize = 2;
const K: usize = 3;

pub struct DhtRouting {
    rx: mpsc::Receiver<DhtRoutingCmd>,
    buckets: Buckets<B, P>,
    peer_dispatcher: PeerTableDispatcher,
}

impl DhtRouting {
    pub fn new(peer_id: PeerId, peer_dispatcher: PeerTableDispatcher) ->(Self, DhtRoutingDispatcher) {
        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        (Self { rx, buckets: Buckets::new(peer_id), peer_dispatcher }, DhtRoutingDispatcher { tx })
    }
}

impl Service for DhtRouting {
    type Error = DhtRoutingError;
    async fn run(mut self, token: CancellationToken) -> Result<(), DhtRoutingError> {
        println!("Running DHT routing");

        let mut ticker = interval(TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    return Ok(())
                }
                _ = ticker.tick() => {
                    let peers = self.peer_dispatcher.get_all_ids().await?;
                    self.buckets.fill(&peers)?;
                }
                Some(cmd) = self.rx.recv() => {
                    match cmd {
                        DhtRoutingCmd::ClosestPeers { cid, reply } => {
                            reply.send(self.buckets.get_closest_peers(&cid, K)?).map_err(|_| ChannelError::Closed)?;
                        },
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use tokio_util::sync::CancellationToken;

    use crate::{dht::{CID, DhtRouting}, net::NetIdentity, peer::{Peer, PeerId, PeerTable, PeerTableDispatcher}, service::Service};
    use super::TICK_INTERVAL;

    fn create_peer_table() -> PeerTableDispatcher {
        let (peer_table, peer_table_dispatcher) = PeerTable::new();

        tokio::spawn(peer_table.run(CancellationToken::new()));

        peer_table_dispatcher
    }

    fn pid(n: u8) -> PeerId {
        let mut id = [0u8; 32];
        id[31] = n;
        id.into()
    }

    fn peer(n: u8) -> Peer {
        let mut id = [0u8; 32];
        id[31] = n;
        
        Peer { id: id.into(), address: "0.0.0.0".into(), identity: NetIdentity { x_pk: [1u8; 32], kb_pk: [1u8; 800] } }
    }

    #[tokio::test]
    async fn test_dht_storage() {
        let peer_table = create_peer_table();

        let me = peer(0);

        let other = peer(1);
        let other_id = other.id;

        peer_table.add_peer(other).await.expect("failed to add peer");
        
        let (dht_routing, mut dht_routing_dispatcher) = DhtRouting::new(me.id, peer_table);
    
        let token = CancellationToken::new();
        let handle = tokio::spawn(dht_routing.run(token.clone()));

        sleep(Duration::from_secs(1)).await;

        assert_eq!(dht_routing_dispatcher.closest_peers(CID::new(b"new CID")).await.expect("failed to get closest peers").get(0), Some(&other_id));
        
        token.cancel();
        handle.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn dht_refreshes_buckets() {
        let peer_table = create_peer_table();

        let me = peer(0);

        peer_table.add_peer(peer(1)).await.expect("failed to add peer");
        peer_table.add_peer(peer(2)).await.expect("failed to add peer");

        let (service, mut api) = DhtRouting::new(me.id, peer_table);

        let token = CancellationToken::new();
        let handle = tokio::spawn(service.run(token.clone()));

        sleep(TICK_INTERVAL * 2).await;

        let closest = api.closest_peers(CID::new(pid(1).as_ref())).await.unwrap();
        assert!(closest.contains(&pid(1)));

        token.cancel();
        handle.await.unwrap().unwrap();
    }
    }
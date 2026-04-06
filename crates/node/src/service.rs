use tokio::time::timeout;
use std::{collections::HashMap, time::Duration};
use futures::{StreamExt, stream::FuturesUnordered};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

use crate::{dht::{CID, DhtRouting, DhtRoutingDispatcher, DhtStorageDispatcher, includes_id}, message::{MsgId, OutgoingMessage, OutgoingMessageBuilder}, net::NetClient, node::{NodeCmd, NodeDispatcher, NodeError}, payload::{DhtQuery, DhtReply, Payload, PowQuery, PowReply, PowReplyErr, Query, Reply, TagQuery, TagReply}, peer::PeerTableDispatcher, pow::Pow, service::Service, tag::TagDispatcher, transport::TransportDispatcher, utils::ChannelError};

const CHAN_SIZE: usize = 128;
const TIMEOUT: Duration = Duration::from_secs(5);

pub struct NodeService {
    secret: [u8; 32],
    rx: mpsc::Receiver<NodeCmd>,
    tag: TagDispatcher,
    transport: TransportDispatcher,
    pending: HashMap<MsgId, oneshot::Sender<Reply>>, // TODO: TTL
    dht_storage: DhtStorageDispatcher,
    dht_routing: DhtRoutingDispatcher,
}

impl NodeService {
    pub fn new(
        transport: TransportDispatcher,
        tag: TagDispatcher,
        secret: [u8; 32],
        dht_storage: DhtStorageDispatcher,
        peer_dispatcher: PeerTableDispatcher,
    ) -> (NodeDispatcher, Self, DhtRouting) {
        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        let peer_id = NetClient::from_seed(secret.clone()).identity().expect("node shoud have static identity").peer_id();

        let (dht_routing_service, dht_routing) = DhtRouting::new(peer_id, peer_dispatcher);

        (
            NodeDispatcher::new(secret, tx),
            Self { tag, rx, secret: secret.clone(), transport, pending: HashMap::new(), dht_storage, dht_routing },
            dht_routing_service
        )
    }

    fn net_client(&self) -> NetClient {
        NetClient::from_seed(self.secret)
    }

    async fn reply<R: Into<Reply>>(&mut self, builder: OutgoingMessageBuilder, reply: R) -> Result<(), ChannelError> {
        self.transport.send(builder.reply(reply.into())).await
    }
}

impl Service for NodeService {
    type Error = NodeError;
    
    async fn run(mut self, token: CancellationToken) -> Result<(), NodeError> {
        println!("Running node");

        let mut futures = FuturesUnordered::new();

        let peer_id = self.net_client().identity().expect("node should have a static identity").peer_id();

        loop {
            tokio::select! {
                _ = token.cancelled() => { return Ok(()); }
                Some(msg) = self.transport.recv() => {
                    let builder = OutgoingMessageBuilder::new(&msg);
                    match msg.payload {
                        Payload::Query(q) => {
                            match q {
                                Query::Tag(q) => {
                                    match q {
                                        TagQuery::Get => {
                                            let tags = self.tag.get_tags().await?;
                                            self.reply(builder, TagReply::Return(tags)).await?;
                                        }
                                        TagQuery::Publish { tag, pow, nonce } => {
                                            if pow.verify_with_secret(&self.secret, nonce) {
                                                self.tag.put_tag(tag.clone()).await?;

                                                self.reply(builder, Reply::Ok).await?;
                                            } else {
                                                self.reply(builder, PowReplyErr::IncorrectNonce).await?;
                                            }
                                        }
                                    }
                                }
                                Query::Pow(q) => {
                                    match q {
                                        PowQuery::Get(action) => self.reply(builder, PowReply::Require(Pow::new(&self.secret, action, 16))).await?,
                                    }
                                }
                                Query::Dht(q) => {
                                    match q {
                                        DhtQuery::Get(cid) => {
                                            if let Some(content) = self.dht_storage.get(cid.clone()).await? {
                                                self.reply(builder, DhtReply::Return(Some(content))).await?;
                                            } else {
                                                let peers = self.dht_routing.closest_peers(cid.clone()).await?;

                                                for peer in peers {
                                                    let msg = OutgoingMessage::query(&peer, DhtQuery::Get(cid.clone()));

                                                    let (tx, rx) = oneshot::channel();
                                                    self.pending.insert(msg.id, tx);

                                                    self.transport.send(msg).await?;
                                                    futures.push(async move { 
                                                        timeout(TIMEOUT, rx).await
                                                    });
                                                }
                                            }
                                        }
                                        DhtQuery::Put(content) => {
                                            let cid = CID::new(&content);
                                            let peers = self.dht_routing.closest_peers(cid.clone()).await?;

                                            let includes_self = includes_id(&peer_id, &cid, &peers);

                                            for peer in peers {
                                                let msg = OutgoingMessage::query(&peer, DhtQuery::Get(cid.clone()));
                                                self.transport.send(msg).await?;
                                            }

                                            if includes_self {
                                                self.dht_storage.put(cid, content).await?;
                                            }

                                            self.reply(builder, Reply::Ok).await?;
                                        }
                                    }
                                }
                            }
                        }
                        Payload::Reply(r) => {
                            if let Some(tx) = self.pending.remove(&msg.id) {
                                tx.send(r).map_err(|_| ChannelError::Closed)?;
                            }
                        }
                    }
                }
                Some(cmd) = self.rx.recv() => {
                    self.pending.insert(cmd.msg.id, cmd.reply_tx);
                    self.transport.send(cmd.msg).await?;
                }
                Some(res) = futures.next() => { res??; }
            }
        }
    }
}
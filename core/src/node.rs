use rand::{RngCore, rngs::OsRng};
use tokio_util::sync::CancellationToken;

use crate::{channels::{self, ChannelError}, message::{IncomingMessage, OutgoingMessage, Payload, Query, Reply}, net::client::NetClient, peer::Peer, pow::Pow, service::Service, tag_service::TagDispatcher, transport::{TransportDispatcher, TransportHandler}};

pub struct Node {
    secret: [u8; 32],
    tag_dispatcher: TagDispatcher,
    chans: TransportDispatcher,
}

impl Node {
    pub fn new(tag_dispatcher: TagDispatcher) -> (Self, NetClient, Peer, TransportHandler) {
        let (chans, handler) = channels::new(128);

        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);

        let (client, identity) = NetClient::from_seed(secret);

        (Self { tag_dispatcher, secret: secret.clone(), chans }, client, Peer::new(identity, Default::default()), handler)
    }

    pub async fn send(&mut self, message: OutgoingMessage) -> Result<(), ChannelError> {
        self.chans.send(message).await
    }
    pub async fn reply(&mut self, message: &IncomingMessage, reply: Reply) -> Result<(), ChannelError> {
        self.send(message.reply(reply)).await
    }
}

impl Service for Node {
    type Error = ChannelError;
    
    async fn run(mut self, token: CancellationToken) -> Result<(), ChannelError> {
        println!("Running node");

        loop {
            tokio::select! {
                _ = token.cancelled() => { return Ok(()); }
                Some(msg) = self.chans.recv() => {
                    if let Payload::Query(query) = &msg.payload {
                        match query {
                            Query::GetTags => {
                                let tags = self.tag_dispatcher.get_tags().await?;
                                self.reply(&msg, Reply::ReturnTags(tags)).await?;
                            },
                            Query::PublishTag { tag, pow, nonce } => {
                                if pow.verify_with_secret(&self.secret, *nonce) {
                                    println!("Tag to publish: {:#?}", tag.hash);
                                    self.tag_dispatcher.send_tag(tag.clone()).await?;
                                    self.reply(&msg, Reply::Ok).await?;
                                } else {
                                    self.reply(&msg, Reply::Err("Incorrect PoW".into())).await?;
                                }
                            },
                            Query::GetPow(action) => {
                                self.reply(&msg, Reply::RequirePow(Pow::new(&self.secret, *action, 16))).await?;
                            },
                        }
                    }
                }
            }
        }
    }
}
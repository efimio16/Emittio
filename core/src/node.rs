use crate::{channels::{self, ChannelError}, message::{IncomingMessage, OutgoingMessage, Payload, Query, Reply}, pow::Pow, tag_service::TagDispatcher, transport::{TransportDispatcher, TransportHandler}};

pub struct Node {
    secret: [u8; 32],
    tag_dispatcher: TagDispatcher,
    pub chans: TransportDispatcher,
}

impl Node {
    pub fn new(tag_dispatcher: TagDispatcher) -> (Self, TransportHandler) {
        let (chans, handler) = channels::new(128);

        (Self { tag_dispatcher, secret: rand::random(), chans }, handler)
    }

    pub async fn run(&mut self) -> Result<(), ChannelError> {
        while let Some(msg) = self.chans.recv().await {
            if let Payload::Query(query) = &msg.payload {
                match query {
                    Query::GetTags => {
                        let tags = self.tag_dispatcher.get_tags().await?;
                        self.reply(&msg, Reply::ReturnTags(tags)).await?;
                    },
                    Query::PublishTag { tag, pow, nonce } => {
                        if pow.verify_with_secret(&self.secret, *nonce) {
                            println!("Tag to publish: {:#?}", tag.clone());
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
        Ok(())
    }

    pub async fn send(&mut self, message: OutgoingMessage) -> Result<(), ChannelError> {
        self.chans.send(message).await
    }
    pub async fn reply(&mut self, message: &IncomingMessage, reply: Reply) -> Result<(), ChannelError> {
        self.send(message.reply(reply)).await
    }
}
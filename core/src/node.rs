use tokio::sync::oneshot;

use crate::{channels, message::{IncomingMessage, OutgoingMessage, Payload, Query, Reply}, pow::Pow, tag_service::TagDispatcher};

pub struct Node {
    secret: [u8; 32],
    tag_dispatcher: TagDispatcher,
    pub chans: channels::Dispatcher<IncomingMessage, OutgoingMessage>,
}

impl Node {
    pub fn new(tag_dispatcher: TagDispatcher) -> (Self, channels::Handler<OutgoingMessage, IncomingMessage>) {
        let (chans, handler) = channels::new(128);

        (Self { tag_dispatcher, secret: rand::random(), chans }, handler)
    }

    pub async fn run(&mut self) -> Result<(), String> {
        while let Some(msg) = self.chans.rx.recv().await {
            if let Payload::Query(query) = &msg.payload {
                match query {
                    Query::GetTags => {
                        let (tx, rx) = oneshot::channel();
                        self.tag_dispatcher.get_tx.send(tx).await.map_err(|_| "send failed".to_string())?;
                        self.reply(&msg, Reply::ReturnTags(rx.await.map_err(|_| "oneshot receive failed".to_string())?)).await?;
                    },
                    Query::PublishTag { tag, pow, nonce } => {
                        if pow.verify_with_secret(&self.secret, *nonce) {
                            println!("Tag to publish: {:#?}", tag.clone());
                            self.tag_dispatcher.tag_tx.send(tag.clone()).await.map_err(|_| "send failed".to_string())?;
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

    pub async fn send(&mut self, message: OutgoingMessage) -> Result<(), String> {
        self.chans.tx.send(message).await.map_err(|e| format!("Failed to send message: {:?}", e))?;
        Ok(())
    }
    pub async fn reply(&mut self, message: &IncomingMessage, reply: Reply) -> Result<(), String> {
        self.send(message.reply(reply)).await
    }
}
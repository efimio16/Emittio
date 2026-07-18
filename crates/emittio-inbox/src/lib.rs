use std::collections::HashMap;
use actorify::{actor, tokio::io::AsyncRead};
use emittio_crypto::{id::Id, kem::{Kem, PublicKey}, tag::{TagAddress, TagVerifier}};
use emittio_network::{actor::NetworkActorHandle, query::Query};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Message {
    pub from: Address,
	pub to: Id,
	pub subject: String,
	pub text_root: Vec<Id>,
	pub attachments: HashMap<String, u64>, // Names and locations of attachments
	pub attachment_root: Vec<Id>,
}

pub type Bucket = ();

#[derive(Clone, Deserialize, Serialize)]
pub struct Address {
    message_pk: PublicKey,
    tag_address: TagAddress,
}

pub type TimeBlock = u64;
const CHAN_SIZE: usize = 1024;
const TIME_BLOCK_SIZE: TimeBlock = 30 * 60;

pub struct InboxActor {
    message_sk: Kem,
    tag_verifier: TagVerifier,
    network: NetworkActorHandle,
    last_refresh_time: u64,
}

#[actor]
impl InboxActor {
    #[command]
    async fn send(&mut self, subject: String, to: Address, body: Box<dyn AsyncRead + Send + Unpin>, #[callback] cb: ()) {
        todo!()
    }

    #[command]
    async fn pull(&mut self, #[callback] cb: Vec<(Id, Message)>) {
        let block = self.time_block();
        let bucket = self.bucket();

        // let pointers = Vec::new();

        // self.network.query_stream(Query::)

        todo!()
    }

    pub fn new(network: NetworkActorHandle, message_sk: Kem, tag_verifier: TagVerifier) -> Self {
        Self {
            message_sk,
            network,
            tag_verifier,
            last_refresh_time: 0,
        }
    }

    fn time_block(&self) -> TimeBlock {
        self.last_refresh_time / TIME_BLOCK_SIZE
    }

    fn bucket(&self) -> Bucket {
        todo!()
    }
        
    async fn recv_message(&self) -> Message {
        todo!("Checks for new messages")
    }
    async fn send_message(&self, _message: Message) -> Id {
        todo!()
    }
    
    async fn get_text(&self, _chunks: Vec<Id>) -> Box<dyn AsyncRead + Send + Unpin> {
        todo!()
    }
    async fn put_text(&self, _stream: Box<dyn AsyncRead + Send + Unpin>) -> Vec<Id> {
        todo!()
    }

    // async fn get_attachment(&self, _chunks: Vec<Id>, _location: u64) -> Reader {
    //     todo!()
    // }
    // async fn put_attachments(&self, _attachments: HashMap<String, u64>, _stream: Reader) -> Vec<Id> {
    //     todo!()
    // }
}
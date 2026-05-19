use std::collections::HashMap;

use crypto::id::Id;

use crate::types::Message;

pub struct MessageClient {
    messages: HashMap<Id, Message>,
}

impl MessageClient {
    async fn recv_message() {
        todo!()
    }
    async fn send_message() {
        todo!()
    }

    async fn get_text() {
        todo!()
    }
    async fn get_attachment() {
        todo!()
    }

    async fn put_text() {
        todo!()
    }
    async fn put_attachment() {
        todo!()
    }
}
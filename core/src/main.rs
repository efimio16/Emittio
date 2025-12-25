mod bundles;
mod session;
mod inbox;
mod envelope;
mod utils;
mod transport;
mod peer;
mod client;
mod message;
mod pow;
mod node;
mod client_service;
mod tag;
mod tag_service;
mod channels;

use std::time::Instant;
use tokio;

use crate::{client::Client, message::Reply, node::Node, peer::PeerId, session::Session, tag::{TagManager, TagPayload}, tag_service::TagService, transport::MockTransport};


const VERSION: usize = 1;
#[tokio::main]
async fn main() {
    let start = Instant::now();

    let (mut tag_service, tag_dispatcher) = TagService::new("tags.bin".into());

    let transport = MockTransport::new();

    let (mut client, mut client_service, t_params) = Client::new();
    transport.add_peer(PeerId::new("client"), t_params);

    let node_id = PeerId::new("node");
    let (mut node, t_params) = Node::new(tag_dispatcher);
    transport.add_peer(node_id.clone(), t_params);

    tokio::spawn(async move { client_service.run().await });
    tokio::spawn(async move { node.run().await });
    tokio::spawn(async move { /*tag_service.load().await.expect("load tag service failed");*/ tag_service.run().await });

    let mut tag_manager = TagManager::new();
    if let Reply::ReturnTags(tags) = client.get_tags(&node_id).await.expect("get tags failed") {
        tag_manager.load_tags(tags).expect("load tags failed");
    }
    
    let owned_tag = tag_manager.new_tag(TagPayload { data: b"Hello!".into() }).expect("failed create tag");

    client.publish_tag(&node_id, owned_tag.tag).await.expect("Failed to publish tag");

    let alice = Session::new();
    let bob = Session::new();
    println!("Sessions initialized");

    let mut alice_inbox = alice.inbox(0);
    let bob_inbox = bob.inbox(0);
    println!("Inboxes initialized");

    let envelope = alice_inbox.new_envelope(bob_inbox.sender.public(), b"Hello, Bob! How are you?").expect("Failed to create envelope");
    println!("Encrypted: {:x?}", envelope.as_bytes());

    let plaintext = envelope.decrypt(bob_inbox.sender).unwrap();
    assert_eq!(&plaintext, b"Hello, Bob! How are you?");
    println!("Decrypted: {}", str::from_utf8(&plaintext).unwrap());
    
    println!("⏱ Completed in: {:?}", start.elapsed());
}

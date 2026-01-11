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
mod peer_table;
mod net;
mod service;

use std::time::Duration;
use tokio::time::sleep;

use crate::{client::Client, net::client::NetClient, node::Node, peer::PeerId, peer_table::PeerTable, service::ServiceManager, session::Session, tag::{TagManager, TagPayload}, tag_service::TagService, transport::MockTransport};

const VERSION: u8 = 1;

const DEFAULT_INBOX: u32 = 0;

#[tokio::main]
async fn main() {
    let (tag_service, tag_dispatcher) = TagService::new("tags.bin".into());
    let (peer_service, peer_dispatcher) = PeerTable::new();

    let mut transport = MockTransport::new(peer_dispatcher.clone());

    let (mut client, client_service, transport_handler) = Client::new();
    transport.add_peer(PeerId::new("client"), NetClient::Ephemeral, transport_handler);

    let (node, node_client, node_peer, transport_handler) = Node::new(tag_dispatcher);
    
    transport.add_peer(node_peer.id, node_client, transport_handler);

    peer_dispatcher.add_peer(node_peer.clone()).await.expect("add peer failed");

    let mut tag_manager = TagManager::new();

    let mut service_manager = ServiceManager::new();

    service_manager.spawn(client_service);
    service_manager.spawn(node);
    service_manager.spawn(tag_service);
    service_manager.spawn(peer_service);
    service_manager.spawn(transport);

    let main = async {

        let tags = client.get_tags(&node_peer.id).await.expect("get tags failed");

        tag_manager.load_tags(tags).expect("load tags failed");
        
        let owned_tag = tag_manager.new_tag(TagPayload { data: b"Hello!".into() }).expect("failed create tag");

        client.publish_tag(&node_peer.id, owned_tag.tag.clone()).await.expect("Failed to publish tag");

        println!("Waiting 5 seconds...");
        sleep(Duration::from_secs(5)).await;

        assert_eq!(client.get_tags(&node_peer.id).await.expect("get tags failed")[0].hash, owned_tag.tag.hash);

        // Alice-Bob example
        let alice = Session::new();
        let bob = Session::new();
        println!("Sessions initialized");

        let mut alice_inbox = alice.inbox(DEFAULT_INBOX);
        let bob_inbox = bob.inbox(DEFAULT_INBOX);
        println!("Inboxes initialized");

        let envelope = alice_inbox.new_envelope(bob_inbox.sender.public(), b"Hello, Bob! How are you?").expect("Failed to create envelope");
        println!("Encrypted: {:0x?}", envelope.as_bytes());

        let plaintext = envelope.decrypt(bob_inbox.sender).unwrap();
        assert_eq!(&plaintext, b"Hello, Bob! How are you?");
        println!("Decrypted: {}", str::from_utf8(&plaintext).unwrap());
    };

    let (mut handle, token) = service_manager.run();

    let ctrl_c = tokio::spawn(async move { tokio::signal::ctrl_c().await.expect("failed to listen to ctrl c event") });

    tokio::select! {
        _ = ctrl_c => {
            token.cancel();
            let _ = handle.await;
        }
        res = &mut handle => {
            match res {
                Ok(Ok(())) => { println!("service manager exited unexpectedly"); },
                Ok(Err(err)) => { panic!("{}", err); },
                Err(join_err) => { panic!("{}", join_err); }
            }
        }
        _ = main => {
            token.cancel();
            let _ = handle.await;
        }
    };
}

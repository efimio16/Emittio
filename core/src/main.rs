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
mod tag;
mod net;
mod service;
mod payload;
mod dht;
mod id;

use std::time::Duration;
use tokio::time::sleep;

use crate::{client::Client, dht::DhtStorage, envelope::Envelope, net::{NetClient, SessionManager}, node::NodeService, peer::PeerTable, service::ServiceManager, session::Session, tag::{TagManager, TagPayload, TagService}, transport::{MockTransport, TransportHandler}, utils::random_bytes};

const VERSION: u8 = 1;
const DEFAULT_INBOX: u32 = 0;

#[tokio::main]
async fn main() {
    let (tag_service, tag_dispatcher) = TagService::create("tags.bin".into()).await.expect("tag service init failed");
    let (peer_service, peer_dispatcher) = PeerTable::new();

    let mut service_manager = ServiceManager::new();

    let mut transport = MockTransport::new(peer_dispatcher.clone());

    let (session_manager_dispatcher, session_manager_service) = SessionManager::new(NetClient::Ephemeral);
    service_manager.spawn(session_manager_service);

    let (transport_handler, transport_dispatcher) = TransportHandler::new();
    let (mut client, client_service) = Client::new(transport_dispatcher);
    transport.add_participant(session_manager_dispatcher, None, transport_handler).await.expect("failed to add peer");

    let seed = random_bytes();
    let node_cl = NetClient::from_seed(seed.clone());
    let node_identity = node_cl.identity().expect("node should have static identity");
    let node_id = node_identity.peer_id();

    let (dht_storage_service, dht_storage_dispatcher) = DhtStorage::create("dht.bin".into()).await.expect("dht storage init failed");

    let (session_manager_dispatcher, session_manager_service) = SessionManager::new(node_cl);
    service_manager.spawn(session_manager_service);

    let (transport_handler, transport_dispatcher) = TransportHandler::new();

    let (_, node_service, dht_routing_service) = NodeService::new(transport_dispatcher, tag_dispatcher, random_bytes(), dht_storage_dispatcher, peer_dispatcher.clone());
    transport.add_participant(session_manager_dispatcher, Some(node_identity), transport_handler).await.expect("failed to add peer");

    let mut tag_manager = TagManager::new();

    service_manager.spawn(client_service);
    service_manager.spawn(node_service);
    service_manager.spawn(tag_service);
    service_manager.spawn(peer_service);
    service_manager.spawn(transport);
    service_manager.spawn(dht_storage_service);
    service_manager.spawn(dht_routing_service);

    let main = async {
        sleep(Duration::from_secs(1)).await;

        let tags = client.get_tags(&node_id).await.expect("get tags failed");

        tag_manager.load_tags(tags).expect("load tags failed");
        
        let owned_tag = tag_manager.new_tag(TagPayload { data: b"Hello!".into() }).expect("failed create tag");

        client.publish_tag(&node_id, owned_tag.tag.clone()).await.expect("failed to publish tag");

        println!("Waiting 5 seconds...");
        sleep(Duration::from_secs(5)).await;

        assert_eq!(client.get_tags(&node_id).await.expect("get tags failed")[0].hash, owned_tag.tag.hash);

        // Alice-Bob example
        let alice = Session::new();
        let bob = Session::new();
        println!("Sessions initialized");

        let alice_inbox = alice.inbox(DEFAULT_INBOX);
        let bob_inbox = bob.inbox(DEFAULT_INBOX);
        println!("Inboxes initialized");

        let envelope = Envelope::encrypt(
            b"Hello, Bob! How are you?",
            &alice_inbox.sender,
            &bob_inbox.sender.public(),
        ).expect("failed to create envelope");

        let envelope_cid = client.dht_put(&node_id, &envelope).await.expect("dht put failed");
        println!("Envelope saved");

        println!("Waiting 1 second...");
        sleep(Duration::from_secs(1)).await;

        let envelope: Envelope = client.dht_get(&node_id, envelope_cid).await.expect("dht get failed").expect("content not found");

        let plaintext = envelope.decrypt(&bob_inbox.sender).expect("decryption failed");
        assert_eq!(&plaintext, b"Hello, Bob! How are you?");
        println!("Decrypted: {}", str::from_utf8(&plaintext).expect("the slice should be UTF-8"));
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

mod bundles;
mod session;
mod inbox;
mod envelope;
mod utils;

use std::time::Instant;

use crate::{session::Session};

fn main() {
    let start = Instant::now();

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
    
    let duration = start.elapsed();
    println!("⏱ Completed in: {:?}", duration);
}

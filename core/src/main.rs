mod bundles;
mod session;
mod inbox;
mod envelope;
mod utils;

use std::time::Instant;

use crate::{session::Session};

fn main() {
    let start = Instant::now();

    let mut alice = Session::new();
    let mut bob = Session::new();

    let mut alice_to_bob = alice.new_inbox(bob.invite());
    let envelope = alice_to_bob.new_envelope(b"Hello, Bob! How are you?").expect("Failed to create envelope");
    
    let bob_to_alice = bob.inbox(0, alice_to_bob.sender.public());
    let plaintext = envelope.decrypt(bob_to_alice.sender).unwrap();

    assert_eq!(&plaintext, b"Hello, Bob! How are you?");
    
    let duration = start.elapsed();
    println!("⏱ Completed in: {:?}", duration);
}

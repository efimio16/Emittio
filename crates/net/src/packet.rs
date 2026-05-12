use crypto::{ciphertext::Ciphertext, kem::{Capsule, PublicKey}, id::Id};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

pub type ConnId = u64;

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub pk: PublicKey,
    #[serde(with = "BigArray")]
    pub capsule: Capsule,
}

#[derive(Serialize, Deserialize)]
pub struct WireMessage {
    pub seq: u64,
    // pub session_id: SessionId,
    pub ciphertext: Ciphertext,
}

pub type SessionId = Id;

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Handshake(Handshake),
    Message(WireMessage),
}
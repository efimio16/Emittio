use crypto::{ciphertext::Ciphertext, kem::{Capsule, PublicKey}};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

pub type ConnId = u64;

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub from: PublicKey,
    #[serde(with = "BigArray")]
    pub capsule: Capsule,
    pub created_conn_id: ConnId,
}

#[derive(Serialize, Deserialize)]
pub struct HandshakeAck {
    pub conn_id: ConnId,
    pub created_conn_id: ConnId,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub seq: u64,
    pub ciphertext: Ciphertext,
    pub conn_id: ConnId,
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Handshake(Handshake),
    HandshakeAck(HandshakeAck),
    Message(Message),
}
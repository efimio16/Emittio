use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::{net::types::{NetIdentity, KyberCiphertext}};

pub type ConnId = u64;

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub from: NetIdentity,
    #[serde(with = "BigArray")]
    pub ct: KyberCiphertext,
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
    pub body: Vec<u8>,
    pub conn_id: ConnId,
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Handshake(Handshake),
    HandshakeAck(HandshakeAck),
    Message(Message),
}
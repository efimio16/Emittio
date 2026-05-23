use bytes::Bytes;
use crypto::{kem::{Capsule, PublicKey}, ciphertext::Sealed};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::payload::{Query, Reply};

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub pk: PublicKey,
    #[serde(with = "BigArray")]
    pub capsule: Capsule,
}

pub type PayloadId = u64;

#[derive(Serialize, Deserialize)]
pub enum FrameData {
    Query(PayloadId, Query),
    Reply(PayloadId, Reply),
    Chunk(Bytes),
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub seq: u64,
    pub data: Sealed<FrameData>,
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Handshake(Handshake),
    Frame(Frame),
}
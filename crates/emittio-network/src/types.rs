use bytes::Bytes;
use emittio_crypto::{ciphertext::Sealed, id::Id, kem::{Capsule, PublicKey}};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::query::Query;

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub pk: PublicKey,
    #[serde(with = "BigArray")]
    pub capsule: Capsule,
}

pub type PayloadId = u64;

#[derive(Serialize, Deserialize)]
pub enum FrameData {
    Query(Query),
    Reply(Bytes),
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

pub struct RouteConfig {
    pub peers: PeerSelection,
    pub pow: PowConfig,
}

#[derive(Clone)]
pub enum PeerSelection {
    Closest {
        target: Id,
        count: u16,
    },
    Random(u8),
    InBucket {
        bucket: Id,
        max_count: u64,
    },
}

pub enum PowConfig {
    None,
    Low,
    Medium,
    High,
}

pub enum VerificationMethod {
    None,
    Median { tolerance: f32 },
    Hash(Id),
}

pub trait IntoQuery {
    const SERVICE_ID: u16;
    const METHOD_ID: u16;

    type Reply;

    fn route_config(&self) -> RouteConfig;
    fn verification_method(&self) -> VerificationMethod {
        VerificationMethod::None
    }
}

pub trait NetworkHandler<Q: IntoQuery> {
    fn handle(&mut self, query: Q) -> impl Future<Output = Q::Reply>;
}

// pub fn median<T>(values: &mut [T]) -> T {
//     values.sort();
//     values[values.len() / 2]
// }
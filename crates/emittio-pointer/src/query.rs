use emittio_crypto::id::Id;
use emittio_network::types::{PeersSelection, PowConfig, IntoQuery, RouteConfig, VerificationMethod};
use serde::{Deserialize, Serialize};

use crate::{POINTER_SERVICE_ID, types::{BlockTime, MAX_POINTERS_IN_BLOCK, Pointer}};

const MEDIAN_TOLERANCE: f32 = 0.05;

#[derive(Clone, Serialize, Deserialize)]
pub struct CountPointers {
    pub time: BlockTime,
}

impl IntoQuery for CountPointers {
    const SERVICE_ID: u16 = POINTER_SERVICE_ID;
    const METHOD_ID: u16 = 1;

    type Reply = u64;

    fn route_config(&self) -> RouteConfig {
        RouteConfig { peers: PeersSelection::Random(5), pow: PowConfig::None }
    }

    fn verification_method(&self) -> VerificationMethod {
        VerificationMethod::Median { tolerance: MEDIAN_TOLERANCE }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPointers {
    pub time: BlockTime,
    pub bucket: Id,
    pub cursor: u64,
    pub count: u64,
}

impl IntoQuery for GetPointers {
    const SERVICE_ID: u16 = POINTER_SERVICE_ID;
    const METHOD_ID: u16 = 2;

    type Reply = Vec<Pointer>;

    fn route_config(&self) -> RouteConfig {
        RouteConfig { peers: PeersSelection::InBucket { bucket: self.bucket.clone(), max_count: MAX_POINTERS_IN_BLOCK }, pow: PowConfig::None }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PutPointer {
    pub bucket: Id,
    pub pointer: Pointer,
}

impl IntoQuery for PutPointer {
    const SERVICE_ID: u16 = POINTER_SERVICE_ID;
    const METHOD_ID: u16 = 3;

    type Reply = ();

    fn route_config(&self) -> RouteConfig {
        RouteConfig { peers: PeersSelection::InBucket { bucket: self.bucket.clone(), max_count: MAX_POINTERS_IN_BLOCK }, pow: PowConfig::High }
    }
}

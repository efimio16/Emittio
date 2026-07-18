use bytes::Bytes;
use emittio_crypto::id::Id;
use emittio_network::types::{PeerSelection, PowConfig, IntoQuery, RouteConfig, VerificationMethod};
use serde::{Deserialize, Serialize};

use crate::{DHT_SERVICE_ID, error::{DhtGetError, DhtPutError}};

const REPLICATION: u16 = 5;

#[derive(Clone, Serialize, Deserialize)]
pub struct DhtGet {
    pub cid: Id,
}

impl IntoQuery for DhtGet {
    const SERVICE_ID: u16 = DHT_SERVICE_ID;
    const METHOD_ID: u16 = 1;
    type Reply = Result<Bytes, DhtGetError>;

    fn route_config(&self) -> RouteConfig {
        RouteConfig { peers: PeerSelection::Closest { target: self.cid.clone(), count: REPLICATION }, pow: PowConfig::High }
    }
    fn verification_method(&self) -> VerificationMethod {
        VerificationMethod::Hash(self.cid.clone())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DhtPut {
    pub bytes: Bytes,
}

impl IntoQuery for DhtPut {
    const SERVICE_ID: u16 = DHT_SERVICE_ID;
    const METHOD_ID: u16 = 2;
    type Reply = Result<(), DhtPutError>;

    fn route_config(&self) -> RouteConfig {
        RouteConfig { peers: PeerSelection::Closest { target: Id::hash_bytes(&self.bytes), count: REPLICATION }, pow: PowConfig::High }
    }
}

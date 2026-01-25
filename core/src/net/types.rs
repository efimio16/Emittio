use blake3::Hasher;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::{peer::PeerId, utils::KyberPublicKey};

#[derive(Serialize, Deserialize, Clone)]
pub struct NetIdentity {
    pub x_pk: [u8; 32],
    #[serde(with = "BigArray")]
    pub kb_pk: KyberPublicKey,
}

impl NetIdentity {
    pub fn peer_id(&self) -> PeerId {
        let mut hasher = Hasher::new();
        hasher.update(&self.x_pk);
        hasher.update(&self.kb_pk);
        
        PeerId::new(hasher.finalize().into())
    }
}
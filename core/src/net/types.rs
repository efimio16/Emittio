use blake3::Hasher;
use pqc_kyber::{KYBER_CIPHERTEXTBYTES, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::peer::PeerId;

pub type KyberSecretKey = [u8; KYBER_SECRETKEYBYTES];
pub type KyberPublicKey = [u8; KYBER_PUBLICKEYBYTES];
pub type KyberCiphertext = [u8; KYBER_CIPHERTEXTBYTES];

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
        
        PeerId(*hasher.finalize().as_bytes())
    }
}
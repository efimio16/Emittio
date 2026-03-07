use rand::{SeedableRng, rngs::OsRng};
use rand_chacha::{ChaCha20Rng};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use thiserror::Error;
use x25519_dalek::x25519;

use crate::utils::{self, KyberCiphertext, KyberPublicKey, KyberSecretKey, random_bytes};

#[derive(Debug, Error)]
pub enum BundleError {
    #[error(transparent)]
    Kyber(#[from] pqc_kyber::KyberError),

    #[error(transparent)]
    Ed25519(#[from] ed25519_dalek::ed25519::Error),

    #[error("invalid shared key")]
    InvalidSharedKey,
}

#[derive(Clone)]
pub struct PrivateBundle {
    seed: [u8; 32],
    x_sk: [u8; 32],
    x_pk: [u8; 32],
    kb_sk: KyberSecretKey,
    kb_pk: KyberPublicKey,
}

impl PrivateBundle {
    pub fn new(seed: [u8; 32], x_sk: [u8; 32], x_pk: [u8; 32], kb_sk: KyberSecretKey, kb_pk: KyberPublicKey) -> Self {
        Self { seed, x_sk, x_pk, kb_sk, kb_pk }
    }
    pub fn random() -> Self {
        Self::from_seed(random_bytes())
    }
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let x_sk = x25519_dalek::StaticSecret::random_from_rng(&mut ChaCha20Rng::from_seed(seed));
        let x_pk = x25519_dalek::PublicKey::from(&x_sk);
        let kb = pqc_kyber::keypair(&mut ChaCha20Rng::from_seed(seed)).expect("failed to generate kyber");
        
        Self::new(seed, x_sk.to_bytes(), x_pk.to_bytes(), kb.secret, kb.public)
    }
    pub fn derive(&self, context: &[u8]) -> Self {
        Self::from_seed(utils::derive(&self.seed, context))
    }
    pub fn public(&self) -> PublicBundle {
        PublicBundle::new(self.x_pk, self.kb_pk)
    }
    pub fn shared(&self, other: &PublicBundle) -> Result<(KyberCiphertext, [u8; 32]), BundleError> {
        let x_shared = x25519(self.x_sk, other.x_pk);
        if x_shared == [0u8; 32] { return Err(BundleError::InvalidSharedKey); }

        let (ct, kb_shared) = pqc_kyber::encapsulate(&other.kb_pk, &mut OsRng)?;
        
        let mut shared = [0u8; 64];
        shared[..32].copy_from_slice(&x_shared);
        shared[32..].copy_from_slice(&kb_shared);

        Ok((ct, utils::derive(&shared, b"shared")))
    }
    pub fn shared_from_ct(&self, other: &PublicBundle, ct: &[u8; pqc_kyber::KYBER_CIPHERTEXTBYTES]) -> Result<[u8; 32], BundleError> {
        let x_shared = x25519(self.x_sk, other.x_pk);
        if x_shared == [0u8; 32] { return Err(BundleError::InvalidSharedKey); };

        let mut shared = [0u8; 64];
        shared[..32].copy_from_slice(&x_shared);
        shared[32..].copy_from_slice(&pqc_kyber::decapsulate(ct, &self.kb_sk)?);

        Ok(utils::derive(&shared, b"shared"))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicBundle {
    pub x_pk: [u8; 32],
    #[serde(with = "BigArray")]
    pub kb_pk: pqc_kyber::PublicKey,
}

impl PublicBundle {
    pub fn new(x_pk: [u8; 32], kb_pk: KyberPublicKey) -> Self {
        Self { x_pk, kb_pk }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bundles::PrivateBundle, utils::{random_bytes, serialize}};

    #[test]
    fn test_shared_secret() {
        let alice = PrivateBundle::random();
        let bob = PrivateBundle::random();

        let alice_pub = alice.public();
        let bob_pub = bob.public();

        let (ct, alice_shared) = alice.shared(&bob_pub).expect("Alice shared failed");
        let bob_shared = bob.shared_from_ct(&alice_pub, &ct).expect("Bob shared_from_ct failed");

        assert_eq!(alice_shared, bob_shared, "Shared secrets must match");
    }

    #[test]
    fn test_deterministic_generation() {
        let seed = random_bytes();

        let alice1 = PrivateBundle::from_seed(seed);
        let alice2 = PrivateBundle::from_seed(seed);
        
        assert_eq!(serialize(&alice1.public()).expect("serialize failed"), serialize(&alice2.public()).expect("serialize failed"), "Equivalent bundles must match");
    }
}
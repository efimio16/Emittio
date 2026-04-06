use pqc_kyber::{KYBER_CIPHERTEXTBYTES, PublicKey as KbPublicKey, SecretKey as KbSecretKey};
use rand::{RngCore, SeedableRng, rngs::OsRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use x25519_dalek::{StaticSecret as XSecretKey, PublicKey as XPublicKey};

use crate::{error::CryptoError, id::{Id, IntoId}};

pub type Capsule = [u8; KYBER_CIPHERTEXTBYTES];
pub type SharedSecret = [u8; 32];

pub struct Kem {
    pub sk: SecretKey,
    pub pk: PublicKey,
}

impl Kem {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let x_sk = XSecretKey::random_from_rng(&mut ChaCha20Rng::from_seed(seed));
        let x_pk = XPublicKey::from(&x_sk);
        let kb = pqc_kyber::keypair(&mut ChaCha20Rng::from_seed(seed)).expect("failed to generate kyber");
        
        Self {
            sk: SecretKey::new(x_sk, kb.secret),
            pk: PublicKey::new(x_pk, kb.public),
        }
    }
    pub fn random() -> Self {
        let mut buf = [0u8; 32];
        OsRng.fill_bytes(&mut buf);
        Self::from_seed(buf)
    }
}

#[derive(Clone)]
pub struct SecretKey {
    x: XSecretKey,
    kb: KbSecretKey,
}

impl SecretKey {
    pub fn new(x: XSecretKey, kb: KbSecretKey) -> Self {
        Self { x, kb }
    }
    pub fn shared(&self, other: &PublicKey) -> Result<(Capsule, SharedSecret), CryptoError> {
        let x_shared = self.x.diffie_hellman(&other.x).to_bytes();
        if x_shared == [0u8; 32] { return Err(CryptoError::InvalidSharedKey); }

        let (ct, kb_shared) = pqc_kyber::encapsulate(&other.kb, &mut OsRng)?;
        
        let mut hasher = blake3::Hasher::new_derive_key("shared");
        hasher.update(&x_shared);
        hasher.update(&kb_shared);

        Ok((ct, hasher.finalize().into()))
    }
    pub fn shared_from_capsule(&self, other: &PublicKey, capsule: &Capsule) -> Result<SharedSecret, CryptoError> {
        let x_shared = self.x.diffie_hellman(&other.x).to_bytes();
        if x_shared == [0u8; 32] { return Err(CryptoError::InvalidSharedKey); };

        let mut hasher = blake3::Hasher::new_derive_key("shared");
        hasher.update(&x_shared);
        hasher.update(&pqc_kyber::decapsulate(capsule, &self.kb)?);

        Ok(hasher.finalize().into())
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PublicKey {
    x: XPublicKey,
    #[serde(with = "BigArray")]
    kb: KbPublicKey,
}

impl PublicKey {
    pub fn new(x: XPublicKey, kb: KbPublicKey) -> Self {
        Self { x, kb }
    }
}

impl IntoId for PublicKey {
    fn id(&self) -> Id {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.x.as_bytes());
        hasher.update(&self.kb);
        Id::new(hasher.finalize().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shared_secret() {
        let alice = Kem::random();
        let bob = Kem::random();

        let (capsule, alice_shared) = alice.sk.shared(&bob.pk).expect("Alice shared failed");
        let bob_shared = bob.sk.shared_from_capsule(&alice.pk, &capsule).expect("Bob shared_from_ct failed");

        assert_eq!(alice_shared, bob_shared, "Shared secrets must match");
    }

    #[test]
    fn test_deterministic_generation() {
        let seed = [1u8; 32];

        let Kem { pk: alice1, .. } = Kem::from_seed(seed);
        let Kem { pk: alice2, .. } = Kem::from_seed(seed);
        
        assert_eq!(&alice1.id(), &alice2.id(), "Equivalent bundles must match");
    }
}
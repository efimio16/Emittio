use rand::{SeedableRng, rngs::OsRng};
use rand_chacha::{ChaCha20Rng};
use ed25519_dalek::{Signer, Verifier};
use pqc_kyber;
use pqc_dilithium_edit as pqc_dilithium;

use crate::utils;

#[derive(Clone)]
pub struct PrivateBundle {
    pub x_sk: x25519_dalek::StaticSecret,
    pub x_pk: x25519_dalek::PublicKey,
    pub ed_sk: ed25519_dalek::SigningKey,
    pub ed_pk: ed25519_dalek::VerifyingKey,
    pub kb_sk: pqc_kyber::SecretKey,
    pub kb_pk: pqc_kyber::PublicKey,
    pub dl_sk: [u8; pqc_dilithium::SECRETKEYBYTES],
    pub dl_pk: [u8; pqc_dilithium::PUBLICKEYBYTES],
}

impl PrivateBundle {
    pub fn new(x_sk: &x25519_dalek::StaticSecret, ed_sk: &ed25519_dalek::SigningKey, kb_seed: [u8; 32], dl_seed: [u8; 32]) -> Self {
        let kb = pqc_kyber::keypair(&mut ChaCha20Rng::from_seed(kb_seed)).expect("Failed to generate kyber keypair");
        let dl = pqc_dilithium::Keypair::generate(&mut ChaCha20Rng::from_seed(dl_seed)).expect("Failed to generate dilithium keypair");
        Self {
            x_sk: x_sk.clone(),
            x_pk: x25519_dalek::PublicKey::from(x_sk),
            ed_sk: ed_sk.clone(),
            ed_pk: ed25519_dalek::VerifyingKey::from(ed_sk),
            kb_sk: kb.secret,
            kb_pk: kb.public,
            dl_sk: dl.secret,
            dl_pk: dl.public,
        }
    }
    pub fn public(&self) -> PublicBundle {
        PublicBundle::new(&self.x_pk, &self.ed_pk, &self.kb_pk, &self.dl_pk)
    }
    pub fn shared(&self, other: &PublicBundle) -> Result<([u8; pqc_kyber::KYBER_CIPHERTEXTBYTES], Vec<u8>), &'static str> {
        let x_shared = self.x_sk.diffie_hellman(&other.x_pk).to_bytes();
        if x_shared == [0u8; 32] {
            return Err("Invalid shared key");
        }

        let (ct, kb_shared) = pqc_kyber::encapsulate(&other.kb_pk, &mut OsRng).map_err(|_| "Encapsulation failed")?;
        Ok((ct, [x_shared, kb_shared].concat()))
    }
    pub fn shared_from_ct(&self, other: &PublicBundle, ct: &[u8; pqc_kyber::KYBER_CIPHERTEXTBYTES]) -> Result<Vec<u8>, &'static str> {
        let x_shared = self.x_sk.diffie_hellman(&other.x_pk).to_bytes();
        if x_shared == [0u8; 32] {
            return Err("Invalid shared key");
        }

        let kb_shared = pqc_kyber::decapsulate(ct, &self.kb_sk).map_err(|_| "Decapsulation failed")?;
        Ok([x_shared, kb_shared].concat())
    }
    pub fn sign(&self, data: Vec<u8>) -> (ed25519_dalek::Signature, [u8; pqc_dilithium::SIGNBYTES]) {
        let hash = utils::hash(&data);

        let dl_keypair = pqc_dilithium::Keypair { public: self.dl_pk, secret: self.dl_sk };
        let dl_signature = dl_keypair.sign(&hash, &mut OsRng).expect("Dilithium signing failed");

        (self.ed_sk.sign(&hash), dl_signature)
    }
}

#[derive(Clone)]
pub struct PublicBundle {
    pub x_pk: x25519_dalek::PublicKey,
    pub ed_pk: ed25519_dalek::VerifyingKey,
    pub kb_pk: pqc_kyber::PublicKey,
    pub dl_pk: [u8; pqc_dilithium::PUBLICKEYBYTES],
}

impl PublicBundle {
    pub fn new(x_pk: &x25519_dalek::PublicKey, ed_pk: &ed25519_dalek::VerifyingKey, kb_pk: &pqc_kyber::PublicKey, dl_pk: &[u8; pqc_dilithium::PUBLICKEYBYTES]) -> Self {
        Self { x_pk: *x_pk, ed_pk: *ed_pk, kb_pk: *kb_pk, dl_pk: *dl_pk }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.x_pk.as_bytes());
        buf.extend_from_slice(self.ed_pk.as_bytes());
        buf
    }
    pub fn verify(&self, signature: (ed25519_dalek::Signature, [u8; pqc_dilithium::SIGNBYTES]), data: Vec<u8>) -> Result<(), &'static str> {
        let hash = utils::hash(&data);

        self.ed_pk.verify(&hash, &signature.0).map_err(|_| "Invalid signature").map_err(|_| "Ed25519 verifying failed")?;
        pqc_dilithium::verify(&signature.1, &hash, &self.dl_pk).map_err(|_| "Dilithium verifying failed")?;
        
        Ok(())
    }
}
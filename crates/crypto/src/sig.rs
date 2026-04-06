use ed25519_dalek::{Signer, SigningKey as EdSigningKey, VerifyingKey as EdVerifyingKey, Signature as EdSignature};
use rand::{SeedableRng, rngs::OsRng};
use rand_chacha::ChaCha20Rng;
use pqc_dilithium_edit::{Keypair as DilithiumKeypair};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::error::CryptoError;

type DlSigningKey = [u8; pqc_dilithium_edit::SECRETKEYBYTES];
type DlVerifyingKey = [u8; pqc_dilithium_edit::PUBLICKEYBYTES];
type DlSignature = [u8; pqc_dilithium_edit::SIGNBYTES];

#[derive(Clone, Serialize, Deserialize)]
pub struct Signature(EdSignature, #[serde(with = "BigArray")] DlSignature);

pub struct Sig;

impl Sig {
    pub fn from_seed(seed: [u8; 32]) -> (SecretKey, PublicKey) {
        let ed_sk = EdSigningKey::generate(&mut ChaCha20Rng::from_seed(seed));
        let ed_pk = ed_sk.verifying_key();
        let dl = DilithiumKeypair::generate(&mut ChaCha20Rng::from_seed(seed)).expect("failed to generate dilithium");
        
        (SecretKey::new(ed_sk, dl.secret), PublicKey::new(ed_pk, dl.public))
    }
}


pub struct SecretKey {
    ed: EdSigningKey,
    dl: DlSigningKey,
}

impl SecretKey {
    pub fn new(ed: EdSigningKey, dl: DlSigningKey) -> Self {
        Self { ed, dl }
    }
    pub fn sign(&self, message: &[u8]) -> Result<Signature, CryptoError> {
        let hash: [u8; 32] = blake3::hash(message).into();

        let ed_sig = self.ed.sign(&hash);
        let dl_sig = pqc_dilithium_edit::sign(&hash, &mut OsRng, &self.dl)?;

        Ok(Signature(ed_sig, dl_sig))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicKey {
    ed: EdVerifyingKey,
    #[serde(with = "BigArray")]
    dl: DlVerifyingKey,
}

impl PublicKey {
    pub fn new(ed: EdVerifyingKey, dl: DlVerifyingKey) -> Self {
        Self { ed, dl }
    }
    pub fn verify(&self, message: &[u8], signature: Signature) -> bool {
        let Signature(ed_sig, dl_sig) = signature;

        let ed_valid = self.ed.verify_strict(message, &ed_sig);
        let dl_valid = pqc_dilithium_edit::verify(&dl_sig, &message, &self.dl);
        
        ed_valid.is_ok() && dl_valid.is_ok()
    }
}
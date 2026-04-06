use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};
use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};

use crate::{error::CryptoError, kem::SharedSecret};

const VERSION: u8 = 1;

pub fn nonce(secret: &SharedSecret, seq: u64) -> [u8; 12] {
    let base = blake3::derive_key("nonce", secret);
    let mut nonce = [0u8; 12];

    let mut seq_bytes = [0u8; 12];
    seq_bytes[4..].copy_from_slice(&seq.to_be_bytes());

    for i in 0..12 {
        nonce[i] = base[i] ^ seq_bytes[i];
    }

    nonce
}

pub fn aad(seq: u64) -> [u8; 9] {
    let mut aad = [0u8; 9];
    aad[0] = VERSION;
    aad[1..].copy_from_slice(&seq.to_be_bytes());
    aad
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Ciphertext {
    bytes: Bytes,
    nonce: [u8; 12],
    tag: [u8; 16],
}

impl Ciphertext {
    pub fn encrypt(shared: SharedSecret, plaintext: &[u8], seq: u64) -> Result<Self, CryptoError> {
        // let nonce = nonce_config.generate(&shared);
        let cipher = Aes256Gcm::new(&shared.into());

        let mut buf = BytesMut::from(plaintext);

        let nonce = nonce(&shared, seq);

        let tag = cipher.encrypt_in_place_detached(&nonce.into(), &aad(seq), &mut buf)?.into();

        Ok(Self { bytes: buf.freeze(), nonce, tag })
    }

    pub fn decrypt(self, shared: SharedSecret, seq: u64) -> Result<Bytes, CryptoError> {
        let cipher = Aes256Gcm::new(&shared.into());

        let mut buf = BytesMut::from(self.bytes);

        cipher.decrypt_in_place_detached(&self.nonce.into(), &aad(seq), &mut buf, &self.tag.into())?;

        Ok(buf.freeze())
    }
}
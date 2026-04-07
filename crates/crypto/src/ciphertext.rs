use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};
use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};

use crate::{error::CryptoError, kem::SharedSecret};

pub type Nonce = [u8; 12];
pub type Tag = [u8; 16];

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Ciphertext {
    bytes: Bytes,
    nonce: Nonce,
    tag: Tag,
}

impl Ciphertext {
    pub fn encrypt(shared: &SharedSecret, plaintext: &[u8], nonce: Nonce, aad: &[u8]) -> Result<Self, CryptoError> {
        let cipher = Aes256Gcm::new(shared.into());

        let mut buf = BytesMut::from(plaintext);

        let tag = cipher.encrypt_in_place_detached(&nonce.into(), aad, &mut buf)?.into();

        Ok(Self { bytes: buf.freeze(), nonce, tag })
    }

    pub fn decrypt(self, shared: SharedSecret, aad: &[u8]) -> Result<Bytes, CryptoError> {
        let cipher = Aes256Gcm::new(&shared.into());

        let mut buf = BytesMut::from(self.bytes);

        cipher.decrypt_in_place_detached(&self.nonce.into(), aad, &mut buf, &self.tag.into())?;

        Ok(buf.freeze())
    }
}
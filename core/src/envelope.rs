use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305, aead::Aead};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use serde_big_array::BigArray;

use crate::{bundles::{BundleError, PrivateBundle, PublicBundle}, utils::{KyberCiphertext, random_bytes}};

#[derive(Debug, Error)]
pub enum EnvelopeError {
    #[error(transparent)]
    Bundle(#[from] BundleError),

    #[error("encryption failed")]
    AesGcmEncryption(aes_gcm::Error),

    #[error("decryption failed")]
    AesGcmDecryption(aes_gcm::Error),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Envelope {
    ciphertext: Vec<u8>,
    nonce: [u8; 24],
    bundle: PublicBundle,
    context: [u8; 32],
    #[serde(with = "BigArray")]
    capsule: KyberCiphertext,
}

impl Envelope {
    pub fn encrypt(plaintext: &[u8], sender: &PrivateBundle, recipient: &PublicBundle) -> Result<Self, EnvelopeError> {
        let context = random_bytes();

        let bundle = sender.derive(&context);
        let (capsule, shared) = bundle.shared(&recipient)?;

        let cipher = XChaCha20Poly1305::new(&shared.into());

        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext).map_err(EnvelopeError::AesGcmEncryption)?;

        Ok(Self { ciphertext, nonce: nonce.into(), bundle: bundle.public(), context, capsule })
    }

    pub fn decrypt(&self, recipient: &PrivateBundle) -> Result<Vec<u8>, EnvelopeError> {
        let cipher = XChaCha20Poly1305::new(&recipient.shared_from_ct(&self.bundle, &self.capsule)?.into());
        let plaintext = cipher.decrypt(&self.nonce.into(), self.ciphertext.as_ref()).map_err(EnvelopeError::AesGcmDecryption)?;

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use crate::{bundles::PrivateBundle, envelope::Envelope};

    #[test]
    fn test_envelope() {
        let sender = PrivateBundle::random();
        let recipient = PrivateBundle::random();
        let envelope = Envelope::encrypt(b"Hello world!", &sender, &recipient.public()).unwrap();
        assert_eq!(envelope.decrypt(&recipient).unwrap(), b"Hello world!", "Text must match");
    }
}
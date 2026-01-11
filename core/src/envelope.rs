use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305, aead::Aead};
use rand::rngs::OsRng;
use pqc_dilithium_edit as pqc_dilithium;
use thiserror::Error;

use crate::{bundles::{BundleError, PrivateBundle, PublicBundle}, utils};

#[derive(Debug, Error)]
pub enum EnvelopeError {
    #[error(transparent)]
    Bundle(#[from] BundleError),

    #[error("encryption failed")]
    AesGcmEncryption(aes_gcm::Error),

    #[error("decryption failed")]
    AesGcmDecryption(aes_gcm::Error),
}

#[derive(Clone)]
pub struct Envelope {
    ciphertext: Vec<u8>,
    nonce: chacha20poly1305::XNonce,
    sender: PublicBundle,
    msg_count: u32,
    capsule: [u8; pqc_kyber::KYBER_CIPHERTEXTBYTES],
    signature: (ed25519_dalek::Signature, [u8; pqc_dilithium::SIGNBYTES]),
}

impl Envelope {
    pub fn encrypt(plaintext: &[u8], msg_count: u32, sender: PrivateBundle, recipient: PublicBundle) -> Result<Self, EnvelopeError> {
        let (capsule, shared) = sender.shared(&recipient)?;
        let cipher = XChaCha20Poly1305::new(&utils::derive(&shared, &Self::info(msg_count)).into());

        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext).map_err(EnvelopeError::AesGcmEncryption)?;
        
        let signature = sender.sign(Self::static_bytes(&ciphertext, &nonce, &sender.public(), &msg_count, &capsule));

        Ok(Self { ciphertext, nonce, sender: sender.public(), msg_count, signature, capsule })
    }

    pub fn decrypt(&self, recipient: PrivateBundle) -> Result<Vec<u8>, EnvelopeError> {
        self.sender.verify(self.signature, self.as_bytes())?;

        let cipher = XChaCha20Poly1305::new(&utils::derive(
            &recipient.shared_from_ct(&self.sender, &self.capsule)?,
            &Self::info(self.msg_count),
        ).into());

        let plaintext = cipher.decrypt(&self.nonce, self.ciphertext.as_ref()).map_err(EnvelopeError::AesGcmDecryption)?;

        Ok(plaintext)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        Self::static_bytes(&self.ciphertext, &self.nonce, &self.sender, &self.msg_count, &self.capsule)
    }
    fn static_bytes(ciphertext: &Vec<u8>, nonce: &chacha20poly1305::XNonce, sender: &PublicBundle, msg_count: &u32, capsule: &[u8; pqc_kyber::KYBER_CIPHERTEXTBYTES]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(ciphertext);
        buf.extend_from_slice(nonce);
        buf.extend_from_slice(&sender.to_bytes());
        buf.extend_from_slice(&msg_count.to_be_bytes());
        buf.extend_from_slice(capsule);
        buf
    }
    fn info(count: u32) -> Vec<u8> {
        utils::info(b"message-key-", count)
    }
}

#[cfg(test)]
mod tests {
    use crate::{bundles::PrivateBundle, envelope::Envelope};

    #[test]
    fn test_envelope() {
        let sender = PrivateBundle::random();
        let recipient = PrivateBundle::random();
        let envelope = Envelope::encrypt(b"Hello world!", 1, sender, recipient.public()).unwrap();
        assert_eq!(envelope.decrypt(recipient).unwrap(), b"Hello world!", "Text must match");
    }
}
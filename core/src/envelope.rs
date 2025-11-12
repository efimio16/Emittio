use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305, aead::Aead};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use ed25519_dalek::{Signer, Verifier};

use crate::bundles::{PrivateBundle, PublicBundle};

#[derive(Clone)]
pub struct Envelope {
    pub ciphertext: Vec<u8>,
    pub nonce: chacha20poly1305::XNonce,
    pub sender: PublicBundle,
    pub signature: ed25519_dalek::Signature,
    pub message_count: u32,
}

impl Envelope {
    pub fn encrypt_and_sign(
        plaintext: &[u8],
        message_count: u32,
        sender: PrivateBundle,
        recipient: PublicBundle,
    ) -> Result<Self, &'static str> {
        // Shared
        let shared = sender.x_sk.diffie_hellman(&recipient.x_pk);
        if shared.as_bytes() == &[0u8; 32] {
            return Err("Invalid shared secret");
        }
        
        // Message-key
        let mut key_bytes = [0u8; 32];
        Hkdf::<Sha256>::new(Some(b"envelope-key-v1"), shared.as_bytes())
            .expand(&Self::key_info(message_count), &mut key_bytes)
            .expect("HKDF expansion failed");

        // Encrypt
        let cipher = XChaCha20Poly1305::new(&key_bytes.into());
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher.encrypt(&nonce, plaintext).map_err(|_| "Encryption failure")?;

        // Sign
        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(&nonce);
        signed_data.extend_from_slice(&ciphertext);
        
        let signature = sender.ed_sk.sign(&signed_data);

        Ok(Self {
            ciphertext,
            nonce,
            sender: sender.public(),
            signature,
            message_count,
        })
    }

    pub fn decrypt_and_verify(
        &self,
        recipient: PrivateBundle,
    ) -> Result<Vec<u8>, &'static str> {
        // Verify
        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(&self.nonce);
        signed_data.extend_from_slice(&self.ciphertext);

        self.sender.ed_pk
            .verify(&signed_data, &self.signature)
            .map_err(|_| "Invalid signature")?;

        // Shared
        let shared = recipient.x_sk.diffie_hellman(&self.sender.x_pk);

        // Message-key
        let mut key_bytes = [0u8; 32];
        Hkdf::<Sha256>::new(Some(b"envelope-key-v1"), shared.as_bytes())
            .expand(&Self::key_info(self.message_count), &mut key_bytes)
            .expect("HKDF expansion failed");

        // Decrypt
        let cipher = XChaCha20Poly1305::new(&key_bytes.into());

        let plaintext = cipher.decrypt(&self.nonce, self.ciphertext.as_ref()).map_err(|_| "Decryption failure")?;

        Ok(plaintext)
    }
    fn key_info(count: u32) -> Vec<u8> {
        let mut info = Vec::new();
        info.extend_from_slice(b"message-key-");
        info.extend_from_slice(&count.to_le_bytes());
        info
    }
}
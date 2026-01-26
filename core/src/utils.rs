use hkdf::Hkdf;
use rand::{RngCore, rngs::OsRng, Rng, thread_rng, distributions::Alphanumeric};
use serde::{Deserialize, Serialize};
use sha2::{Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use pqc_kyber::{KYBER_CIPHERTEXTBYTES, KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES};

#[derive(Debug, Error)]
pub enum SerdeError {
    #[error("serialization failed: {0}")]
    Serialize(postcard::Error),

    #[error("deserialization failed: {0}")]
    Deserialize(postcard::Error),
}

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("channel is closed")]
    Closed
}

pub fn derive(seed: &[u8], context: &[u8]) -> [u8; 32] {
    let mut okm = [0u8; 32];
    Hkdf::<Sha256>::new(Some(b"emittio-protocol-v1"), seed)
        .expand(context, &mut okm)
        .expect("HKDF expansion failed");
    okm
}

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn serialize<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, SerdeError> {
    postcard::to_allocvec(value).map_err(SerdeError::Serialize)
}

pub fn deserialize<'a, T: Deserialize<'a>>(s: &'a [u8]) -> Result<T, SerdeError> {
    postcard::from_bytes(s).map_err(SerdeError::Deserialize)
}

pub fn random_bytes<const T: usize>() -> [u8; T] {
    let mut buf = [0u8; T];
    OsRng.fill_bytes(&mut buf);
    buf
}

pub fn mock_peer_addr() -> String {
    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    format!("mock://{id}")
}

pub(super) type KyberSecretKey = [u8; KYBER_SECRETKEYBYTES];
pub(super) type KyberPublicKey = [u8; KYBER_PUBLICKEYBYTES];
pub(super) type KyberCiphertext = [u8; KYBER_CIPHERTEXTBYTES];
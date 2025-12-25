use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::{Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerdeError {
    #[error("serialization failed: {0}")]
    Serialize(postcard::Error),

    #[error("deserialization failed: {0}")]
    Deserialize(postcard::Error),
}

pub fn info(a: &[u8], b: u32) -> Vec<u8> {
    let mut info = Vec::new();
    info.extend_from_slice(a);
    info.extend_from_slice(&b.to_be_bytes());
    info
}

pub fn derive(seed: &[u8], info: &[u8]) -> [u8; 32] {
    let mut okm = [0u8; 32];
    Hkdf::<Sha256>::new(Some(b"emittio-protocol-v1"), seed)
        .expand(info, &mut okm)
        .expect("HKDF expansion failed");
    okm
}

pub fn hash(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn serialize<T>(value: &T) -> Result<Vec<u8>, SerdeError>
where T: Serialize + ?Sized {
    postcard::to_allocvec(value).map_err(SerdeError::Serialize)
}

pub fn deserialize<'a, T: Deserialize<'a>>(s: &'a [u8]) -> Result<T, SerdeError> {
    postcard::from_bytes(s).map_err(SerdeError::Deserialize)
}

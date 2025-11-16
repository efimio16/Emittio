use hkdf::Hkdf;
use sha2::{Digest, Sha256};

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
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
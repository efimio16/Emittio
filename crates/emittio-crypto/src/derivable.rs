use rand::{RngCore, rngs::OsRng};

pub trait Derivable
where Self: Sized {
    fn derive(seed: [u8; 32]) -> Self;

    fn derive_with_info(seed: [u8; 32], info: &[u8]) -> Self {
        let new_seed = blake3::keyed_hash(&seed, info);
        Self::derive(new_seed.into())
    }

    fn random() -> Self {
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        Self::derive(seed)
    }
}
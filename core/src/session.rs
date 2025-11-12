use rand::{RngCore, rngs::OsRng};
use hkdf::Hkdf;
use sha2::Sha256;

use crate::{bundles::{PrivateBundle, PublicBundle}, inbox::Inbox};

pub struct Session {
    seed: [u8; 32],
    inbox_counter: u32,
}

impl Session {
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        OsRng::fill_bytes(&mut OsRng, &mut seed);
        
        Self { 
            seed,
            inbox_counter: 0,
        }
    }
    pub fn invite(&mut self) -> PublicBundle {
        let inbox_bundle = self.inbox_keys(self.inbox_counter);
        self.inbox_counter += 1;

        inbox_bundle.public()
    }
    pub fn new_inbox(&mut self, with: PublicBundle) -> Inbox {
        let inbox_bundle = self.inbox_keys(self.inbox_counter);
        self.inbox_counter += 1;

        Inbox::new(
            inbox_bundle,
            with
        )
    }
    pub fn inbox(&self, index: u32, with: PublicBundle) -> Inbox {
        Inbox::new(
            self.inbox_keys(index),
            with
        )
    }
    fn inbox_keys(&self, inbox_counter: u32) -> PrivateBundle {
        let hk = Hkdf::<Sha256>::new(None, &self.seed);
        
        let (x_info, ed_info) = self.key_info(inbox_counter);
        let mut new_x_sk = [0u8; 32];
        hk.expand(&x_info, &mut new_x_sk).expect("HKDF expansion failed");

        let mut new_ed_sk = [0u8; 32];
        hk.expand(&ed_info, &mut new_ed_sk).expect("HKDF expansion failed");
        
        PrivateBundle::from_bytes(&new_x_sk, &new_ed_sk)
    }
    fn key_info(&self, count: u32) -> (Vec<u8>, Vec<u8>) {
        let mut x_info = Vec::new();
        x_info.extend_from_slice(b"x25519-key-");
        x_info.extend_from_slice(&count.to_le_bytes());

        let mut ed_info = Vec::new();
        ed_info.extend_from_slice(b"ed25519-key-");
        ed_info.extend_from_slice(&count.to_le_bytes());

        (x_info, ed_info)
    }
}

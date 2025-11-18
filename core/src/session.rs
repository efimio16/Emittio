use rand::{RngCore, rngs::OsRng};

use crate::{bundles::{PrivateBundle}, inbox::Inbox, utils};

pub struct Session {
    seed: [u8; 32],
    // inbox_counter: u32,
}

impl Session {
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        OsRng::fill_bytes(&mut OsRng, &mut seed);
        
        Self { 
            seed,
            // inbox_counter: 0,
        }
    }
    pub fn inbox(&self, index: u32) -> Inbox {
        Inbox::new(self.inbox_keys(index))
    }
    fn inbox_keys(&self, inbox_counter: u32) -> PrivateBundle {
        let x_sk = utils::derive(&self.seed, &utils::info(b"x25519-key-", inbox_counter));
        let ed_sk = utils::derive(&self.seed, &utils::info(b"ed25519-key-", inbox_counter));
        let kb_seed = utils::derive(&self.seed, &utils::info(b"kb-seed-", inbox_counter));
        let dl_seed = utils::derive(&self.seed, &utils::info(b"dl-seed-", inbox_counter));
        
        PrivateBundle::new(&x25519_dalek::StaticSecret::from(x_sk), &ed25519_dalek::SigningKey::from_bytes(&ed_sk), kb_seed, dl_seed)
    }
}
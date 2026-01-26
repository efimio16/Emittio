use rand;

use crate::{bundles::PrivateBundle, inbox::Inbox, utils::random_bytes};

pub struct Session {
    seed: [u8; 32],
}

impl Session {
    pub fn new() -> Self {
        Self { seed: random_bytes() }
    }
    pub fn inbox(&self, index: u32) -> Inbox {
        Inbox::new(self.inbox_keys(index))
    }
    fn inbox_keys(&self, inbox_context: u32) -> PrivateBundle {
        PrivateBundle::from_seed(self.seed).derive(&inbox_context.to_be_bytes())
    }
}
use crate::{bundles::{PrivateBundle, PublicBundle}, envelope::{Envelope, EnvelopeError}};

pub struct Inbox {
    pub sender: PrivateBundle,
}

impl Inbox {
    pub fn new(sender: PrivateBundle) -> Self {
        Self { sender }
    }

    pub fn new_envelope(&mut self, recipient: &PublicBundle, plaintext: &[u8]) -> Result<Envelope, EnvelopeError> {
        Envelope::encrypt(plaintext, &self.sender, recipient)
    }
}
use crate::{bundles::{PrivateBundle, PublicBundle}, envelope::{Envelope, EnvelopeError}};

pub struct Inbox {
    message_counter: u32,
    pub sender: PrivateBundle,
}

impl Inbox {
    pub fn new(sender: PrivateBundle) -> Self {
        Self {
            message_counter: 0,
            sender,
        }
    }

    pub fn new_envelope(&mut self, recipient: PublicBundle, plaintext: &[u8]) -> Result<Envelope, EnvelopeError> {
        Envelope::encrypt(plaintext, self.message_counter, self.sender.clone(), recipient)
    }
}
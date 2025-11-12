use crate::{bundles::{PrivateBundle, PublicBundle}, envelope::Envelope};

pub struct Inbox {
    pub recipient: PublicBundle,
    pub message_counter: u32,
    pub sender: PrivateBundle,
}

impl Inbox {
    pub fn new(sender: PrivateBundle, recipient: PublicBundle) -> Self {
        Self {
            recipient,
            message_counter: 0,
            sender,
        }
    }

    pub fn new_envelope(&mut self, plaintext: &[u8]) -> Result<Envelope, &'static str> {
        Envelope::encrypt_and_sign(plaintext, self.message_counter, self.sender.clone(), self.recipient.clone())
    }
}
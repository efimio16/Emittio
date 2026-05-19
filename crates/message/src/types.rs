use std::collections::HashMap;

use crypto::{id::Id, kem::PublicKey, sig::Signature};

pub enum MailAddress {
	Internal(PublicKey),
	External {
		addr: String,
		sig: Signature, // Signature from the SMTP relay
	},
}

pub struct Message {
    from: MailAddress,
	to: Id,
	subject: String,
	text_root: Vec<Id>,
	attachments: HashMap<String, u64>, // Names and locations of attachments
	attachment_root: Vec<Id>,
}
use emittio_crypto::{id::Id, sig::PublicKey};

pub struct Event {
	id: Id, // Id, or [u8; 32]
	body: EventBody,
	// If you need to replace this event, you need to sign the new event so 
	// nodes could verify that it's your event with one-time cancellation_pk
	cancellation_pk: PublicKey, 
	cancellation_info: [u8; 32], // used that client could reconstruct cancellation_sk from event_seed in the future
}

pub enum EventBody {
	RecvMessage {
		cid: Id,
		msg_pk: PublicKey,
	},
	SendMessage {
		cid: Id,
		info: [u8; 32], // material to recover msg_sk from a seed
	},
}
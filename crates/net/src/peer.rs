use crypto::{id::Id, kem::PublicKey};

pub type PeerId = Id;

#[derive(Clone)]
pub struct Peer {
    pub id: PeerId,
    pub pk: PublicKey,
    pub address: String,
}
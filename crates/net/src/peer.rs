use crypto::{id::Id, kem::PublicKey};

#[derive(Clone)]
pub struct Peer {
    pub id: Id,
    pub identity: PublicKey,
    pub address: String,
}
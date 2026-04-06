use rand::rngs::OsRng;
use crypto::{error::CryptoError, kem::{PublicKey, Kem}};
use crate::{error::NetError, packet::{Handshake, HandshakeAck}, session::PendingSession};

#[derive(Clone)]
pub enum NetClient {
    Ephemeral,
    Static(Kem),
}

impl NetClient {
    pub fn static_client(seed: [u8; 32]) -> Self {
        Self::Static(Kem::from_seed(seed))
    }
    pub fn handshake(&self, other: &PublicKey) -> Result<(PendingSession, Handshake), CryptoError> {
        let kem = match self {
            NetClient::Ephemeral => &Kem::random(),
            NetClient::Static(kem) => kem,
        };

        let (capsule, shared) = kem.sk.shared(other)?;

        Ok((
            PendingSession::new(shared, None),
            Handshake { from: kem.pk, capsule, created_conn_id: OsRng.next_u64() },
        ))
    }

    pub fn pk(&self) -> Option<&PublicKey> {
        match self {
            NetClient::Static(kem) => Some(&kem.pk),
            NetClient::Ephemeral => None,
        }
    }

    pub fn accept(&self, handshake: Handshake) -> Result<(PendingSession, HandshakeAck), NetError> {
        let NetClient::Static(kem) = self else {
            return Err(NetError::EphemeralClient);
        };

        Ok((
            PendingSession::new(kem.sk.shared_from_capsule(&handshake.from, &handshake.capsule)?, Some(handshake.created_conn_id)),
            HandshakeAck { conn_id: handshake.created_conn_id, created_conn_id: OsRng.next_u64() },
        ))
    }
}

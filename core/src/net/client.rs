use rand::{RngCore, SeedableRng, rngs::OsRng};
use rand_chacha::ChaCha20Rng;
use x25519_dalek::x25519;
use crate::{net::{CryptoError, Handshake, HandshakeAck, NetIdentity, PendingSession}, utils::{self, KyberPublicKey, KyberSecretKey}};

#[derive(Clone)]
pub enum NetClient {
    Ephemeral,
    Static {
        x_sk: [u8; 32],
        x_pk: [u8; 32],
        kb_sk: KyberSecretKey,
        kb_pk: KyberPublicKey,
    },
}

impl NetClient {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let x_sk = x25519_dalek::StaticSecret::random_from_rng(&mut ChaCha20Rng::from_seed(seed));
        let x_pk = x25519_dalek::PublicKey::from(&x_sk);
        let kb = pqc_kyber::keypair(&mut ChaCha20Rng::from_seed(seed)).expect("failed to generate kyber");
        
        Self::Static {
            x_sk: x_sk.to_bytes(),
            x_pk: x_pk.to_bytes(),
            kb_sk: kb.secret,
            kb_pk: kb.public,
        }
    }
    pub fn handshake(&self, to: NetIdentity) -> Result<(PendingSession, Handshake), CryptoError> {
        let (
            x_sk,
            x_pk,
            _,
            kb_pk,
        ) = match self {
            NetClient::Ephemeral => {
                let x_sk = x25519_dalek::StaticSecret::random_from_rng(&mut OsRng);
                let x_pk = x25519_dalek::PublicKey::from(&x_sk);
                let kb = pqc_kyber::keypair(&mut OsRng).expect("failed to generate kyber");

                (x_sk.to_bytes(), x_pk.to_bytes(), kb.secret, kb.public)
            },
            NetClient::Static { x_sk, x_pk, kb_pk, .. } => (*x_sk, *x_pk, [0u8; 1632], *kb_pk),
        };

        let x_shared = x25519(x_sk, to.x_pk);
        if x_shared == [0u8; 32] { return Err(CryptoError::InvalidSharedKey); };

        let (ct, kb_shared) = pqc_kyber::encapsulate(&to.kb_pk, &mut OsRng)?;

        let mut shared = [0u8; 64];
        shared[..32].copy_from_slice(&x_shared);
        shared[32..].copy_from_slice(&kb_shared);

        Ok((
            PendingSession::new(utils::derive(&shared, b"shared"), None),
            Handshake { from: NetIdentity { x_pk, kb_pk }, ct, created_conn_id: OsRng.next_u64() },
        ))
    }

    pub fn identity(&self) -> Option<NetIdentity> {
        match self {
            NetClient::Static { x_pk, kb_pk, .. } => Some(NetIdentity { x_pk: *x_pk, kb_pk: *kb_pk }),
            NetClient::Ephemeral => None
        }
    }

    pub fn accept(&self, handshake: Handshake) -> Result<(PendingSession, HandshakeAck), CryptoError> {
        let NetClient::Static { x_sk, kb_sk, .. } = self else {
            return Err(CryptoError::EphemeralClient);
        };

        let x_shared = x25519(*x_sk, handshake.from.x_pk);
        if x_shared == [0u8; 32] { return Err(CryptoError::InvalidSharedKey); };

        let mut shared = [0u8; 64];
        shared[..32].copy_from_slice(&x_shared);
        shared[32..].copy_from_slice(&pqc_kyber::decapsulate(&handshake.ct, kb_sk)?);

        Ok((
            PendingSession::new(utils::derive(&shared, b"shared"), Some(handshake.created_conn_id)),
            HandshakeAck { conn_id: handshake.created_conn_id, created_conn_id: OsRng.next_u64() },
        ))
    }
}

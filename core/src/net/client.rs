use rand::{RngCore, SeedableRng, rngs::OsRng};
use rand_chacha::ChaCha20Rng;
use x25519_dalek::x25519;
use crate::{net::{error::CryptoError, packet::{Handshake, HandshakeAck}, session::NetSession, types::{KyberPublicKey, KyberSecretKey, NetIdentity}}, utils};

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
    pub fn from_seed(seed: [u8; 32]) -> (Self, NetIdentity) {
        let x_sk = x25519_dalek::StaticSecret::random_from_rng(&mut ChaCha20Rng::from_seed(seed));
        let x_pk = x25519_dalek::PublicKey::from(&x_sk);
        let kb = pqc_kyber::keypair(&mut ChaCha20Rng::from_seed(seed)).expect("failed to generate kyber");
        
        (Self::Static {
            x_sk: x_sk.to_bytes(),
            x_pk: x_pk.to_bytes(),
            kb_sk: kb.secret,
            kb_pk: kb.public,
        }, NetIdentity {
            x_pk: x_pk.to_bytes(),
            kb_pk: kb.public,
        })
    }
    pub fn handshake(&self, to: NetIdentity) -> Result<([u8; 32], Handshake), CryptoError> {
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
            utils::derive(&shared, b"shared"),
            Handshake { from: NetIdentity { x_pk, kb_pk }, ct, created_conn_id: OsRng.next_u64() },
        ))
    }

    pub fn accept(&self, handshake: Handshake) -> Result<(NetSession, HandshakeAck), CryptoError> {
        let NetClient::Static { x_sk, kb_sk, .. } = self else {
            return Err(CryptoError::CannotAcceptConn);
        };

        let x_shared = x25519(*x_sk, handshake.from.x_pk);
        if x_shared == [0u8; 32] { return Err(CryptoError::InvalidSharedKey); };

        let mut shared = [0u8; 64];
        shared[..32].copy_from_slice(&x_shared);
        shared[32..].copy_from_slice(&pqc_kyber::decapsulate(&handshake.ct, kb_sk)?);

        let conn_id = OsRng.next_u64();

        Ok((
            NetSession::new(utils::derive(&shared, b"shared"), handshake.created_conn_id),
            HandshakeAck { conn_id: handshake.created_conn_id, created_conn_id: conn_id },
        ))
    }

    pub fn session(&self, shared: [u8; 32], ack: HandshakeAck) -> NetSession {
        NetSession::new(shared, ack.created_conn_id)
    }
}

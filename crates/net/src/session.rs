use bytes::Bytes;
use crypto::{blake3, ciphertext::{Ciphertext, Nonce}, id::Id, kem::{SecretKey, SharedSecret}};

use crate::{error::NetError, packet::WireMessage};

const WINDOW: usize = 32;
const VERSION: u8 = 1;

pub type SessionId = Id;

pub struct PendingSession(SharedSecret);

impl PendingSession {
    #[inline]
    pub fn new(shared: SharedSecret) -> Self {
        Self(shared)
    }
    pub fn activate(self) -> ActiveSession {
        ActiveSession::new(self.0, false)
    }
}

pub type EphemeralState = SecretKey;

pub struct ActiveSession {
    shared: SharedSecret,
    session_id: SessionId,
    last_seen: u64,
    bitmap: u32,
    seq: u64,
    initiator: bool,
}

impl ActiveSession {
    #[inline]
    pub fn new(shared: SharedSecret, initiator: bool) -> Self {
        let session_id = SessionId::from(&shared as &[u8]);
        Self { shared, session_id, last_seen: 0, bitmap: 0, seq: 0, initiator }
    }
    #[inline]
    fn check_seq(&mut self, seq: u64) -> bool {
        if seq > self.last_seen {
            let shift = (seq - self.last_seen) as usize;

            if shift >= WINDOW {
                self.bitmap = 0;
            } else {
                self.bitmap <<= shift;
            }

            self.bitmap |= 1;
            self.last_seen = seq;
            return true;
        }

        let offset = (self.last_seen - seq) as usize;

        if offset >= WINDOW {
            return false; // too old
        }

        let mask = 1 << offset;

        if self.bitmap & mask != 0 {
            return false; // replay
        }

        self.bitmap |= mask;
        true
    }

    pub fn send(&mut self, plaintext: &[u8]) -> Result<WireMessage, NetError> { // WireMessage
        self.seq += 1;

        let ciphertext = Ciphertext::encrypt(&self.shared, plaintext, self.nonce(), &self.aad())?;

        Ok(WireMessage { seq: self.seq, ciphertext, session_id: self.session_id.clone() })
    }

    pub fn recv(&mut self, msg: WireMessage) -> Result<Bytes, NetError> {
        if !self.check_seq(msg.seq) {
            return Err(NetError::InvalidSeq);
        }

        let plaintext = msg.ciphertext.decrypt(self.shared, &self.aad())?;

        Ok(plaintext)
    }

    fn nonce(&self) -> Nonce {
        let base = blake3::derive_key("nonce", &self.shared);
        let mut nonce = [0u8; 12];

        let mut seq_bytes = [0u8; 12];

        if self.initiator {
            seq_bytes[..4].copy_from_slice(b"init");
        } else {
            seq_bytes[..4].copy_from_slice(b"resp");
        }

        seq_bytes[4..].copy_from_slice(&self.seq.to_be_bytes());

        for i in 0..12 {
            nonce[i] = base[i] ^ seq_bytes[i];
        }

        nonce
    }

    fn aad(&self) -> [u8; 41] {
        let mut aad = [0u8; 41];
        aad[0] = VERSION;
        aad[1..33].copy_from_slice(&self.session_id.as_bytes());
        aad[33..41].copy_from_slice(&self.seq.to_be_bytes());
        aad
    }
}
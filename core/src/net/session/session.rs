use aes_gcm::{Aes256Gcm, AesGcm, KeyInit, aead::{Aead, Payload}, aes::Aes256};
use chacha20poly1305::consts::U12;

use crate::{VERSION, net::{CryptoError, NetError, ConnId, Message}};

const WINDOW: usize = 32;
const NONCE_PREFIX: [u8; 4] = [0x12,0x34,0x56,0x78];

pub struct PendingSession {
    shared: [u8; 32],
    conn_id: Option<ConnId>,
}

impl PendingSession {
    pub fn new(shared: [u8; 32], conn_id: Option<ConnId>) -> Self {
        Self { shared, conn_id }
    }
    pub fn activate(self, conn_id: Option<ConnId>) -> Option<ActiveSession> {
        self.conn_id.or(conn_id).map(
            |conn_id| ActiveSession { shared: self.shared, conn_id, last_seen: 0, bitmap: 0, seq: 0 }
        )
    }
}

pub struct ActiveSession {
    shared: [u8; 32],
    conn_id: ConnId,
    last_seen: u64,
    bitmap: u32,
    seq: u64,
}

impl ActiveSession {
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

    fn cipher(&self) -> AesGcm<Aes256, U12> {
        Aes256Gcm::new(&self.shared.into())
    }
    fn nonce(seq: u64) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        nonce[..4].copy_from_slice(&NONCE_PREFIX);
        nonce[4..].copy_from_slice(&seq.to_be_bytes());
        nonce
    }
    fn aad(seq: u64) -> [u8; 9] {
        let mut aad = [0u8; 9];
        aad[..1].copy_from_slice(&VERSION.to_be_bytes());
        aad[1..].copy_from_slice(&seq.to_be_bytes());
        aad
    }

    pub fn send(&mut self, plaintext: &Vec<u8>) -> Result<Message, CryptoError> {
        self.seq += 1;

        let cipher = self.cipher();
        
        let nonce = Self::nonce(self.seq);
        let aad = Self::aad(self.seq);

        let body = cipher.encrypt(&nonce.into(), Payload { msg: plaintext, aad: &aad }).map_err(CryptoError::AesGcmEncryption)?;

        Ok(Message { seq: self.seq, body, conn_id: self.conn_id })
    }

    pub fn receive(&mut self, msg: Message) -> Result<Vec<u8>, NetError> {
        if !self.check_seq(msg.seq) {
            return Err(NetError::InvalidSeq);
        }

        let cipher = self.cipher();

        Ok(
            cipher.decrypt(&Self::nonce(msg.seq).into(), Payload { msg: &msg.body, aad: &Self::aad(msg.seq)}).map_err(CryptoError::AesGcmDecryption)?
        )
    }
}
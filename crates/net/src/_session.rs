use bytes::Bytes;
use crypto::ciphertext::Ciphertext;

use crate::{error::NetError, packet::{ConnId, Message}};

const WINDOW: usize = 32;

pub struct NetSession {
    shared: [u8; 32],
    conn_id: ConnId,
    last_seen: u64,
    bitmap: u32,
    seq: u64,
}

impl NetSession {
    pub fn new(shared: [u8; 32], conn_id: ConnId) -> Self {
        Self { shared, conn_id, last_seen: 0, bitmap: 0, seq: 0 }
    }
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

    pub fn send(&mut self, plaintext: &[u8]) -> Result<Message, NetError> {
        self.seq += 1;

        let ciphertext = Ciphertext::encrypt(self.shared, plaintext, self.seq)?;

        Ok(Message { seq: self.seq, ciphertext, conn_id: self.conn_id })
    }

    pub fn receive(&mut self, msg: Message) -> Result<Bytes, NetError> {
        if !self.check_seq(msg.seq) {
            return Err(NetError::InvalidSeq);
        }

        let plaintext = msg.ciphertext.decrypt(self.shared, msg.seq)?;

        Ok(plaintext)
    }
}
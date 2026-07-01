use emittio_crypto::{blake3, ciphertext::{Nonce, Sealed}, kem::SharedSecret};

use crate::{error::NetworkError, types::{Frame, FrameData}};

const WINDOW: usize = 32;
const VERSION: u8 = 1;

pub struct Session {
    shared: SharedSecret,
    // session_id: SessionId,
    last_seen: u64,
    bitmap: u32,
    seq: u64,
    initiator: bool,
}

impl Session {
    #[inline]
    pub fn new(shared: SharedSecret, initiator: bool) -> Self {
        Self { shared, last_seen: 0, bitmap: 0, seq: 0, initiator }
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

    pub fn send(&mut self, data: &FrameData) -> Result<Frame, NetworkError> {
        self.seq += 1;

        let sealed = Sealed::encrypt(&self.shared, data, self.nonce(), &self.aad())?;

        Ok(Frame { seq: self.seq, data: sealed })
    }

    pub fn recv(&mut self, frame: Frame) -> Result<FrameData, NetworkError> {
        if !self.check_seq(frame.seq) {
            return Err(NetworkError::InvalidSeq);
        }

        let data = frame.data.decrypt(self.shared, &self.aad())?;

        Ok(data)
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
        aad[1..9].copy_from_slice(&self.seq.to_be_bytes());
        aad
    }
}
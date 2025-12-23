use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use blake3;

use crate::{message::Action, utils::get_timestamp};

fn pow_input(secret: &[u8; 32], timestamp: u64, action: Action, random: &[u8; 16]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(secret);
    hasher.update(&timestamp.to_be_bytes());
    hasher.update(&action.value().to_be_bytes());
    hasher.update(random);
    hasher.finalize().into()
}

#[derive(Serialize, Deserialize)]
pub struct Pow {
    pub input: [u8; 32],
    pub timestamp: u64,
    pub action: Action,
    pub random: [u8; 16],
    pub target: [u8; 32],
}

impl Pow {
    pub fn new(secret: &[u8; 32], action: Action, difficulty: u8) -> Self {
        let mut target = [0xff; 32];

        let full_bytes = (difficulty / 8) as usize;
        let remaining_bits = difficulty % 8;

        for i in 0..full_bytes {
            target[i] = 0x00;
        }

        if remaining_bits > 0 && full_bytes < 32 {
            target[full_bytes] = 0xff >> remaining_bits;
        }

        let timestamp = get_timestamp();

        let mut random =  [0u8; 16];
        OsRng.fill_bytes(&mut random);

        Pow {
            input: pow_input(secret, timestamp, action, &random),
            timestamp,
            action,
            random,
            target,
        }
    }

    pub fn verify(&self, nonce: u64) -> bool {
        self.hash(nonce) < self.target
    }

    pub fn verify_with_secret(&self, secret: &[u8; 32], nonce: u64) -> bool {
        let now = get_timestamp();
        self.verify(nonce) &&
        now >= self.timestamp &&
        (now - self.timestamp < 300) &&
        pow_input(secret, self.timestamp, self.action, &self.random) == self.input
    }

    pub fn solve(&self) -> u64 {
        let mut nonce = 0u64;

        loop {
            if self.verify(nonce) {
                return nonce;
            }
            nonce += 1;
        }
    }

    fn hash(&self, nonce: u64) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(self.input);
        hasher.update(nonce.to_be_bytes());

        let result = hasher.finalize();

        result.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{message::Action, pow::Pow, utils::{deserialize, serialize}};

    use rand::{RngCore, rngs::OsRng};
    use std::time::Instant;

    #[test]
    fn pow_test() {
        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);

        let action = Action::PublishTag;
        let difficulty = 16; // 17-450ms on Mac M3

        let pow = Pow::new(&secret, action, difficulty);

        let start = Instant::now();

        let nonce = pow.solve();

        let duration = start.elapsed();

        assert!(pow.verify_with_secret(&secret, nonce));
        println!("Solved in {:#?}", duration);
    }

    #[test]
    fn serde_test() {
        let pow = Pow::new(&[1u8; 32], Action::PublishTag, 16);

        let encoded = serialize(&pow).expect("serialization failed");
        let decoded: Pow = deserialize(&encoded).expect("deserialization failed");

        assert_eq!(pow.input, decoded.input);
    }
}
use std::{fmt, ops::{BitAnd, BitAndAssign}};
use faster_hex::hex_encode;
use serde::{Deserialize, Serialize};

use crate::error::CryptoError;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Hash, Copy, Default)]
pub struct Id(pub [u8; 32]);

impl Id {
    #[inline]
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    #[inline]
    pub fn hash_from<T: Serialize>(object: &T) -> Result<Self, CryptoError> {
        Ok(Self(blake3::hash(&postcard::to_stdvec(object)?).into()))
    }
    #[inline]
    pub fn hash_bytes(bytes: &[u8]) -> Self {
        Self(blake3::hash(&bytes).into())
    }
    #[inline]
    pub fn bucket(&self, mask: &Mask) -> Self {
        Self(self.0 & mask)
    }
}

impl BitAnd for Id {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(bitand_bytes(&self.0, &rhs.0))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hex = [0u8; 64];
        let s = hex_encode(&self.0, &mut hex).expect("failed to encode id to hex");
        f.write_str(s)
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Mask(pub [u8; 32]);

impl Mask {
    pub fn new_hex_mask(max: u64, mut current: u64) -> Self {
        let mut mask = [0u8; 32];

        let mut i = 0;
        while current > max {
            mask[i] = 0xf;
            i += 1;
            current /= 0x10;
        }

        Self(mask)
    }
}

impl BitAnd<&Mask> for [u8; 32] {
    type Output = [u8; 32];
    fn bitand(self, rhs: &Mask) -> Self::Output {
        bitand_bytes(&self, &rhs.0)
    }
}

impl BitAndAssign<&Mask> for [u8; 32] {
    fn bitand_assign(&mut self, rhs: &Mask) {
        *self = *self & rhs;
    }
}

pub fn bitand_bytes(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];

    let mut i = 0;
    for byte in a {
        out[i] = byte & b[i];
        i += 1;
    }

    out
}
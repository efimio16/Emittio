use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct CID([u8; 32]);

impl CID {
    pub fn new(bytes: &[u8]) -> Self {
        Self(blake3::hash(bytes).into())
    }
}

impl AsRef<[u8; 32]> for CID {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<&str> for CID {
    fn from(value: &str) -> Self {
        Self(*blake3::hash(value.as_bytes()).as_bytes())
    }
}

impl From<[u8; 32]> for CID {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}
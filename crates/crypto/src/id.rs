use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct Id([u8; 32]);

impl Id {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

pub trait IntoId {
    fn id(&self) -> Id;
}

impl IntoId for Id {
    #[inline]
    fn id(&self) -> Id {
        Self(self.0)
    }
}

impl From<&[u8]> for Id {
    fn from(value: &[u8]) -> Self {
        Self(blake3::hash(value).into())
    }
}
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct Id([u8; 32]);

impl Id {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
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
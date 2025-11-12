#[derive(Clone)]
pub struct PrivateBundle {
    pub x_sk: x25519_dalek::StaticSecret,
    pub x_pk: x25519_dalek::PublicKey,
    pub ed_sk: ed25519_dalek::SigningKey,
    pub ed_pk: ed25519_dalek::VerifyingKey,
}

impl PrivateBundle {
    pub fn new(x_sk: &x25519_dalek::StaticSecret, ed_sk: &ed25519_dalek::SigningKey) -> Self {
        Self {
            x_sk: x_sk.clone(),
            x_pk: x25519_dalek::PublicKey::from(x_sk),
            ed_sk: ed_sk.clone(),
            ed_pk: ed25519_dalek::VerifyingKey::from(ed_sk),
        }
    }
    pub fn from_bytes(x_sk_bytes: &[u8; 32], ed_sk_bytes: &[u8; 32]) -> Self {
        Self::new(&x25519_dalek::StaticSecret::from(*x_sk_bytes), &ed25519_dalek::SigningKey::from_bytes(ed_sk_bytes))
    }
    pub fn public(&self) -> PublicBundle {
        PublicBundle::new(&self.x_pk, &self.ed_pk)
    }
}

#[derive(Clone)]
pub struct PublicBundle {
    pub x_pk: x25519_dalek::PublicKey,
    pub ed_pk: ed25519_dalek::VerifyingKey,
}

impl PublicBundle {
    pub fn new(x_pk: &x25519_dalek::PublicKey, ed_pk: &ed25519_dalek::VerifyingKey) -> Self {
        Self { x_pk: *x_pk, ed_pk: *ed_pk }
    }
}
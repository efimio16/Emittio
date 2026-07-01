use curve25519_dalek::{MontgomeryPoint, Scalar};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::derivable::Derivable;

#[derive(Clone, Deserialize, Serialize)]
pub struct Tag {
    pk: MontgomeryPoint,
    shared: MontgomeryPoint,
}

pub struct TagVerifier(Scalar);

#[derive(Clone, Deserialize, Serialize)]
pub struct TagAddress(MontgomeryPoint);

impl Derivable for TagVerifier {
    fn derive(seed: [u8; 32]) -> Self {
        Self(Scalar::from_bytes_mod_order(seed))
    }
}

impl TagVerifier {
    #[inline]
    pub fn verify(&self, tag: Tag) -> bool {
        self.0 * tag.pk == tag.shared
    }
    pub fn address(&self) -> TagAddress {
        TagAddress(MontgomeryPoint::mul_base(&self.0))
    }
}

impl TagAddress {
    pub fn generate_tag(&self) -> Tag {
        let sk = Scalar::random(&mut OsRng);
        let pk = MontgomeryPoint::mul_base(&sk);
        let shared = sk * self.0;
        Tag { pk, shared }
    }
}
use std::marker::PhantomData;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{ciphertext::{Ciphertext, Nonce}, error::SealedError, kem::SharedSecret};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Sealed<T> {
    ciphertext: Ciphertext,
    _marker: PhantomData<T>,
}

impl<T: DeserializeOwned + Serialize> Sealed<T> {
    #[inline]
    pub fn encrypt(shared: &SharedSecret, data: &T, nonce: Nonce, aad: &[u8]) -> Result<Self, SealedError> {
        Ok(Self {
            ciphertext: Ciphertext::encrypt(shared, &postcard::to_stdvec(data)?, nonce, aad)?,
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn decrypt(self, shared: SharedSecret, aad: &[u8]) -> Result<T, SealedError> {
        let bytes = self.ciphertext.decrypt(shared, aad)?;
        Ok(postcard::from_bytes(&bytes)?)
    }
}
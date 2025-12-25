use chacha20poly1305::consts::U12;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use aes_gcm::{
    AeadCore, Aes256Gcm, AesGcm, aead::{self, Aead, KeyInit}, aes::Aes256
};
use bytes::{BufMut, Bytes, BytesMut};
use thiserror::Error;

use crate::{VERSION, utils::{self, SerdeError, deserialize, serialize}};

#[derive(Debug, Error)]
pub enum TagError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error("encryption failed")]
    AesGcmEncryption(aes_gcm::Error),

    #[error("decryption failed")]
    AesGcmDecryption(aes_gcm::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    nonce: [u8; 12],
    content: Vec<u8>,

    pub hash: [u8; 32],
    pub info: [u8; 16],
    pub created_at: u64,    
}

impl Tag {
    pub fn new(seed: &[u8; 32], plaintext: TagPayload) -> Result<Self, TagError> {
        let info = rand::random();
        let hash = Self::hash(seed, &info);

        let created_at = utils::get_timestamp();

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let content = Self::cipher(seed).encrypt(
            &nonce,
            aead::Payload {
                msg: &serialize(&plaintext)?,
                aad: &Self::aad(created_at, &info),
            },
        ).map_err(TagError::AesGcmEncryption)?;

        Ok(Self { hash, nonce: nonce.into(), info, content, created_at })
    }
    pub fn hash(seed: &[u8; 32], info: &[u8; 16]) -> [u8; 32] {
        *blake3::keyed_hash(&utils::derive(seed, b"tag-hash"), info).as_bytes()
    }
    pub fn cipher(seed: &[u8; 32]) -> AesGcm<Aes256, U12> {
        Aes256Gcm::new(&utils::derive(seed, b"tag-key").into())
    }
    pub fn aad(created_at: u64, info: &[u8; 16]) -> Bytes {
        let mut aad = BytesMut::with_capacity(25);
        aad.put_u8(VERSION as u8);
        aad.put_u64(created_at);
        aad.extend_from_slice(info);
        aad.freeze()
    }
    pub fn is_owner(&self, seed: &[u8; 32]) -> bool {
        Self::hash(seed, &self.info) == self.hash
    }
    pub fn decrypt(&self, seed: &[u8; 32]) -> Result<TagPayload, TagError> {
        let payload = deserialize(
            &Self::cipher(seed).decrypt(
                &self.nonce.into(),
                aead::Payload {
                    msg: &self.content,
                    aad: &Self::aad(self.created_at, &self.info),
                },
            ).map_err(TagError::AesGcmDecryption)?
        )?;
        Ok(payload)
    }
    pub fn to_owned_tag(&self, seed: &[u8; 32]) -> Result<OwnedTag, TagError> {
        Ok(OwnedTag { tag: self.clone(), payload: self.decrypt(seed)? })
    }
}

#[derive(Debug, Clone)]
pub struct OwnedTag {
    pub tag: Tag,
    pub payload: TagPayload,
}

impl OwnedTag {
    pub fn new(seed: &[u8; 32], payload: TagPayload) -> Result<Self, TagError> {
        Ok(Self { tag: Tag::new(seed, payload.clone())?, payload })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TagPayload {
    pub data: Vec<u8>,
}

pub struct TagManager {
    seed: [u8; 32],
    pub tags: Vec<OwnedTag>,
}

impl TagManager {
    pub fn new() -> Self {
        Self { seed: rand::random(), tags: Vec::new() }
    }
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self { seed: *seed, tags: Vec::new() }
    }
    pub fn load_tags(&mut self, all_tags: Vec<Tag>) -> Result<(), TagError> {
        for tag in all_tags {
            if tag.is_owner(&self.seed) {
                self.tags.push(tag.to_owned_tag(&self.seed)?);
            }
        }
        Ok(())
    }
    pub fn new_tag(&mut self, payload: TagPayload) -> Result<OwnedTag, TagError> {
        let tag = OwnedTag::new(&self.seed, payload)?;
        self.tags.push(tag.clone());
        Ok(tag)
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use crate::tag::{OwnedTag, TagManager, TagPayload};

    #[test]
    fn test_tag() {
        let seed = random();

        let owned_tag = OwnedTag::new(&seed, TagPayload { data: b"Hello!".into() }).expect("failed create tag");

        let mut manager = TagManager::from_seed(&seed);
        manager.load_tags(vec![owned_tag.tag]).expect("load failed");

        assert_eq!(manager.tags[0].payload, owned_tag.payload);
    }
}
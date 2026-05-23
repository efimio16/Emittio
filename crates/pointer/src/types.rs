use crypto::{ciphertext::Sealed, kem::PublicKey};

pub struct InternalPointerHeader {
    pk: [u8; 32],
    tag: [u8; 32], // tag = hash(a_lookup_sk * pointer_pk) = hash(pointer_sk * a_lookup_pk)
}

pub struct ExternalPointerHeader {
    info: [u8; 32],
    tag: [u8; 32], // tag = derive(shared_secret, random_pk)
}

pub struct PointerBody {
    // msg_pk is the public key of message metadata
    // and it's different from pk, so you could delegate
    // lookup_sk exposing minimal metadata
    msg_pk: PublicKey, // hybrid, almost 1KB
    cid: Sealed<[u8; 32]>, // encrypted with shared(recv, msg_pk)
}
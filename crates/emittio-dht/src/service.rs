use std::path::PathBuf;

use bytes::{Bytes, BytesMut};
use emittio_crypto::id::Id;
use emittio_network::types::Handler;
use tokio::{fs::File, io::{self, AsyncReadExt, AsyncWriteExt}};

use crate::{error::{DhtGetError, DhtPutError}, query::{DhtGet, DhtPut}};

const MAX_LEN: usize = 256 * 1000;

pub struct DhtStorage {
    dir: PathBuf,
}

impl Handler<DhtGet> for DhtStorage {
    async fn handle(&mut self, query: DhtGet) -> Result<Bytes, DhtGetError> {
        let path = self.dir.join(format!("{}", query.cid));

        let mut bytes = BytesMut::new();
        let file = File::open(path).await.map_err(|| DhtGetError::Internal)?;
        file.read(&mut bytes).await.map_err(|| DhtGetError::Internal)?;

        Ok(bytes.freeze())
    }
}

impl Handler<DhtPut> for DhtStorage {
    async fn handle(&mut self, query: DhtPut) -> Result<(), DhtPutError> {
        if query.bytes.len() > MAX_LEN {
            return Err(DhtPutError::TooLarge);
        }

        let path = self.dir.join(format!("{}", Id::hash_bytes(&query.bytes)));

        let file = File::create(path).await.map_err(|| DhtPutError::Internal)?;

        file.write(&query.bytes).await.map_err(|| DhtPutError::Internal)?;

        Ok(())
    }
}

// TODO: implement service for DHT storage
use std::{collections::HashMap, path::PathBuf, time::Duration};
use bytes::Bytes;
use tokio::{fs::{File, OpenOptions}, io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc, time::interval};
use tokio_util::sync::CancellationToken;

use crate::{dht::{CID, DhtStorageCmd, DhtStorageDispatcher, DhtStorageError}, service::Service, utils::{self, ChannelError}};

const CHAN_SIZE: usize = 100;
const TICK_INTERVAL: Duration = Duration::from_secs(5);

pub struct DhtStorage {
    rx: mpsc::Receiver<DhtStorageCmd>,
    storage: HashMap<CID, Bytes>,
    path: PathBuf,
}

impl DhtStorage {
    pub async fn create(path: PathBuf) -> Result<(Self, DhtStorageDispatcher), DhtStorageError> {
        File::create(path.clone()).await?;

        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        Ok((Self { rx, storage: HashMap::new(), path }, DhtStorageDispatcher { tx }))
    }

    pub async fn load_from_file(path: PathBuf) -> Result<(Self, DhtStorageDispatcher), DhtStorageError> {
        let mut buffer = Vec::new();
        File::open(path.clone()).await?.read_to_end(&mut buffer).await?;

        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        Ok((Self { rx, storage: utils::deserialize(&buffer)?, path }, DhtStorageDispatcher { tx }))
    }
}

impl Service for DhtStorage {
    type Error = DhtStorageError;
    async fn run(mut self, token: CancellationToken) -> Result<(), DhtStorageError> {
        println!("Running DHT storage");

        let mut file = OpenOptions::new()
            .write(true)
            .open(self.path.clone()).await?;

        let mut ticker = interval(TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    file.flush().await?;
                    file.sync_all().await?;
                    
                    return Ok(())
                }
                _ = ticker.tick() => {
                    file.set_len(0).await?;
                    file.write_all(&utils::serialize(&self.storage)?).await?;
                }
                Some(cmd) = self.rx.recv() => {
                    match cmd {
                        DhtStorageCmd::Get { cid, reply } => {
                            let content = self.storage.get(&cid).cloned();
                            reply.send(content).map_err(|_| ChannelError::Closed)?;
                        },
                        DhtStorageCmd::Put { content, cid } => {
                            self.storage.insert(cid, content);
                        },
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dht_storage() {
        todo!("Testing logic");
    }
}
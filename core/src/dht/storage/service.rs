use std::{collections::HashMap, io::SeekFrom, path::PathBuf, time::Duration};
use bytes::Bytes;
use tokio::{fs::{File, OpenOptions}, io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt}, sync::mpsc, time::interval};
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
                    file.seek(SeekFrom::Start(0)).await?;
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
    use std::{path::PathBuf, time::Duration};
    use bytes::Bytes;
    use tempfile::tempdir;
    use tokio::{task::JoinHandle, time::sleep};
    use tokio_util::sync::CancellationToken;

    use crate::{dht::{CID, DhtStorage, DhtStorageDispatcher, DhtStorageError}, service::Service, utils::random_bytes};

    async fn setup_service(path: PathBuf, token: CancellationToken) -> (DhtStorageDispatcher, JoinHandle<Result<(), DhtStorageError>>) {
        let (service, dispatcher) = DhtStorage::create(path).await.unwrap();
        let handle = tokio::spawn(service.run(token));

        (dispatcher, handle)
    }

    fn example_content() -> (CID, Bytes) {
        let buf = Bytes::copy_from_slice(&[1u8; 100]);
        (CID::new(&buf), buf)
    }

    #[tokio::test]
    async fn test_content() {
        let tmp = tempdir().unwrap();
        let (dispatcher, _) = setup_service(tmp.path().join("dht1.bin"), CancellationToken::new()).await;
        let (cid, content) = example_content();

        dispatcher.put(cid.clone(), content.clone()).await.expect("save content failed");

        println!("Waiting 2 seconds...");
        sleep(Duration::from_secs(2)).await;
        
        let result1 = dispatcher.get(cid).await.expect("get content failed");
        assert_eq!(result1, Some(content));

        let result2 = dispatcher.get(CID::new(&random_bytes::<32>())).await.expect("get content failed");
        assert_eq!(result2, None);
    }

    #[tokio::test]
    async fn test_load_from_file() {
        let token = CancellationToken::new();
        let tmp = tempdir().unwrap();
        let p = tmp.path().join("dht2.bin");

        let (dispatcher_1, handle) = setup_service(p.clone(), token.clone()).await;
        let (cid, content) = example_content();

        dispatcher_1.put(cid.clone(), content.clone()).await.expect("save content failed");

        println!("Waiting 1 seconds...");
        sleep(Duration::from_secs(1)).await;

        token.cancel();
        handle.await.unwrap().unwrap();

        let (service, dispatcher_2) = DhtStorage::load_from_file(p).await.unwrap();
        tokio::spawn(service.run(CancellationToken::new()));

        let result = dispatcher_2.get(cid).await.expect("get content failed");
        assert_eq!(result, Some(content));
    }
}
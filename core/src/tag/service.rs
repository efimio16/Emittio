use std::{collections::BTreeMap, io::SeekFrom, path::PathBuf};
use tokio::{fs::{File, OpenOptions}, io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt}, sync::mpsc, time::{Duration, interval}};
use tokio_util::sync::CancellationToken;
use std::mem;

use crate::{service::Service, tag::{Tag, TagDispatcher, TagServiceCmd, TagServiceError}, utils::{self, ChannelError}};

const TTL_PRODUCTION: u64 = 3600 * 24 * 7;
const TTL_DEVELOPMENT: u64 = 5;
const MIN_HASH: [u8; 32] = [0u8; 32];
const CHAN_SIZE: usize = 10000;
const TICK_INTERVAL: Duration = Duration::from_secs(5);

pub struct TagService {
    rx: mpsc::Receiver<TagServiceCmd>,
    tags: BTreeMap<(u64, [u8; 32]), Tag>,
    ttl_seconds: u64,
    path: PathBuf,
    pending_tags: Vec<Tag>,
}

impl TagService {
    pub async fn create(path: PathBuf) -> Result<(Self, TagDispatcher), TagServiceError> {
        File::create(path.clone()).await?;

        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        Ok((Self { rx, tags: BTreeMap::new(), ttl_seconds: TTL_DEVELOPMENT, path, pending_tags: Vec::new() }, TagDispatcher { tx }))
    }

    pub async fn load_from_file(path: PathBuf) -> Result<(Self, TagDispatcher), TagServiceError> {
        let mut buffer = Vec::new();
        File::open(path.clone()).await?.read_to_end(&mut buffer).await?;

        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        let tags: BTreeMap<(u64, [u8; 32]), Tag> = utils::deserialize(&buffer)?;

        Ok((Self { rx, tags, ttl_seconds: TTL_DEVELOPMENT, path, pending_tags: Vec::new() }, TagDispatcher { tx }))
    }
}

impl Service for TagService {
    type Error = TagServiceError;
    
    async fn run(mut self, token: CancellationToken) -> Result<(), TagServiceError> {
        println!("Running tag service");

        let mut file = OpenOptions::new()
            .write(true)
            .open(self.path.clone()).await?;

        let mut ticker = interval(TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    file.flush().await?;
                    file.sync_all().await?;

                    return Ok(());
                }
                _ = ticker.tick() => {
                    for tag in self.pending_tags.drain(..) {
                        self.tags.insert((tag.created_at, tag.hash), tag);
                    }

                    let expire_time = utils::get_timestamp() - self.ttl_seconds;

                    let mut expired = self.tags.split_off(&(expire_time + 1, MIN_HASH));
                    mem::swap(&mut self.tags, &mut expired);

                    file.set_len(0).await?;
                    file.seek(SeekFrom::Start(0)).await?;
                    file.write_all(&utils::serialize(&self.tags)?).await?;
                }
                Some(cmd) = self.rx.recv() => {
                    match cmd {
                        TagServiceCmd::Put(tag) => {
                            self.pending_tags.push(tag);
                        },
                        TagServiceCmd::Get(reply_tx) => {
                            let tags_vec = self.tags.values().cloned().collect();
                            reply_tx.send(tags_vec).map_err(|_| ChannelError::Closed)?;
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
    use tempfile::tempdir;
    use tokio::{task::JoinHandle, time::sleep};
    use tokio_util::sync::CancellationToken;

    use crate::{service::Service, tag::{Tag, TagDispatcher, TagPayload, TagService, TagServiceError}, utils::random_bytes};

    async fn setup_service(path: PathBuf, token: CancellationToken) -> (TagDispatcher, JoinHandle<Result<(), TagServiceError>>) {
        let (service, dispatcher) = TagService::create(path).await.unwrap();
        let handle = tokio::spawn(service.run(token));

        (dispatcher, handle)
    }

    fn example_tag() -> Tag {
        Tag::new(&random_bytes(), TagPayload { data: b"Hello!".into() }).unwrap()
    }

    #[tokio::test]
    async fn test_tag_exists() {
        let tmp = tempdir().unwrap();
        let (dispatcher, _) = setup_service(tmp.path().join("tags1.bin"), CancellationToken::new()).await;
        let tag = example_tag();

        dispatcher.put_tag(tag.clone()).await.expect("send tag failed");

        println!("Waiting 2 seconds...");
        sleep(Duration::from_secs(2)).await;
        
        let tags = dispatcher.get_tags().await.expect("receive tags failed");
        assert_eq!(tags[0].hash, tag.hash);
    }

    #[tokio::test]
    async fn test_tag_disappears() {
        let tmp = tempdir().unwrap();
        let (dispatcher, _) = setup_service(tmp.path().join("tags2.bin"), CancellationToken::new()).await;
        let tag = example_tag();

        dispatcher.put_tag(tag.clone()).await.expect("send tag failed");

        println!("Waiting 10 seconds...");
        sleep(Duration::from_secs(10)).await;

        let tags = dispatcher.get_tags().await.expect("receive tags failed");
        assert_eq!(tags.len(), 0);
    }

    #[tokio::test]
    async fn test_load_from_file() {
        let token = CancellationToken::new();
        let tmp = tempdir().unwrap();
        let p = tmp.path().join("tags3.bin");

        let (dispatcher_1, handle) = setup_service(p.clone(), token.clone()).await;
        let tag = example_tag();

        dispatcher_1.put_tag(tag.clone()).await.expect("send tag failed");

        println!("Waiting 1 seconds...");
        sleep(Duration::from_secs(1)).await;

        token.cancel();
        handle.await.unwrap().unwrap();

        let (service, dispatcher_2) = TagService::load_from_file(p).await.unwrap();
        tokio::spawn(service.run(CancellationToken::new()));

        let tags = dispatcher_2.get_tags().await.expect("receive tags failed");
        assert_eq!(tags[0].hash, tag.hash);
    }
}
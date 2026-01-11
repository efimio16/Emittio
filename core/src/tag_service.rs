use std::{collections::BTreeMap, path::PathBuf};
use tokio::{fs::OpenOptions, io::{AsyncReadExt, AsyncWriteExt}, sync::{mpsc, oneshot}, time::{Duration, interval}};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::{channels::ChannelError, service::Service, tag::Tag, utils::{self, SerdeError}};

const TTL_PRODUCTION: usize = 3600 * 24 * 7;
const TTL_DEVELOPMENT: usize = 20;
const MIN_HASH: [u8; 32] = [0u8; 32];

#[derive(Debug, Error)]
pub enum TagServiceError {
    #[error(transparent)]
    Serde(#[from] SerdeError),

    #[error(transparent)]
    Io(#[from] tokio::io::Error),

    #[error(transparent)]
    Channel(#[from] ChannelError),
}

pub struct TagDispatcher {
    tag_tx: mpsc::Sender<Tag>,
    get_tx: mpsc::Sender<oneshot::Sender<Vec<Tag>>>,
}

impl TagDispatcher {
    pub async fn send_tag(&self, tag: Tag) -> Result<(), ChannelError> {
        self.tag_tx.send(tag).await.map_err(|_| ChannelError::Closed)
    }
    pub async fn get_tags(&mut self) -> Result<Vec<Tag>, ChannelError> {
        let (tx, rx) = oneshot::channel();
        self.get_tx.send(tx).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)
    }
}

pub struct TagService {
    tag_rx: mpsc::Receiver<Tag>,
    get_rx: mpsc::Receiver<oneshot::Sender<Vec<Tag>>>,
    tags: BTreeMap<(u64, [u8; 32]), Tag>,
    ttl_seconds: u64,
    path: PathBuf,
}

impl TagService {
    pub fn new(path: PathBuf) -> (Self, TagDispatcher) {
        let (tag_tx, tag_rx) = mpsc::channel(10000);
        let (get_tx, get_rx) = mpsc::channel(10000);
        (
            Self { tag_rx, get_rx, tags: BTreeMap::new(), ttl_seconds: TTL_DEVELOPMENT as u64, path },
            TagDispatcher { tag_tx, get_tx }
        )
    }

    pub async fn load(&mut self) -> Result<(), TagServiceError> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(self.path.clone())
            .await?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        self.tags.append(&mut utils::deserialize(&buffer)?);
        Ok(())
    }
}

impl Service for TagService {
    type Error = TagServiceError;
    
    async fn run(mut self, token: CancellationToken) -> Result<(), TagServiceError> {
        println!("Running tag service");

        let mut options = OpenOptions::new();
        options.write(true).create(true);

        let mut ticker = interval(Duration::from_secs(5));
        
        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    return Ok(());
                }
                _ = ticker.tick() => {
                    while let Ok(tag) = self.tag_rx.try_recv() {
                        self.tags.insert((tag.created_at, tag.hash), tag);
                    }

                    let expire_time = utils::get_timestamp() - self.ttl_seconds;

                    let mut expired = self.tags.split_off(&(expire_time + 1, MIN_HASH));
                    std::mem::swap(&mut self.tags, &mut expired);

                    let mut file = options.open(self.path.clone()).await?;

                    file.set_len(0).await?;
                    file.write_all(&utils::serialize(&self.tags)?).await?;
                }
                Some(reply_tx) = self.get_rx.recv() => {
                    let tags_vec: Vec<_> = self.tags.values().cloned().collect();
                    reply_tx.send(tags_vec).map_err(|_| ChannelError::Closed)?;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use tokio_util::sync::CancellationToken;

    use crate::{tag::{Tag, TagPayload}, tag_service::{TagDispatcher, TagService}, service::Service};

    fn setup_service(path: &str) -> TagDispatcher {
        let (service, dispatcher) = TagService::new(path.into());
        tokio::spawn(async move { service.run(CancellationToken::new()).await.unwrap() });

        dispatcher
    }

    fn example_tag() -> Tag {
        Tag::new(&rand::random(), TagPayload { data: b"Hello!".into() }).unwrap()
    }

    #[tokio::test]
    async fn test_tag_exists() {
        let mut dispatcher = setup_service("tags1.bin");
        let tag = example_tag();

        dispatcher.send_tag(tag.clone()).await.expect("send tag failed");

        println!("Waiting 5 seconds...");
        sleep(Duration::from_secs(5)).await;
        
        let tags = dispatcher.get_tags().await.expect("receive tags failed");
        assert_eq!(tags[0].hash, tag.hash);
    }

    #[tokio::test]
    async fn test_tag_disappears() {
        let mut dispatcher = setup_service("tags2.bin");
        let tag = example_tag();

        dispatcher.send_tag(tag.clone()).await.expect("send tag failed");

        println!("Waiting 25 seconds...");
        sleep(Duration::from_secs(25)).await;

        let tags = dispatcher.get_tags().await.expect("receive tags failed");
        assert_eq!(tags.len(), 0);
    }

    #[tokio::test]
    async fn test_load_from_file() {
        let dispatcher_1 = setup_service("tags3.bin");
        let tag = example_tag();

        dispatcher_1.send_tag(tag.clone()).await.expect("send tag failed");

        println!("Waiting 5 seconds...");
        sleep(Duration::from_secs(5)).await;

        println!("Waiting 1 more second...");
        sleep(Duration::from_secs(1)).await;

        let (mut service, mut dispatcher_2) = TagService::new("tags3.bin".into());
        tokio::spawn(async move { service.load().await.unwrap(); service.run(CancellationToken::new()).await.unwrap() });

        let tags = dispatcher_2.get_tags().await.expect("receive tags failed");
        assert_eq!(tags[0].hash, tag.hash);
    }
}
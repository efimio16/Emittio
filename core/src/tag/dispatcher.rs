use tokio::sync::{mpsc, oneshot};

use crate::{utils::ChannelError, tag::Tag};

pub enum TagServiceCmd {
    Put(Tag),
    Get(oneshot::Sender<Vec<Tag>>)
}

#[derive(Clone)]
pub struct TagDispatcher {
    pub(super) tx: mpsc::Sender<TagServiceCmd>,
}

impl TagDispatcher {
    pub async fn put_tag(&self, tag: Tag) -> Result<(), ChannelError> {
        self.tx.send(TagServiceCmd::Put(tag)).await.map_err(|_| ChannelError::Closed)
    }
    pub async fn get_tags(&self) -> Result<Vec<Tag>, ChannelError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(TagServiceCmd::Get(tx)).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)
    }
}
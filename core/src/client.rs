use tokio::sync::{mpsc, oneshot};
use std::time::Duration;
use tokio::time::{timeout};
use thiserror::Error;

use crate::{channels::{self, ChannelError}, client_service::{ClientCmd, ClientService}, message::{Action, OutgoingMessage, Payload, Query, Reply, ReplyErr}, peer::PeerId, tag::Tag, transport::TransportHandler};

const CHAN_SIZE: usize = 128;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error("Timeout")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    Reply(#[from] ReplyErr),

    #[error("Invalid reply")]
    InvalidReply,
}

pub struct Client {
    cmd_tx: mpsc::Sender<ClientCmd>,
}

impl Client {
    pub fn new() -> (Self, ClientService, TransportHandler) {
        let (tr_dispatcher, tr_handler) = channels::new(128);
        let (cmd_tx, cmd_rx) = mpsc::channel(CHAN_SIZE);

        (Self { cmd_tx }, ClientService::new(cmd_rx, tr_dispatcher), tr_handler)
    }

    pub async fn get_tags(&mut self, peer_id: &PeerId) -> Result<Reply, ClientError> {
        self.send(peer_id, Payload::Query(Query::GetTags)).await
    }

    pub async fn publish_tag(&mut self, peer_id: &PeerId, tag: Tag) -> Result<(), ClientError> {
        if let Reply::RequirePow(pow) = self.send(peer_id, Payload::Query(Query::GetPow(Action::PublishTag))).await? {
            let nonce = pow.solve();
            self.send(peer_id, Payload::Query(Query::PublishTag { tag, pow, nonce })).await?.as_ok()?;
            Ok(())
        } else {
            Err(ClientError::InvalidReply)
        }
    }

    pub async fn send(
        &self,
        to: &PeerId,
        payload: Payload,
    ) -> Result<Reply, ClientError> {
        let (tx, rx) = oneshot::channel();

        self.cmd_tx.send(ClientCmd {
            msg: OutgoingMessage::new(to, payload),
            reply_tx: tx,
        }).await.map_err(|_| ChannelError::Closed)?;

        timeout(Duration::from_secs(5), rx)
            .await?
            .map_err(|_| ChannelError::Closed.into())
    }
}
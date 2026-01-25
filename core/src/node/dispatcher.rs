use std::time::Duration;
use tokio::{sync::{mpsc, oneshot}, time::timeout};

use crate::{message::OutgoingMessage, net::NetClient, node::NodeError, payload::{Payload, Query, Reply}, peer::PeerId, transport::TransportParticipant, utils::ChannelError};

const TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct NodeCmd {
    pub(super) msg: OutgoingMessage,
    pub(super) reply_tx: oneshot::Sender<Reply>,
}

#[derive(Clone)]
pub struct NodeDispatcher {
    secret: [u8; 32],
    tx: mpsc::Sender<NodeCmd>,
}

impl NodeDispatcher {
    pub(super) fn new(secret: [u8; 32], tx: mpsc::Sender<NodeCmd>) -> Self {
        Self { secret, tx }
    }

    pub async fn send<Q>(&self, to: &PeerId, query: Q) -> Result<Reply, NodeError>
    where Query: From<Q> {
        let (tx, rx) = oneshot::channel();

        self.tx.send(NodeCmd {
            msg: OutgoingMessage::new(to, Payload::Query(query.into())),
            reply_tx: tx,
        }).await.map_err(|_| ChannelError::Closed)?;

        let reply = timeout(TIMEOUT, rx)
            .await?
            .map_err(|_| ChannelError::Closed)?;

        Ok(reply)
    }
}

impl TransportParticipant for NodeDispatcher {
    fn net_client(&self) -> NetClient {
        NetClient::from_seed(self.secret)
    }
}
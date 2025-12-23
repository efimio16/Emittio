use tokio::sync::{mpsc, oneshot};
use std::time::Duration;
use tokio::time::{timeout};

use crate::{channels, client_service::{ClientCmd, ClientService}, message::{Action, IncomingMessage, OutgoingMessage, Payload, Query, Reply}, peer::PeerId, tag::Tag};

const CHAN_SIZE: usize = 128;

pub struct Client {
    cmd_tx: mpsc::Sender<ClientCmd>,
}

impl Client {
    pub fn new() -> (Self, ClientService, channels::Handler<OutgoingMessage, IncomingMessage>) {
        let (s_params, t_params) = channels::new(128);
        let (cmd_tx, cmd_rx) = mpsc::channel(CHAN_SIZE);

        (Self { cmd_tx }, ClientService::new(cmd_rx, s_params), t_params)
    }

    pub async fn get_tags(&mut self, peer_id: &PeerId) -> Result<Reply, String> {
        self.send(peer_id, Payload::Query(Query::GetTags)).await
    }

    pub async fn publish_tag(&mut self, peer_id: &PeerId, tag: Tag) -> Result<(), String> {
        if let Reply::RequirePow(pow) = self.send(peer_id, Payload::Query(Query::GetPow(Action::PublishTag))).await? {
            let nonce = pow.solve();
            self.send(peer_id, Payload::Query(Query::PublishTag { tag, pow, nonce })).await?.as_ok()?;
            return Ok(())
        }
        Err("The reply is not a PoW".into())
    }

    pub async fn send(
        &self,
        to: &PeerId,
        payload: Payload,
    ) -> Result<Reply, String> {
        let (tx, rx) = oneshot::channel();

        self.cmd_tx.send(ClientCmd {
            msg: OutgoingMessage::new(to, payload),
            reply_tx: tx,
        }).await.map_err(|_| "Client runner stopped")?;

        timeout(Duration::from_secs(5), rx)
            .await
            .map_err(|_| "Timeout")?
            .map_err(|_| "Reply channel dropped".into())
    }
}
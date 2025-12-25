use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
use thiserror::Error;

use crate::{channels::ChannelError, message::{IncomingMessage, MsgId, OutgoingMessage, Payload, Reply}, transport::TransportDispatcher};

const MAX_PENDING: usize = 1024;

#[derive(Debug, Error)]
pub enum ClientServiceError {
    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error("Too many pending")]
    TooManyPending,
}

pub struct ClientCmd {
    pub msg: OutgoingMessage,
    pub reply_tx: oneshot::Sender<Reply>,
}

pub struct ClientService {
    cmd_rx: mpsc::Receiver<ClientCmd>,
    chans: TransportDispatcher,
    pending: HashMap<MsgId, oneshot::Sender<Reply>>,
}

impl ClientService {
    pub fn new(cmd_rx: mpsc::Receiver<ClientCmd>, chans: TransportDispatcher) -> Self {
        Self {
            cmd_rx,
            chans,
            pending: HashMap::with_capacity(MAX_PENDING),
        }
    }

    pub async fn run(&mut self) -> Result<(), ClientServiceError> {
        loop {
            tokio::select! {
                Some(cmd) = self.cmd_rx.recv() => { self.handle_cmd(cmd).await?; }
                Some(msg) = self.chans.recv() => { self.handle_recv(msg)?; }
            }
        }
    }

    async fn handle_cmd(&mut self, cmd: ClientCmd) -> Result<(), ClientServiceError> {
        if self.pending.len() >= MAX_PENDING {
            return Err(ClientServiceError::TooManyPending);
        }

        self.pending.insert(cmd.msg.id.clone(), cmd.reply_tx);
        self.chans.send(cmd.msg).await?;
        Ok(())
    }

    fn handle_recv(&mut self, msg: IncomingMessage) -> Result<(), ChannelError> {
        if let Payload::Reply(reply) = msg.payload {
            if let Some(tx) = self.pending.remove(&msg.id) {
                tx.send(reply).map_err(|_| ChannelError::Closed)?;
            }
        }
        Ok(())
    }
}

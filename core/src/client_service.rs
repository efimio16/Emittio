use std::collections::HashMap;

use tokio::sync::{mpsc, oneshot};

use crate::{channels, message::{IncomingMessage, MsgId, OutgoingMessage, Payload, Reply}};

const MAX_PENDING: usize = 1024;

pub struct ClientCmd {
    pub msg: OutgoingMessage,
    pub reply_tx: oneshot::Sender<Reply>,
}

pub struct ClientService {
    cmd_rx: mpsc::Receiver<ClientCmd>,
    chans: channels::Dispatcher<IncomingMessage, OutgoingMessage>,
    pending: HashMap<MsgId, oneshot::Sender<Reply>>,
}

impl ClientService {
    pub fn new(cmd_rx: mpsc::Receiver<ClientCmd>, chans: channels::Dispatcher<IncomingMessage, OutgoingMessage>) -> Self {
        Self {
            cmd_rx,
            chans,
            pending: HashMap::with_capacity(MAX_PENDING),
        }
    }

    pub async fn run(&mut self) -> Result<(), String> {
        loop {
            tokio::select! {
                Some(cmd) = self.cmd_rx.recv() => { self.handle_cmd(cmd).await?; }
                Some(msg) = self.chans.rx.recv() => { self.handle_recv(msg)?; }
            }
        }
    }

    async fn handle_cmd(&mut self, cmd: ClientCmd) -> Result<(), String> {
        if self.pending.len() >= MAX_PENDING {
            return Err("Too many pending".into());
        }

        self.pending.insert(cmd.msg.id.clone(), cmd.reply_tx);
        self.chans.tx.send(cmd.msg).await.map_err(|_| "Send failed")?;
        Ok(())
    }

    fn handle_recv(&mut self, msg: IncomingMessage) -> Result<(), String> {
        if let Payload::Reply(reply) = msg.payload {
            if let Some(tx) = self.pending.remove(&msg.id) {
                tx.send(reply).map_err(|_| "send failed")?;
            }
        }
        Ok(())
    }
}

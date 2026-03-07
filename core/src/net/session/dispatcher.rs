use tokio::sync::{mpsc, oneshot};

use crate::{message::{IncomingMessage, OutgoingMessage}, net::{ConnId, Handshake, HandshakeAck, Message, NetIdentity, SessionManagerDispatcherError}, peer::PeerId, utils::ChannelError};

pub(super) enum SessionManagerCmd {
    Handshake {
        with: NetIdentity,
        addr: String,
        tx: oneshot::Sender<Result<Handshake, SessionManagerDispatcherError>>,
    },
    Accept {
        handshake: Handshake,
        addr: String,
        tx: oneshot::Sender<Result<HandshakeAck, SessionManagerDispatcherError>>,
    },
    Confirm {
        ack: HandshakeAck,
        tx: oneshot::Sender<Result<ConnId, SessionManagerDispatcherError>>,
    },
    Conns {
        of: PeerId,
        tx: oneshot::Sender<Vec<ConnId>>,
    },
    Addr {
        of: PeerId,
        tx: oneshot::Sender<Option<String>>,
    },
    Send {
        conn_id: ConnId,
        msg: OutgoingMessage,
        tx: oneshot::Sender<Result<Message, SessionManagerDispatcherError>>,
    },
    Recv {
        msg: Message,
        tx: oneshot::Sender<Result<IncomingMessage, SessionManagerDispatcherError>>,
    },
}

#[derive(Clone)]
pub struct SessionManagerDispatcher {
    pub(super) tx: mpsc::Sender<SessionManagerCmd>,
}

impl SessionManagerDispatcher {
    pub async fn handshake(&self, with: NetIdentity, addr: String) -> Result<Handshake, SessionManagerDispatcherError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(SessionManagerCmd::Handshake { with, addr, tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)?
    }
    pub async fn accept(&self, handshake: Handshake, addr: String) -> Result<HandshakeAck, SessionManagerDispatcherError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(SessionManagerCmd::Accept { handshake, addr, tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)?
    }
    pub async fn confirm(&self, ack: HandshakeAck) -> Result<ConnId, SessionManagerDispatcherError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(SessionManagerCmd::Confirm { ack, tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)?
    }
    pub async fn connections(&self, of: PeerId) -> Result<Vec<ConnId>, SessionManagerDispatcherError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(SessionManagerCmd::Conns { of, tx }).await.map_err(|_| ChannelError::Closed)?;
        Ok(rx.await.map_err(|_| ChannelError::Closed)?)
    }
    pub async fn addr(&self, of: PeerId) -> Result<Option<String>, SessionManagerDispatcherError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(SessionManagerCmd::Addr { of, tx }).await.map_err(|_| ChannelError::Closed)?;
        Ok(rx.await.map_err(|_| ChannelError::Closed)?)
    }
    pub async fn send(&self, conn_id: ConnId, msg: OutgoingMessage) -> Result<Message, SessionManagerDispatcherError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(SessionManagerCmd::Send { conn_id, msg, tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)?
    }
    pub async fn recv(&self, msg: Message) -> Result<IncomingMessage, SessionManagerDispatcherError> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(SessionManagerCmd::Recv { msg, tx }).await.map_err(|_| ChannelError::Closed)?;
        rx.await.map_err(|_| ChannelError::Closed)?
    }
}
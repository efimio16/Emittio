use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::{mpsc, oneshot};
use std::time::Duration;
use tokio::time::{timeout};

use crate::{client::{ClientCmd, ClientError, ClientService}, dht::CID, message::OutgoingMessage, net::NetClient, payload::{Action, DhtQuery, DhtReply, Payload, PowQuery, PowReply, Query, Reply, TagQuery, TagReply}, peer::PeerId, tag::Tag, transport::{TransportDispatcher, TransportParticipant}, utils::{ChannelError, deserialize, serialize}};

const CHAN_SIZE: usize = 128;
const TIMEOUT: Duration = Duration::from_secs(5);

pub struct Client {
    tx: mpsc::Sender<ClientCmd>,
}

impl Client {
    pub fn new(transport_dispatcher: TransportDispatcher) -> (Self, ClientService) {
        let (tx, rx) = mpsc::channel(CHAN_SIZE);

        (Self { tx }, ClientService::new(rx, transport_dispatcher))
    }

    pub async fn get_tags(&mut self, peer_id: &PeerId) -> Result<Vec<Tag>, ClientError> {
        match self.send(peer_id, TagQuery::Get).await?.try_into()? {
            TagReply::Return(tags) => Ok(tags)
        }
    }

    pub async fn publish_tag(&mut self, peer_id: &PeerId, tag: Tag) -> Result<(), ClientError> {
        let PowReply::Require(pow) = self.send(peer_id, PowQuery::Get(Action::PublishTag)).await?.try_into()? else {
            return Err(ClientError::InvalidReply);
        };

        let nonce = pow.solve();
        
        match self.send(peer_id, TagQuery::Publish { tag, pow, nonce }).await? {
            Reply::Ok => Ok(()),
            _ => Err(ClientError::InvalidReply),
        }
    }

    pub async fn dht_get<T: DeserializeOwned>(&mut self, peer_id: &PeerId, cid: CID) -> Result<Option<T>, ClientError> {
        match self.send(peer_id, DhtQuery::Get(cid)).await?.try_into()? {
            DhtReply::Return(opt_bytes) => Ok(opt_bytes.map(|b| deserialize(&b)).transpose()?),
        }
    }

    pub async fn dht_put<T: Serialize + ?Sized>(&mut self, peer_id: &PeerId, content: &T) -> Result<CID, ClientError> {
        let bytes = serialize(content)?;
        let cid = CID::new(&bytes);

        match self.send(peer_id, DhtQuery::Put(bytes.into())).await? {
            Reply::Ok => Ok(cid),
            _ => Err(ClientError::InvalidReply),
        }
    }

    async fn send<Q>(&self, to: &PeerId, query: Q) -> Result<Reply, ClientError>
    where Query: From<Q> {
        let (tx, rx) = oneshot::channel();

        self.tx.send(ClientCmd {
            msg: OutgoingMessage::new(to, Payload::Query(query.into())),
            reply_tx: tx,
        }).await.map_err(|_| ChannelError::Closed)?;

        let reply = timeout(TIMEOUT, rx)
            .await?
            .map_err(|_| ChannelError::Closed)?;

        Ok(reply)
    }
}

impl TransportParticipant for Client {
    fn net_client(&self) -> NetClient {
        NetClient::Ephemeral
    }
}
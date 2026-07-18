use bytes::{Bytes, BytesMut};
use emittio_crypto::id::Id;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{actor::NetworkActorHandle, error::NetworkError, reply::Replyable, verifier::{NoVerifier, Verifier}};

/// Describes how to select peers
#[derive(Clone)]
pub enum PeerSelection {
    /// Selects N closest peers to the `target`
    Closest {
        target: Id,
        count: u16,
    },
    /// Selects N random peers
    Random {
        count: u8,
    },
    /// Selects no more than `max_count` peers that satisfy `peer_id & bucket == bucket` (`peer_id` is in the `bucket`)
    InBucket {
        bucket: Id,
        max_count: u64,
    },
}

/// Network-level query struct with a data in raw bytes and configs about its destination and verification
#[derive(Serialize, Deserialize, Clone)]
pub struct Query {
    /// The query's data in bytes
    pub bytes: Bytes,
    /// The service that processes it
    pub service_id: u16,
    /// Query type
    pub method_id: u16,
    /// Identifies a single query instance. Used to match query with a reply
    pub query_id: u16,
}

pub trait Queryable: Serialize + DeserializeOwned {
    const SERVICE_ID: u16;
    const METHOD_ID: u16;

    type Reply: Replyable;

    /// Who to send this query to
    fn peer_selection(&self) -> PeerSelection;
    /// Structure that will verify replies
    fn verifier(&self) -> impl Verifier<Self::Reply> {
        NoVerifier
    }
    /// How many times should we repeat the query if we don't receive a single valid answer
    fn retries() -> u8 {
        3
    }

    /// Send the query through a network handle
    fn query(&self, network: &NetworkActorHandle) -> impl Future<Output = Result<Option<Self::Reply>, NetworkError>> {
        async move {
            let mut bytes = BytesMut::new();

            postcard::to_slice(self, &mut bytes[..])?;

            let q = Query {
                bytes: bytes.freeze(),
                service_id: Self::SERVICE_ID,
                method_id: Self::METHOD_ID,
                query_id: 0, // network actor chooses it
            };

            let verifier = self.verifier();
            let peer_selection = self.peer_selection();

            let mut retries = Self::retries();

            loop {
                let replies = network.query(peer_selection.clone(), q.clone()).await??
                    .into_iter()
                    .filter_map(|(id, r)| r.parse().ok().map(|r| (id, r)))
                    .collect();

                let (results, reply) = verifier.verify(replies);

                network.verifications(results).await?;

                if let Some(reply) = reply {
                    return Ok(Some(reply));
                }

                if retries == 0 {
                    return Ok(None);
                }
                retries -= 1;
            }
        }
    }
}

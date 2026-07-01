use std::collections::HashMap;
use actorify::{tokio_util::sync::CancellationToken, Actor, ActorJoinMap};
use emittio_crypto::{OsRng, RngCore, blake3, derivable::Derivable, kem::Kem, tag::TagVerifier};
use emittio_inbox::{InboxActor, InboxActorHandle};
use emittio_network::actor::{NetworkActorHandle, NetworkActor};

type InboxId = [u8; 32];

const INBOX_CTX: &str = "inbox";

pub struct Client {
    seed: [u8; 32],
    inboxes: HashMap<InboxId, InboxActorHandle>,
    inbox_actors: ActorJoinMap<InboxId>,
    network: NetworkActorHandle,
}

impl Client {
    pub fn new_seed() -> (Self, [u8; 32]) {
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);
        (Self::from_seed(seed), seed)
    }

    #[inline]
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let handle = NetworkActor::new().spawn();

        Self {
            seed,
            inboxes: HashMap::new(),
            inbox_actors: ActorJoinMap::new(),
            network: handle,
        }
    }

    pub fn use_inbox(&mut self, name: &str) -> &InboxActorHandle {
        let inbox_id: InboxId = blake3::derive_key(INBOX_CTX, name.as_bytes()).into();

        let inbox = self.inboxes
            .entry(inbox_id)
            .or_insert_with(|| {
                let (handle, actor_future) = InboxActor::new(self.network.clone(), Kem::derive_with_info(self.seed, &inbox_id), TagVerifier::derive_with_info(self.seed, &inbox_id))
                    .run(CancellationToken::new());
                
                self.inbox_actors.spawn(inbox_id, actor_future);
                handle
            });

        inbox
    }
}

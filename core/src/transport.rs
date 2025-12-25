use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::mpsc;

use crate::{channels, message::{IncomingMessage, OutgoingMessage}, peer::PeerId, utils::{deserialize, serialize}};

pub type TransportDispatcher = channels::Participant<IncomingMessage, OutgoingMessage>;
pub type TransportHandler = channels::Participant<OutgoingMessage, IncomingMessage>;

pub struct MockTransport {
    channels: Arc<Mutex<HashMap<PeerId, mpsc::Sender<IncomingMessage>>>>,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_peer(&self, peer: PeerId, mut params: TransportHandler) {
        let channels = Arc::clone(&self.channels);
        channels.lock().expect("Mutex error").insert(peer.clone(), params.tx);

        tokio::spawn(async move {
            while let Some(msg) = params.rx.recv().await {
                let channel = {
                    let guard = channels.lock().expect("Mutex error");
                    guard.get(&msg.to).cloned()
                };

                let bytes = serialize(&msg).expect("serialization failed");
                
                if let Some(channel) = channel {
                    channel.send(
                        IncomingMessage::receive(
                            peer.clone(),
                            deserialize(&bytes).expect("deserialization failed")
                        )
                    ).await.map_err(|_| "Sending error".to_string())?;
                }
            }

            Ok::<_, String>(())
        });
    }
}
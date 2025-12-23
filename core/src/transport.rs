use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::mpsc;

use crate::{channels, message::{IncomingMessage, OutgoingMessage}, peer::PeerId, utils::{deserialize, serialize}};

// pub mod transport_params {
//     use tokio::sync::mpsc;

//     use crate::message::{OutgoingMessage, IncomingMessage};

//     const CHAN_SIZE: usize = 128;

//     pub struct Transport {
//         pub rx: mpsc::Receiver<OutgoingMessage>,
//         pub tx: mpsc::Sender<IncomingMessage>,
//     }
//     pub struct Service {
//         pub rx: mpsc::Receiver<IncomingMessage>,
//         pub tx: mpsc::Sender<OutgoingMessage>,
//     }

//     pub fn new() -> (Service, Transport) {
//         let (send_tx, send_rx) = mpsc::channel(CHAN_SIZE);
//         let (recv_tx, recv_rx) = mpsc::channel(CHAN_SIZE);

//         (Service { rx: recv_rx, tx: send_tx }, Transport { rx: send_rx, tx: recv_tx })
//     }
// }

pub struct MockTransport {
    channels: Arc<Mutex<HashMap<PeerId, mpsc::Sender<IncomingMessage>>>>,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_peer(&self, peer: PeerId, mut params: channels::Handler<OutgoingMessage, IncomingMessage>) {
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
mod error;
mod dispatcher;
mod service;
mod session;

pub use error::*;
pub use dispatcher::*;
pub use service::*;
pub use session::*;

use tokio::sync::mpsc;

use crate::net::NetClient;

const CHAN_SIZE: usize = 10000;

pub struct SessionManager;

impl SessionManager {
    pub fn new(client: NetClient) -> (SessionManagerDispatcher, SessionManagerService) {
        let (tx, rx) = mpsc::channel(CHAN_SIZE);
        (SessionManagerDispatcher { tx }, SessionManagerService::new(client, rx))
    }
}
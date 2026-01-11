use thiserror::Error;
use tokio::task::{JoinError, JoinHandle, JoinSet};
use tokio_util::sync::CancellationToken;
use std::future::Future;

use crate::{channels::ChannelError, client_service::ClientServiceError, peer_table::PeerTableError, tag_service::TagServiceError, transport::MockTransportError};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Join(#[from] JoinError),
    
    #[error(transparent)]
    MockTransport(#[from] MockTransportError),
    
    #[error(transparent)]
    TagService(#[from] TagServiceError),

    #[error(transparent)]
    PeerTable(#[from] PeerTableError),

    #[error(transparent)]
    ClientService(#[from] ClientServiceError),

    #[error(transparent)]
    Channel(#[from] ChannelError),
}

pub trait Service {
    type Error: Into<ServiceError>;
    fn run(self, token: CancellationToken) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub struct ServiceManager {
    services: JoinSet<Result<(), ServiceError>>,
    token: CancellationToken,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self { services: JoinSet::new(), token: CancellationToken::new() }
    }

    pub fn spawn<S: Service + 'static + Send>(&mut self, service: S)
    where ServiceError: From<<S as Service>::Error> {
        let token = self.token.clone();
        self.services.spawn(async move { service.run(token).await?; Ok(()) });
    }

    pub fn run(mut self) -> (JoinHandle<Result<(), ServiceError>>, CancellationToken) {
        println!("Running all services");

        let token = self.token.clone();

        (tokio::spawn(async move {
            let Some(res) = self.services.join_next().await else { return Ok(()); };

            token.cancel();
            let _ = self.services.join_all().await;

            res?
        }), self.token)
    }
}
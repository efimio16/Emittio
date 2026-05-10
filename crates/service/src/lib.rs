pub mod channel;

// use tokio::task::{JoinError, JoinHandle, JoinSet};
use tokio_util::sync::CancellationToken;
// use std::{future::Future, sync::mpsc};

// use crate::{client::ClientServiceError, dht::{DhtRoutingError, DhtStorageError}, net::SessionManagerServiceError, node::NodeError, peer::PeerTableError, tag::TagServiceError, transport::MockTransportError, utils::ChannelError};

// I don't think we'll need a dynamic orchestration of services,
// so let's try without this enum but keeping it just in case

// #[derive(Debug, Error)]
// pub enum ServiceError {
//     #[error(transparent)]
//     Join(#[from] JoinError),
    
//     #[error(transparent)]
//     MockTransport(#[from] MockTransportError),
    
//     #[error(transparent)]
//     TagService(#[from] TagServiceError),

//     #[error(transparent)]
//     PeerTable(#[from] PeerTableError),

//     #[error(transparent)]
//     ClientService(#[from] ClientServiceError),

//     #[error(transparent)]
//     Channel(#[from] ChannelError),

//     #[error(transparent)]
//     DhtStorage(#[from] DhtStorageError),

//     #[error(transparent)]
//     DhtRouting(#[from] DhtRoutingError),

//     #[error(transparent)]
//     Node(#[from] NodeError),

//     #[error(transparent)]
//     NetSession(#[from] SessionManagerServiceError),
// }

// pub struct ServiceManager {
//     services: JoinSet<Result<(), ServiceError>>,
//     token: CancellationToken,
// }

// impl ServiceManager {
//     pub fn new() -> Self {
//         Self { services: JoinSet::new(), token: CancellationToken::new() }
//     }

//     pub fn spawn<S: Service + 'static + Send>(&mut self, service: S)
//     where ServiceError: From<<S as Service>::Error> {
//         let token = self.token.clone();
//         self.services.spawn(async move { service.run(token).await?; Ok(()) });
//     }

//     pub fn run(mut self) -> (JoinHandle<Result<(), ServiceError>>, CancellationToken) {
//         println!("Running all services");

//         let token = self.token.clone();

//         (tokio::spawn(async move {
//             let Some(res) = self.services.join_next().await else { return Ok(()); };

//             token.cancel();
//             let _ = self.services.join_all().await;

//             res?
//         }), self.token)
//     }
// }

pub trait Service {
    type Error;
    fn run(self, token: CancellationToken) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[macro_export]
macro_rules! commands {
    (
        $cmd:ident,
        $dispatcher:ident,
        $(
            $fn_name:ident
            => $name:ident {
                $( $field_name:ident : $field_type:ty ),* $(,)?
            }
            -> $out:ty
        ),* $(,)?
    ) => {
        pub enum $cmd {
            $( $name {
                $( $field_name:$field_type, )*
                reply_tx: tokio::sync::oneshot::Sender<$out>
            } ),*
        }

        #[derive(Clone)]
        pub struct $dispatcher {
            tx: tokio::sync::mpsc::Sender<$cmd>
        }

        impl $dispatcher {
            $(
                async fn $fn_name(&self, $( $field_name:$field_type ),*) -> Result<$out, $crate::channel::ChannelError> {
                    let (tx, rx) = $crate::channel::create_oneshot();

                    $crate::channel::send(
                        &self.tx,
                        $cmd::$name {
                            $( $field_name, )*
                            reply_tx: tx,
                        }
                    ).await?;

                    Ok(rx.await?)
                }
            ),*
        }
    };
}
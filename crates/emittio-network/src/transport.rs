use std::{net::{ToSocketAddrs}};
use thiserror::Error;
use crate::{types::{ConnId,Packet,Handshake, Frame}};

struct Connection<T: TripType> {
    id: ConnId,
    //... Other stuff pertinent to a connection
    state: std::marker::PhantomData<T>
}

enum RTT0 {} // Initial connection established. Send one Frame max
enum RTT1 {} // The only way to send additional data
enum Closed{} // Connectiong has been closed, no data can be sent, caller to re-establish

trait TripType {}
impl TripType for RTT0 {}
impl TripType for RTT1 {}
impl TripType for Closed{}


// An implementor of the Transport trait handles multiple connections that have
// been established with the node
trait Transport: Sized {
    type Error: Into<TransportError>;
    // Bind to the specified address and listen for incoming connection requests
    fn bind<T: ToSocketAddrs>(addr: T) -> impl Future<Output = Result<Self,Self::Error>> + Send;

    // Establish a connection, with optional data, starting a 0-RTT session
    // TODO the return shouldn't be the connection as that will be stored within the <Transport> implementer
    // TODO but is here for handy note that the connect method will open an RTT0
    // RTT0 should be able to upgrade itself asynchronously to an RTT1 by listening out for an incoming handshake?
    async fn connect<T: ToSocketAddrs>(&mut self, handshake: Handshake, data: Option<Frame>) -> impl Future<Output = Result<Connection<RTT0>,Self::Error>> + Send;

    // Send a Packet (Frame or Handshake) to a connection managed by this transport
    async fn send(&mut self, f: Packet, to: ConnId) -> Result<(),Self::Error>;

    // Receive any incoming Packets (Frame or Handshake) in a queue
    async fn receive(&mut self) -> impl Future<Output = Result<(ConnId,Packet), Self::Error>> + Send;

    // Close a specific connection
    async fn close(&mut self, id: ConnId) -> Result<(),Self::Error>;
}




#[derive(Debug,Error)]
pub enum TransportError {
    #[error("connection ID not found")]
    ConnectionNotFound,

    // TODO - Connection closed error

    // TODO - Serde Error

    // TODO - Net Error


}
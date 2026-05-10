use crate::net::NetClient;

pub trait TransportParticipant {
    fn net_client(&self) -> NetClient;
}
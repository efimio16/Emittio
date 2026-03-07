use thiserror::Error;


pub struct IPV4{
    addr: [u8; 4],
    port: u16
}

#[derive(Error, Debug)]
pub enum AddressParseError {
    #[error("Invalid address format")]
    InvalidAddress,
    #[error("Invalid port number")]
    InvalidPort,

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

implement IPV4 {
    pub fn from_string(s: &str) -> Result<Self, AddressParseError> {

    }
}
pub mod query;
pub mod types;
#[cfg(feature = "node")]
pub mod service;
pub mod error;

pub const DHT_SERVICE_ID: u16 = 2;
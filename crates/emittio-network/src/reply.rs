// TODO: implement `Reply` and `Replyable`

use serde::{Serialize, de::DeserializeOwned};

pub struct Reply {}
pub trait Replyable {}

impl<T: Serialize + DeserializeOwned> Replyable for T {}

impl Reply {
    pub fn parse<T: Replyable>(&self) -> Result<T, postcard::Error> {
        todo!()
    }
}
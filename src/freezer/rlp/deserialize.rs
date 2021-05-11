use super::Rlp;
use thiserror::Error;

pub struct RlpDeserializer<'a> {
    rlp: &'a Rlp,
}

impl<'a> RlpDeserializer<'a> {
    pub fn from_rlp(rlp: &'a Rlp) -> Self {
        RlpDeserializer { rlp }
    }
}

#[derive(Debug, Error)]
pub enum RlpDeserializeError {
    #[error("Failed deserialization")]
    RlpError1,
}

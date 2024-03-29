use crate::numeric::{u32_from_bytes_end_be_padded, u64_from_bytes_end_be_padded, NumericError};
use log::trace;
use serde::de::SeqAccess;
use serde::Deserializer;
use thiserror::Error;

mod parse;
use parse::{parse, Rlp};

/// A deserializer used to convert from RLP bytes into the different types
#[derive(Debug)]
pub(crate) struct RlpDeserializer<'de> {
    parsed: Vec<Rlp<'de>>,
    rest: &'de [u8],
}

impl<'de> RlpDeserializer<'de> {
    /// Create a new rlp deserializer from some byte slice
    pub(crate) fn new(bytes: &'de [u8]) -> Result<RlpDeserializer, RlpError> {
        trace!("Creating new rlp deserializer for {:?}", &bytes);
        let rlp_deserializer = RlpDeserializer {
            parsed: vec![],
            rest: bytes,
        };
        Ok(rlp_deserializer)
    }

    fn parse(&mut self) -> Result<(), RlpError> {
        trace!(
            "Parsing Rlp: Parsed: {:?}\n Unparsed: {:?}",
            self.parsed,
            self.rest
        );
        if let Some(Rlp::List(inner)) = self.parsed.last_mut() {
            let (parsed, slice) = parse(inner)?;
            *inner = slice;
            self.parsed.push(parsed);
            return Ok(());
        }
        let (parsed, slice) = parse(self.rest)?;
        self.parsed.push(parsed);
        self.rest = slice;
        Ok(())
    }
}

impl<'de: 'a, 'a> Deserializer<'de> for &'a mut RlpDeserializer<'de> {
    type Error = RlpError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parsed.last_mut() {
            Some(Rlp::Bytes(bytes)) => {
                let byte = bytes[0];
                *bytes = &bytes[1..];
                visitor.visit_u8(byte)
            }
            Some(empty @ Rlp::Empty) => {
                *empty = Rlp::Bytes(&[]);
                visitor.visit_u8(0)
            }
            _ => Err(RlpError::UnexpectedMatch),
        }
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parsed.last_mut() {
            Some(Rlp::Bytes(bytes)) => {
                let new_u32 = u32_from_bytes_end_be_padded(bytes).map_err(RlpError::Conversion)?;
                *bytes = &bytes[..bytes.len().saturating_sub(4)];
                visitor.visit_u32(new_u32)
            }
            Some(empty @ Rlp::Empty) => {
                *empty = Rlp::Bytes(&[]);
                visitor.visit_u32(0)
            }
            _ => Err(RlpError::UnexpectedMatch),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.parsed.last_mut() {
            Some(Rlp::Bytes(bytes)) => {
                let new_u64 = u64_from_bytes_end_be_padded(bytes).map_err(RlpError::Conversion)?;
                *bytes = &bytes[..bytes.len().saturating_sub(8)];
                visitor.visit_u64(new_u64)
            }
            Some(empty @ Rlp::Empty) => {
                *empty = Rlp::Bytes(&[]);
                visitor.visit_u64(0)
            }
            _ => Err(RlpError::UnexpectedMatch),
        }
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.parse()?;
        visitor.visit_seq(SeqAccessor {
            de: self,
            iterate: false,
        })
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccessor {
            de: self,
            iterate: false,
        })
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccessor {
            de: self,
            iterate: false,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.parse()?;
        visitor.visit_seq(SeqAccessor {
            de: self,
            iterate: true,
        })
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }
}

struct SeqAccessor<'a, 'de: 'a> {
    de: &'a mut RlpDeserializer<'de>,
    iterate: bool,
}

impl<'de: 'a, 'a> SeqAccess<'de> for SeqAccessor<'a, 'de> {
    type Error = RlpError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(Rlp::List(&[])) = self.de.parsed.last() {
            self.de.parsed.pop().ok_or(RlpError::NoInputLeft)?;
        }
        if let Some(Rlp::List(&[]) | Rlp::Bytes(&[]) | Rlp::EmptyList) = self.de.parsed.last() {
            self.de.parsed.pop().ok_or(RlpError::NoInputLeft)?;
            if !self.iterate {
                return Ok(None);
            }
        }
        if self.iterate {
            self.de.parse()?;
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

/// Enum for collecting RLP errors
#[derive(Debug, Error)]
pub enum RlpError {
    #[error("No match found while parsing rlp slice")]
    NoMatch,
    #[error("No input left to parse")]
    NoInputLeft,
    #[error("Unexpected match")]
    UnexpectedMatch,
    #[error("Type conversion error: {0}")]
    Conversion(#[source] NumericError),
    #[error("Error during RLP deserialization: {0}")]
    CustomError(String),
}

impl serde::de::Error for RlpError {
    fn custom<T>(msg: T) -> RlpError
    where
        T: std::fmt::Display,
    {
        RlpError::CustomError(msg.to_string())
    }
}

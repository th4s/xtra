use crate::helper::{u32_from_bytes_end_be, u64_from_bytes_end_be};
use serde::de::SeqAccess;
use serde::Deserializer;

mod parse;
use parse::{parse, Rlp, RlpError};

#[derive(Debug)]
pub struct RlpDeserializer<'de> {
    rlp_stack: Vec<Rlp<'de>>,
    slice: &'de [u8],
}

struct SeqAccessor<'a, 'de: 'a> {
    de: &'a mut RlpDeserializer<'de>,
    dynamic: bool,
}

impl<'de: 'a, 'a> SeqAccess<'de> for SeqAccessor<'a, 'de> {
    type Error = RlpError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        println!("\nnext_element_seed: {:?}", self.de.rlp_stack);
        println!("seed type {}", std::any::type_name::<T>());
        if self.de.last_element_len()? == 0 {
            self.de.pop_stack()?;
            if self.dynamic {
                println!("{:?}", self.de.rlp_stack);
                println!("RETURNED NONE IN SEQUENCE");
                return Ok(None);
            }
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de> RlpDeserializer<'de> {
    pub fn new(bytes: &'de [u8]) -> Result<RlpDeserializer, RlpError> {
        let rlp_deserializer = RlpDeserializer {
            rlp_stack: vec![],
            slice: bytes,
        };
        Ok(rlp_deserializer)
    }

    fn next(&mut self) -> Result<(), RlpError> {
        if let Some(last_element) = self.rlp_stack.last_mut() {
            if let Rlp::List(inner) = last_element {
                let (parsed, slice) = parse(inner)?;
                *inner = &slice;
                self.rlp_stack.push(parsed);
                return Ok(());
            }
        }
        let (parsed, slice) = parse(self.slice)?;
        self.rlp_stack.push(parsed);
        self.slice = slice;
        Ok(())
    }

    fn pop_stack(&mut self) -> Result<Rlp, RlpError> {
        self.rlp_stack.pop().ok_or(RlpError::NoInputLeft)
    }

    fn last_element_len(&self) -> Result<usize, RlpError> {
        if let Some(last) = self.rlp_stack.last() {
            return match last {
                Rlp::Bytes(inner) => Ok(inner.len()),
                Rlp::List(inner) => Ok(inner.len()),
                Rlp::Empty => Ok(1),
                Rlp::EmptyList => Ok(1),
            };
        }
        Err(RlpError::NoSizeHint)
    }
}

impl<'de: 'a, 'a> Deserializer<'de> for &'a mut RlpDeserializer<'de> {
    type Error = RlpError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("deserialize_u8");
        match self.rlp_stack.last_mut() {
            Some(Rlp::Empty) => visitor.visit_u8(0),
            Some(Rlp::Bytes(bytes)) => {
                // println!("bytes {:?}", bytes);
                let byte = bytes[0];
                *bytes = &bytes[1..];
                visitor.visit_u8(byte)
            }
            _ => Err(RlpError::UnexpectedMatch),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("deserialize_u32");
        if let Some(Rlp::Bytes(bytes)) = self.rlp_stack.last_mut() {
            let new_u32 = u32_from_bytes_end_be(&bytes).map_err(RlpError::Conversion)?;
            *bytes = &bytes[..bytes.len().checked_sub(4).unwrap_or(0)];
            return visitor.visit_u32(new_u32);
        }
        if let Some(empty @ Rlp::Empty) = self.rlp_stack.last_mut() {
            *empty = Rlp::Bytes(&[]);
            return visitor.visit_u32(0);
        }
        Err(RlpError::UnexpectedMatch)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("deserialize_u64");
        self.next()?;
        if let Some(Rlp::Bytes(bytes)) = self.rlp_stack.last_mut() {
            let new_u64 = u64_from_bytes_end_be(&bytes).map_err(RlpError::Conversion)?;
            *bytes = &bytes[..bytes.len().checked_sub(8).unwrap_or(0)];
            return visitor.visit_u64(new_u64);
        }
        if let Some(empty @ Rlp::Empty) = self.rlp_stack.last_mut() {
            *empty = Rlp::Bytes(&[]);
            return visitor.visit_u64(0);
        }
        Err(RlpError::UnexpectedMatch)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("deserialize_seq");
        self.next()?;
        visitor.visit_seq(SeqAccessor {
            de: self,
            dynamic: true,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("deserialize_tuple with length {}", len);
        self.next()?;
        visitor.visit_seq(SeqAccessor {
            de: self,
            dynamic: false,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("deserialize_struct {}", name);
        self.next()?;
        visitor.visit_seq(SeqAccessor {
            de: self,
            dynamic: false,
        })
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
    }
}

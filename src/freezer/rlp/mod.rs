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
    len: Option<usize>,
}

impl<'de: 'a, 'a> SeqAccess<'de> for SeqAccessor<'a, 'de> {
    type Error = RlpError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let None = self.size_hint() {
            self.len = self.de.last_element_len().map(Some)?;
        }

        if let Some(len) = self.size_hint() {
            if len > 0 {
                println!(
                    "BEFORE NEXT IN ITERATOR length: {:?}, stack: {:?}",
                    &self.size_hint(),
                    &self.de.rlp_stack
                );
                self.len = Some(len - 1);
                self.de.next()?;
            } else {
                println!("RETURNED NONE IN SEQUENCE");
                return Ok(None);
            }
        }
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        self.len
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
                if slice.is_empty() {
                    self.rlp_stack.pop();
                } else {
                    *last_element = Rlp::List(slice);
                }
                self.rlp_stack.push(parsed);
                return Ok(());
            }

            if let Rlp::Bytes(inner) = last_element {
                let (first, rest) = inner.split_first().ok_or(RlpError::NoInputLeft)?;
                if rest.is_empty() {
                    self.rlp_stack.pop();
                } else {
                    *last_element = Rlp::Bytes(rest);
                }
                self.rlp_stack.push(Rlp::Byte(first));
                return Ok(());
            }
        }
        let (parsed, slice) = parse(self.slice)?;
        self.rlp_stack.push(parsed);
        self.slice = slice;
        Ok(())
    }

    fn eat(&mut self) -> Result<Rlp, RlpError> {
        self.rlp_stack.pop().ok_or(RlpError::NoInputLeft)
    }

    fn last_element_len(&self) -> Result<usize, RlpError> {
        if let Some(last) = self.rlp_stack.last() {
            return match last {
                Rlp::Bytes(inner) => Ok(inner.len()),
                Rlp::List(inner) => Ok(inner.len()),
                _ => Err(RlpError::NoSizeHint),
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
        if let Some(Rlp::Bytes(_)) = self.rlp_stack.last() {
            self.next()?;
        }
        match self.eat()? {
            Rlp::Byte(byte) => visitor.visit_u8(*byte),
            Rlp::Empty => visitor.visit_u8(0),
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
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!()
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
        println!("SEQUENCE WITH UNKNOWN LENGTH");
        visitor.visit_seq(SeqAccessor {
            de: self,
            len: None,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("TUPLE WITH LENGTH {}", len);
        visitor.visit_seq(SeqAccessor {
            de: self,
            len: Some(len),
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
        self.next()?;
        println!("STRUCT COMING");
        visitor.visit_seq(SeqAccessor {
            de: self,
            len: Some(fields.len()),
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

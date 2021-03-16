use std::convert::TryInto;
use thiserror::Error;

pub enum Rlp<'a> {
    Bytes(&'a [u8]),
    List(&'a Rlp<'a>),
}

pub fn decode(rlp_slice: &[u8]) -> Result<(&[u8], i32), RlpError> {
    let input_len = rlp_slice.len() as usize;
    if input_len == 0 {
        return Err(RlpError::InvalidLength);
    }
    let prefix = rlp_slice[0] as usize;
    Ok(
        // This is a single byte
        if prefix <= 0x7f {
            (&rlp_slice[..1], -1)
            // A short string
        } else if prefix <= 0xb7 && input_len > prefix - 0x80 {
            (&rlp_slice[1..prefix - 0x80], -1)
            // A long string
        } else if prefix <= 0xbf
            && input_len > prefix - 0xb7
            && input_len > prefix - 0xb7 + usize_from_u8(&rlp_slice[1..prefix - 0xb7])?
        {
            (
                &rlp_slice[prefix - 0xb6..usize_from_u8(&rlp_slice[1..prefix - 0xb7])?],
                -1,
            )
            // A short list
        } else if prefix <= 0xf7 && input_len > prefix - 0xc0 {
            (&rlp_slice[1..prefix - 0xc0], (prefix - 0xbf) as i32)
            // A long list
        } else if prefix <= 0xff
            && input_len > prefix - 0xf7
            && input_len > prefix - 0xf7 + usize_from_u8(&rlp_slice[1..prefix - 0xf7])?
        {
            (
                &rlp_slice[prefix - 0xf6..usize_from_u8(&rlp_slice[1..prefix - 0xf7])?],
                (prefix - 0xf6 + usize_from_u8(&rlp_slice[1..prefix - 0xf7])?) as i32,
            )
        } else {
            return Err(RlpError::Decoding);
        },
    )
}

fn usize_from_u8(input: &[u8]) -> Result<usize, RlpError> {
    Ok(usize::from_be_bytes(
        input.try_into().map_err(|_err| RlpError::Decoding)?,
    ))
}

#[derive(Debug, Error)]
pub enum RlpError {
    #[error("Invalid Length")]
    InvalidLength,
    #[error("Error while decoding")]
    Decoding,
}

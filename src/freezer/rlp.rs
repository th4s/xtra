use thiserror::Error;

#[derive(Debug)]
pub enum Rlp {
    Bytes(Vec<u8>),
    List(Vec<Rlp>),
    Byte(u8),
    EmptyList,
    EmptyString,
}

pub fn decode(rlp_slice: &[u8]) -> Result<Vec<Rlp>, RlpError> {
    let mut out: Vec<Rlp> = Vec::new();
    let (mut len, mut slice) = (rlp_slice.len(), rlp_slice);
    while len > 0 {
        let matched = match_rlp(slice)?;
        out.push(matched.0);
        slice = matched.1;
        len = matched.1.len();
    }
    Ok(out)
}

fn usize_from_u8(input: &[u8]) -> usize {
    let len = input.len();
    let out = input.iter().enumerate().fold(0_usize, |acc, (i, el)| {
        acc + 256_usize.pow((len - 1 - i) as u32) * (*el as usize)
    });
    out
}

fn match_rlp(rlp_slice: &[u8]) -> Result<(Rlp, &[u8]), RlpError> {
    let len = rlp_slice.len();
    if let (Some(rlp), slice) = match_empty(rlp_slice)? {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_byte(rlp_slice)? {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_short_str(rlp_slice, len)? {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_long_str(rlp_slice, len)? {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_short_list(rlp_slice, len)? {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_long_list(rlp_slice, len)? {
        return Ok((rlp, slice));
    }
    Err(RlpError::NoMatch)
}

fn match_empty(rlp_slice: &[u8]) -> Result<(Option<Rlp>, &[u8]), RlpError> {
    Ok(if rlp_slice[0] == 0xc0 {
        (Some(Rlp::EmptyList), &rlp_slice[1..])
    } else if rlp_slice[0] == 0x80 {
        (Some(Rlp::EmptyString), &rlp_slice[1..])
    } else {
        (None, rlp_slice)
    })
}

fn match_byte(rlp_slice: &[u8]) -> Result<(Option<Rlp>, &[u8]), RlpError> {
    Ok(if rlp_slice[0] <= 0x7f {
        (Some(Rlp::Byte(rlp_slice[0])), &rlp_slice[1..])
    } else {
        (None, rlp_slice)
    })
}

fn match_short_str(rlp_slice: &[u8], len: usize) -> Result<(Option<Rlp>, &[u8]), RlpError> {
    Ok(
        if rlp_slice[0] <= 0xb7 && len > (rlp_slice[0] - 0x80) as usize {
            (
                Some(Rlp::Bytes(
                    rlp_slice[1..(rlp_slice[0] - 0x7f) as usize].to_vec(),
                )),
                &rlp_slice[(rlp_slice[0] - 0x7f) as usize..],
            )
        } else {
            (None, rlp_slice)
        },
    )
}

fn match_long_str(rlp_slice: &[u8], len: usize) -> Result<(Option<Rlp>, &[u8]), RlpError> {
    Ok(
        if rlp_slice[0] <= 0xbf
            && len > (rlp_slice[0] - 0xb7) as usize
            && len
                > (rlp_slice[0] - 0xb7) as usize
                    + usize_from_u8(&rlp_slice[1..(rlp_slice[0] - 0xb6) as usize])
        {
            (
                Some(Rlp::Bytes(
                    rlp_slice[(rlp_slice[0] - 0xb6) as usize
                        ..usize_from_u8(&rlp_slice[1..(rlp_slice[0] - 0xb6) as usize])
                            + (rlp_slice[0] - 0xb6) as usize]
                        .to_vec(),
                )),
                &rlp_slice[usize_from_u8(&rlp_slice[1..(rlp_slice[0] - 0xb6) as usize])
                    + (rlp_slice[0] - 0xb6) as usize..],
            )
        } else {
            (None, rlp_slice)
        },
    )
}

fn match_short_list(rlp_slice: &[u8], len: usize) -> Result<(Option<Rlp>, &[u8]), RlpError> {
    Ok(
        if rlp_slice[0] <= 0xf7 && len > (rlp_slice[0] - 0xc0) as usize {
            (
                Some(Rlp::List(decode(
                    &rlp_slice[1..(rlp_slice[0] - 0xbf) as usize],
                )?)),
                &rlp_slice[(rlp_slice[0] - 0xbf) as usize..],
            )
        } else {
            (None, rlp_slice)
        },
    )
}

fn match_long_list(rlp_slice: &[u8], len: usize) -> Result<(Option<Rlp>, &[u8]), RlpError> {
    Ok(
        if len > (rlp_slice[0] - 0xf7) as usize
            && len
                > (rlp_slice[0] - 0xf7) as usize
                    + usize_from_u8(&rlp_slice[1..(rlp_slice[0] - 0xf6) as usize])
        {
            (
                Some(Rlp::List(decode(
                    &rlp_slice[(rlp_slice[0] - 0xf6) as usize
                        ..usize_from_u8(&rlp_slice[1..(rlp_slice[0] - 0xf6) as usize])
                            + (rlp_slice[0] - 0xf6) as usize],
                )?)),
                &rlp_slice[usize_from_u8(&rlp_slice[1..(rlp_slice[0] - 0xf6) as usize])
                    + (rlp_slice[0] - 0xf6) as usize..],
            )
        } else {
            (None, rlp_slice)
        },
    )
}

#[derive(Debug, Error)]
pub enum RlpError {
    #[error("No match found while parsing rlp slice")]
    NoMatch,
}

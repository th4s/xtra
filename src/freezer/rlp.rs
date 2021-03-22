use log::trace;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Rlp {
    Bytes(Vec<u8>),
    List(Vec<Rlp>),
    Byte(u8),
    EmptyList,
    EmptyString,
}

pub fn decode(rlp_slice: &[u8]) -> Result<Vec<Rlp>, RlpError> {
    trace!("RLP-decoding {:?}", rlp_slice);
    let mut out: Vec<Rlp> = Vec::new();
    let (mut len, mut slice) = (rlp_slice.len(), rlp_slice);
    while len > 0 {
        let matched = match_rlp(slice)?;
        out.push(matched.0);
        slice = matched.1;
        len = matched.1.len();
    }
    trace!("Decoded: {:?}", out);
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
    if let (Some(rlp), slice) = match_empty(rlp_slice) {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_byte(rlp_slice) {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_short_str(rlp_slice, len) {
        return Ok((rlp, slice));
    }
    if let (Some(rlp), slice) = match_long_str(rlp_slice, len) {
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

fn match_empty(rlp_slice: &[u8]) -> (Option<Rlp>, &[u8]) {
    if rlp_slice[0] == 0xc0 {
        (Some(Rlp::EmptyList), &rlp_slice[1..])
    } else if rlp_slice[0] == 0x80 {
        (Some(Rlp::EmptyString), &rlp_slice[1..])
    } else {
        (None, rlp_slice)
    }
}

fn match_byte(rlp_slice: &[u8]) -> (Option<Rlp>, &[u8]) {
    if rlp_slice[0] <= 0x7f {
        (Some(Rlp::Byte(rlp_slice[0])), &rlp_slice[1..])
    } else {
        (None, rlp_slice)
    }
}

fn match_short_str(rlp_slice: &[u8], len: usize) -> (Option<Rlp>, &[u8]) {
    if rlp_slice[0] <= 0xb7 && len > (rlp_slice[0] - 0x80) as usize {
        (
            Some(Rlp::Bytes(
                rlp_slice[1..(rlp_slice[0] - 0x7f) as usize].to_vec(),
            )),
            &rlp_slice[(rlp_slice[0] - 0x7f) as usize..],
        )
    } else {
        (None, rlp_slice)
    }
}

fn match_long_str(rlp_slice: &[u8], len: usize) -> (Option<Rlp>, &[u8]) {
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
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rlp_decode() {
        // This is the body of block number 4
        let vec: Vec<u8> = vec![
            249, 2, 31, 192, 249, 2, 27, 249, 2, 24, 160, 212, 229, 103, 64, 248, 118, 174, 248,
            192, 16, 184, 106, 64, 213, 245, 103, 69, 161, 24, 208, 144, 106, 52, 230, 154, 236,
            140, 13, 177, 203, 143, 163, 160, 29, 204, 77, 232, 222, 199, 93, 122, 171, 133, 181,
            103, 182, 204, 212, 26, 211, 18, 69, 27, 148, 138, 116, 19, 240, 161, 66, 253, 64, 212,
            147, 71, 148, 80, 136, 214, 35, 186, 15, 207, 1, 49, 224, 137, 122, 145, 115, 74, 77,
            131, 89, 106, 160, 160, 154, 101, 151, 178, 106, 220, 14, 89, 21, 207, 204, 165, 55,
            186, 73, 58, 100, 124, 173, 28, 60, 146, 61, 64, 108, 222, 198, 202, 73, 160, 160, 109,
            160, 86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72,
            224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33, 160, 86, 232, 31, 23,
            27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108,
            173, 192, 1, 98, 47, 181, 227, 99, 180, 33, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 133, 3, 255, 128, 0, 0, 1, 130, 19, 136, 128,
            132, 85, 186, 66, 55, 160, 71, 101, 116, 104, 47, 118, 49, 46, 48, 46, 48, 45, 102, 99,
            55, 57, 100, 51, 50, 100, 47, 108, 105, 110, 117, 120, 47, 103, 111, 49, 46, 52, 160,
            208, 69, 184, 82, 119, 1, 96, 218, 22, 158, 199, 147, 236, 12, 110, 111, 245, 98, 228,
            115, 178, 191, 63, 129, 146, 220, 89, 132, 46, 54, 247, 84, 136, 219, 130, 26, 119, 91,
            249, 218, 206,
        ];
        let _rlp = decode(&vec[..]).unwrap();
    }

    #[test]
    fn test_rlp_u8_from_usize() {
        let first = vec![0_u8];
        let second = vec![0xff_u8];
        let third = vec![0x2a_u8, 0xac_u8];

        assert_eq!(usize_from_u8(&first[..]), 0_usize);
        assert_eq!(usize_from_u8(&second[..]), 255_usize);
        assert_eq!(usize_from_u8(&third[..]), 10924_usize);
    }

    #[test]
    fn test_rlp_match_empty() {
        let first = vec![0xc0_u8];
        let second = vec![0x80_u8];
        let third = vec![0x02_u8];

        assert_eq!(match_empty(&first[..]), (Some(Rlp::EmptyList), &first[1..]));
        assert_eq!(
            match_empty(&second[..]),
            (Some(Rlp::EmptyString), &second[1..])
        );
        assert_eq!(match_empty(&third[..]), (None, &third[..]));
    }

    #[test]
    fn test_rlp_match_byte() {
        let first = vec![0x1b_u8];
        let second = vec![0x80_u8];

        assert_eq!(match_byte(&first[..]), (Some(Rlp::Byte(0x1b)), &first[1..]));
        assert_eq!(match_byte(&second[..]), (None, &second[..]));
    }

    #[test]
    fn test_rlp_match_short_str() {
        let vec = vec![0x83, b'c', b'a', b't', b'X'];

        assert_eq!(
            match_short_str(&vec[..], vec.len()),
            (Some(Rlp::Bytes(vec![b'c', b'a', b't'])), &vec[4..])
        );
    }

    #[test]
    fn test_rlp_match_long_str() {
        let mut vec = vec![0xb8_u8, 0xaa_u8];
        vec.extend(std::iter::repeat(b'a').take(170));
        vec.push(b'X');

        assert_eq!(
            match_long_str(&vec[..], vec.len()),
            (Some(Rlp::Bytes(vec[2..172].to_vec())), &vec[172..])
        );
    }

    #[test]
    fn test_rlp_match_short_list() {
        let vec = vec![0xc2_u8, 0xc0_u8, 0xc0_u8];

        assert_eq!(
            match_short_list(&vec[..], vec.len()).unwrap(),
            (
                Some(Rlp::List(vec![Rlp::EmptyList, Rlp::EmptyList])),
                &vec[3..]
            )
        );
    }

    #[test]
    fn test_rlp_match_long_list() {
        let mut vec = vec![0xf8_u8, 0xaa_u8];
        vec.extend(std::iter::repeat(0xc0).take(170));
        vec.push(b'X');

        assert_eq!(
            match_long_list(&vec[..], vec.len()).unwrap(),
            (Some(Rlp::List(vec![Rlp::EmptyList; 170])), &vec[172..])
        );
    }
}

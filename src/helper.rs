use std::array::TryFromSliceError;
use std::convert::TryFrom;
use thiserror::Error;

/// Converts the first n big endian bytes of a slice into usize
/// Uses padding if necessary
pub fn usize_from_bytes_be(input: &[u8]) -> Result<usize, NumericError> {
    const SIZE_OF_USIZE: usize = std::mem::size_of::<usize>();
    let input_len = input.len();
    let input = match input_len {
        len @ 0..=SIZE_OF_USIZE => {
            let mut padded = [0_u8; SIZE_OF_USIZE];
            padded[SIZE_OF_USIZE - len..].copy_from_slice(&input);
            padded
        }
        _ => <[u8; SIZE_OF_USIZE]>::try_from(&input[..SIZE_OF_USIZE])
            .map_err(NumericError::Conversion)?,
    };
    let out = usize::from_be_bytes(input);
    Ok(out)
}

/// Converts the last 4 bytes of a slice from left to right into a u32
/// Uses padding if necessary
pub fn u32_from_bytes_end_be(input: &[u8]) -> Result<u32, NumericError> {
    let input_len = input.len();
    let input = match input_len {
        len @ 0..=4 => {
            let mut padded = [0_u8; 4];
            padded[4 - len..].copy_from_slice(&input);
            padded
        }
        _ => <[u8; 4]>::try_from(&input[input_len - 4..]).map_err(NumericError::Conversion)?,
    };
    let out = u32::from_be_bytes(input);
    Ok(out)
}

/// Converts the last 8 bytes of a slice from left to right into a u64
/// Uses padding if necessary
pub fn u64_from_bytes_end_be(input: &[u8]) -> Result<u64, NumericError> {
    let input_len = input.len();
    let input = match input_len {
        len @ 0..=8 => {
            let mut padded = [0_u8; 8];
            padded[8 - len..].copy_from_slice(&input);
            padded
        }
        _ => <[u8; 8]>::try_from(&input[input_len - 8..]).map_err(NumericError::Conversion)?,
    };
    let out = u64::from_be_bytes(input);
    Ok(out)
}

/// Error for numeric conversions
#[derive(Debug, Error)]
pub enum NumericError {
    #[error("Error during numeric conversion: {0}")]
    Conversion(#[source] TryFromSliceError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_usize_from_bytes_be() {
        let first = vec![0_u8];
        let second = vec![0xff_u8];
        let third = vec![
            0x2a_u8, 0xac_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8,
        ];

        assert_eq!(usize_from_bytes_be(&first[..]).unwrap(), 0_usize);
        assert_eq!(usize_from_bytes_be(&second[..]).unwrap(), 255_usize);
        assert_eq!(
            usize_from_bytes_be(&third[..]).unwrap(),
            3075114120563916799_usize
        );
    }

    #[test]
    fn test_helper_u32_from_bytes_end_be() {
        let first = vec![0_u8];
        let second = vec![0xff_u8];
        let third = vec![0x2a_u8, 0xac_u8, 0xff_u8, 0xff_u8, 0xff_u8];

        assert_eq!(u32_from_bytes_end_be(&first[..]).unwrap(), 0_u32);
        assert_eq!(u32_from_bytes_end_be(&second[..]).unwrap(), 255_u32);
        assert_eq!(u32_from_bytes_end_be(&third[..]).unwrap(), 2902458367_u32);
    }

    #[test]
    fn test_helper_u64_from_bytes_end_be() {
        let first = vec![0_u8];
        let second = vec![0xff_u8];
        let third = vec![
            0x2a_u8, 0xac_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8,
        ];

        assert_eq!(u64_from_bytes_end_be(&first[..]).unwrap(), 0_u64);
        assert_eq!(u64_from_bytes_end_be(&second[..]).unwrap(), 255_u64);
        assert_eq!(u64_from_bytes_end_be(&third[..]).unwrap(), 2902458367_u64);
    }
}

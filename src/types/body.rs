use super::{BlockHeader, ByteArray, ByteVec, NiceBigUint, NiceVec};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The body of an Ethereum block
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BlockBody {
    pub transactions: NiceVec<Transaction>,
    pub uncles: NiceVec<BlockHeader>,
}

impl std::fmt::Display for BlockBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

/// The transaction object
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(serialize_with = "crate::types::str_serialize")]
    pub nonce: u64,
    pub gas_price: NiceBigUint,
    #[serde(serialize_with = "crate::types::str_serialize")]
    pub gas: u64,
    #[serde(deserialize_with = "deserialize_transaction")]
    pub to: To,
    pub value: NiceBigUint,
    pub data: ByteVec,
    #[serde(serialize_with = "crate::types::str_serialize")]
    v: u8,
    #[serde(deserialize_with = "deserialize_signature")]
    r: ByteArray<32>,
    #[serde(deserialize_with = "deserialize_signature")]
    s: ByteArray<32>,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

fn deserialize_transaction<'de, D: Deserializer<'de>>(deserializer: D) -> Result<To, D::Error> {
    let buf = Vec::<u8>::deserialize(deserializer)?;
    if buf.len() == 1 {
        return Ok(To::ContractCreation(ByteArray::<1>([0_u8])));
    }
    let mut out: [u8; 20] = [0; 20];
    out.copy_from_slice(&buf);
    Ok(To::Address(ByteArray::<20>(out)))
}

fn deserialize_signature<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ByteArray<32>, D::Error> {
    let buf = Vec::<u8>::deserialize(deserializer)?;
    if buf.len() < 32 {
        let mut signature: Vec<u8> = vec![0; 32 - buf.len()];
        signature.extend_from_slice(&buf);
        return Ok(ByteArray::<32>(
            <[u8; 32]>::try_from(signature).expect("Should be impossible."),
        ));
    }
    Ok(ByteArray::<32>(
        <[u8; 32]>::try_from(buf).expect("Should be impossible."),
    ))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum To {
    Address(ByteArray<20>),
    ContractCreation(ByteArray<1>),
}

impl std::fmt::Display for To {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            To::Address(bytes) => write!(f, "{}", bytes),
            To::ContractCreation(bytes) => write!(f, "{}", bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlp::RlpDeserializer;
    use num_bigint::BigUint;

    #[test]
    fn test_body_deserialize() {
        // This is the body of block number 46147. It contains the first mainnet transaction.
        let body_input: Vec<u8> = vec![
            0xf8, 0x6c, 0xf8, 0x69, 0xf8, 0x67, 0x80, 0x86, 0x2d, 0x79, 0x88, 0x3d, 0x20, 0x00,
            0x82, 0x52, 0x08, 0x94, 0x5d, 0xf9, 0xb8, 0x79, 0x91, 0x26, 0x2f, 0x6b, 0xa4, 0x71,
            0xf0, 0x97, 0x58, 0xcd, 0xe1, 0xc0, 0xfc, 0x1d, 0xe7, 0x34, 0x82, 0x7a, 0x69, 0x80,
            0x1c, 0xa0, 0x88, 0xff, 0x6c, 0xf0, 0xfe, 0xfd, 0x94, 0xdb, 0x46, 0x11, 0x11, 0x49,
            0xae, 0x4b, 0xfc, 0x17, 0x9e, 0x9b, 0x94, 0x72, 0x1f, 0xff, 0xd8, 0x21, 0xd3, 0x8d,
            0x16, 0x46, 0x4b, 0x3f, 0x71, 0xd0, 0xa0, 0x45, 0xe0, 0xaf, 0xf8, 0x00, 0x96, 0x1c,
            0xfc, 0xe8, 0x05, 0xda, 0xef, 0x70, 0x16, 0xb9, 0xb6, 0x75, 0xc1, 0x37, 0xa6, 0xa4,
            0x1a, 0x54, 0x8f, 0x7b, 0x60, 0xa3, 0x48, 0x4c, 0x06, 0xa3, 0x3a, 0xc0,
        ];
        let mut body_deserializer = RlpDeserializer::new(&body_input).unwrap();
        let body = BlockBody::deserialize(&mut body_deserializer).unwrap();

        let body_expected = BlockBody {
            transactions: NiceVec(vec![Transaction {
                nonce: 0,
                gas_price: NiceBigUint(BigUint::from(50000000000000_u64)),
                gas: 21000,
                to: To::Address(ByteArray::<20>([
                    0x5d, 0xf9, 0xb8, 0x79, 0x91, 0x26, 0x2f, 0x6b, 0xa4, 0x71, 0xf0, 0x97, 0x58,
                    0xcd, 0xe1, 0xc0, 0xfc, 0x1d, 0xe7, 0x34,
                ])),
                value: NiceBigUint(BigUint::from(31337_u32)),
                data: ByteVec(vec![0_u8]),
                v: 28_u8,
                r: ByteArray::<32>([
                    0x88, 0xff, 0x6c, 0xf0, 0xfe, 0xfd, 0x94, 0xdb, 0x46, 0x11, 0x11, 0x49, 0xae,
                    0x4b, 0xfc, 0x17, 0x9e, 0x9b, 0x94, 0x72, 0x1f, 0xff, 0xd8, 0x21, 0xd3, 0x8d,
                    0x16, 0x46, 0x4b, 0x3f, 0x71, 0xd0,
                ]),
                s: ByteArray::<32>([
                    0x45, 0xe0, 0xaf, 0xf8, 0x00, 0x96, 0x1c, 0xfc, 0xe8, 0x05, 0xda, 0xef, 0x70,
                    0x16, 0xb9, 0xb6, 0x75, 0xc1, 0x37, 0xa6, 0xa4, 0x1a, 0x54, 0x8f, 0x7b, 0x60,
                    0xa3, 0x48, 0x4c, 0x06, 0xa3, 0x3a,
                ]),
            }]),
            uncles: NiceVec(vec![]),
        };
        assert_eq!(body, body_expected);
    }
}

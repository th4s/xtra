use super::{ByteArray, ByteVec, Header, NiceVec};
use num_bigint::BigUint;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Body {
    pub transactions: NiceVec<Transaction>,
    pub uncles: NiceVec<Header>,
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\ttransactions: {}, \n\tuncles: {}\n}}",
            self.transactions, self.uncles
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Transaction {
    pub nonce: u64,
    pub gas_price: BigUint,
    pub gas: u64,
    pub to: ByteArray<20>,
    pub value: BigUint,
    pub data: ByteVec,
    v: u8,
    r: ByteArray<32>,
    s: ByteArray<32>,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\tnonce: {},\n\tgas_price: {}\n\tgas: {},\n\tto: {},\n\t\
            value: {},\n\tdata: {},\n\tv: {},\n\tr: {},\n\ts: {}\n}}",
            self.nonce,
            self.gas_price,
            self.gas,
            self.to,
            self.value,
            self.data,
            self.v,
            self.r,
            self.s
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::freezer::rlp::RlpDeserializer;

    #[test]
    fn test_body_deserialize() {
        // This is the body of block number 46147. It contains the first mainnet transaction.
        let test_body: Vec<u8> = vec![
            0xf8, 0x6c, 0xf8, 0x69, 0xf8, 0x67, 0x80, 0x86, 0x2d, 0x79, 0x88, 0x3d, 0x20, 0x00,
            0x82, 0x52, 0x08, 0x94, 0x5d, 0xf9, 0xb8, 0x79, 0x91, 0x26, 0x2f, 0x6b, 0xa4, 0x71,
            0xf0, 0x97, 0x58, 0xcd, 0xe1, 0xc0, 0xfc, 0x1d, 0xe7, 0x34, 0x82, 0x7a, 0x69, 0x80,
            0x1c, 0xa0, 0x88, 0xff, 0x6c, 0xf0, 0xfe, 0xfd, 0x94, 0xdb, 0x46, 0x11, 0x11, 0x49,
            0xae, 0x4b, 0xfc, 0x17, 0x9e, 0x9b, 0x94, 0x72, 0x1f, 0xff, 0xd8, 0x21, 0xd3, 0x8d,
            0x16, 0x46, 0x4b, 0x3f, 0x71, 0xd0, 0xa0, 0x45, 0xe0, 0xaf, 0xf8, 0x00, 0x96, 0x1c,
            0xfc, 0xe8, 0x05, 0xda, 0xef, 0x70, 0x16, 0xb9, 0xb6, 0x75, 0xc1, 0x37, 0xa6, 0xa4,
            0x1a, 0x54, 0x8f, 0x7b, 0x60, 0xa3, 0x48, 0x4c, 0x06, 0xa3, 0x3a, 0xc0,
        ];
        let mut body_deserializer = RlpDeserializer::new(&test_body).unwrap();
        let _body = Body::deserialize(&mut body_deserializer).unwrap();
        println!("{}", _body);
    }
}

use super::{ByteArray, ByteVec, NiceVec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionReceipt {
    post_state: ByteArray<32>,
    #[serde(serialize_with = "crate::types::str_serialize")]
    cum_gas_used: u64,
    logs: NiceVec<Log>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Log {
    address: ByteArray<20>,
    topics: NiceVec<ByteArray<64>>,
    data: ByteVec,
    #[serde(serialize_with = "crate::types::str_serialize")]
    block_number: u64,
    tx_hash: ByteArray<32>,
    #[serde(serialize_with = "crate::types::str_serialize")]
    tx_index: u64,
    block_hash: ByteArray<32>,
    #[serde(serialize_with = "crate::types::str_serialize")]
    log_index: u64,
    removed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::freezer::rlp::RlpDeserializer;

    #[test]
    fn test_receipt_deserialize() {
        // This is the receipt of the first transaction on mainnet in block 46147
        let receipt_input: Vec<u8> = vec![
            0xe6, 0xe5, 0xa0, 0x96, 0xa8, 0xe0, 0x09, 0xd2, 0xb8, 0x8b, 0x14, 0x83, 0xe6, 0x94,
            0x1e, 0x68, 0x12, 0xe3, 0x22, 0x63, 0xb0, 0x56, 0x83, 0xfa, 0xc2, 0x02, 0xab, 0xc6,
            0x22, 0xa3, 0xe3, 0x1a, 0xed, 0x19, 0x57, 0x82, 0x52, 0x08, 0xc0,
        ];

        let receipt_expected: TransactionReceipt = TransactionReceipt {
            post_state: ByteArray::<32>([
                0x96, 0xa8, 0xe0, 0x09, 0xd2, 0xb8, 0x8b, 0x14, 0x83, 0xe6, 0x94, 0x1e, 0x68, 0x12,
                0xe3, 0x22, 0x63, 0xb0, 0x56, 0x83, 0xfa, 0xc2, 0x02, 0xab, 0xc6, 0x22, 0xa3, 0xe3,
                0x1a, 0xed, 0x19, 0x57,
            ]),
            cum_gas_used: 8540680,
            logs: NiceVec(vec![]),
        };

        let mut receipt_deserializer = RlpDeserializer::new(&receipt_input);
        let receipt = TransactionReceipt::deserialize(&mut receipt_deserializer).unwrap();

        assert_eq!(receipt, receipt_expected);
    }
}

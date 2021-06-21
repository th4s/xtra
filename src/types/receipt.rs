use super::{ByteArray, ByteVec, NiceBigUint, NiceVec};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Receipts(NiceVec<TransactionReceipt>);

impl std::fmt::Display for Receipts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionReceipt {
    #[serde(deserialize_with = "deserialize_post_state")]
    post_state: PostState,
    cum_gas_used: NiceBigUint,
    logs: NiceVec<Log>,
}

impl std::fmt::Display for TransactionReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

fn deserialize_post_state<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<PostState, D::Error> {
    let buf = Vec::<u8>::deserialize(deserializer)?;
    if buf.len() == 1 {
        return Ok(PostState::Success(matches!(buf[0], 0x01)));
    }
    let mut out: [u8; 32] = [0; 32];
    out.copy_from_slice(&buf);
    Ok(PostState::State(ByteArray::<32>(out)))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostState {
    State(ByteArray<32>),
    Success(bool),
}

impl std::fmt::Display for PostState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostState::State(bytes) => write!(f, "{}", bytes),
            PostState::Success(success) => write!(f, "{}", success),
        }
    }
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

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::*;
    use crate::rlp::RlpDeserializer;

    #[test]
    fn test_receipt_deserialize() {
        // This is the receipt of the first transaction on mainnet in block 46147
        let receipt_input: Vec<u8> = vec![
            0xe6, 0xe5, 0xa0, 0x96, 0xa8, 0xe0, 0x09, 0xd2, 0xb8, 0x8b, 0x14, 0x83, 0xe6, 0x94,
            0x1e, 0x68, 0x12, 0xe3, 0x22, 0x63, 0xb0, 0x56, 0x83, 0xfa, 0xc2, 0x02, 0xab, 0xc6,
            0x22, 0xa3, 0xe3, 0x1a, 0xed, 0x19, 0x57, 0x82, 0x52, 0x08, 0xc0,
        ];

        let receipt_expected = Receipts(NiceVec(vec![TransactionReceipt {
            post_state: PostState::State(ByteArray::<32>([
                0x96, 0xa8, 0xe0, 0x09, 0xd2, 0xb8, 0x8b, 0x14, 0x83, 0xe6, 0x94, 0x1e, 0x68, 0x12,
                0xe3, 0x22, 0x63, 0xb0, 0x56, 0x83, 0xfa, 0xc2, 0x02, 0xab, 0xc6, 0x22, 0xa3, 0xe3,
                0x1a, 0xed, 0x19, 0x57,
            ])),
            cum_gas_used: NiceBigUint(BigUint::from(21000_u32)),
            logs: NiceVec(vec![]),
        }]));

        let mut receipt_deserializer = RlpDeserializer::new(&receipt_input).unwrap();
        let receipt = Receipts::deserialize(&mut receipt_deserializer).unwrap();

        assert_eq!(receipt, receipt_expected);
    }
}

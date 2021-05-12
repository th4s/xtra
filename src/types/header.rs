use ethereum_types::{H160, H256, H64, U128, U256};
use serde::Deserialize;
// Header returned by subscription
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Header {
    pub number: U128,
    pub hash: H256,
    pub parent_hash: H256,
    pub nonce: H64,
    pub transactions_root: H256,
    pub state_root: H256,
    pub receipts_root: H256,
    pub difficulty: U256,
    pub sha3_uncles: H256,
    pub miner: H160,
    pub logs_bloom: H256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub extra_data: Vec<u8>,
    pub mix_hash: H256,
}

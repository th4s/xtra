use super::NiceBigUint;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TotalDifficulty(pub NiceBigUint);

impl std::fmt::Display for TotalDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlp::RlpDeserializer;
    use num_bigint::BigUint;

    #[test]
    fn test_difficulty_deserialize() {
        // These are the total difficulties of blocks 2 and 1000
        let diff1_input: Vec<u8> = vec![0x85, 0x0b, 0xfe, 0x80, 0x10, 0x00];
        let diff2_input: Vec<u8> = vec![0x86, 0x14, 0x06, 0xe2, 0x2e, 0x34, 0xf5];

        let mut diff_deserializer1 = RlpDeserializer::new(&diff1_input);
        let mut diff_deserializer2 = RlpDeserializer::new(&diff2_input);

        let diff1 = TotalDifficulty::deserialize(&mut diff_deserializer1).unwrap();
        let diff2 = TotalDifficulty::deserialize(&mut diff_deserializer2).unwrap();

        assert_eq!(
            diff1,
            TotalDifficulty(NiceBigUint(BigUint::from(51_514_445_824_u64)))
        );
        assert_eq!(
            diff2,
            TotalDifficulty(NiceBigUint(BigUint::from(22_019_797_038_325_u64)))
        );
    }
}

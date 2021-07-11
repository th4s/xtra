use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

mod body;
mod difficulty;
mod hash;
mod header;
mod receipt;

pub use body::BlockBody;
pub use difficulty::TotalDifficulty;
pub use hash::BlockHash;
pub use header::BlockHeader;
pub use receipt::Receipts;

/// A const-sized byte array for types of known byte length
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct ByteArray<const N: usize>(#[serde(with = "serde_arrays")] pub [u8; N]);

impl<const N: usize> std::fmt::Display for ByteArray<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("0x");
        self.0
            .iter()
            .for_each(|x| out.push_str(&format!("{:02x}", x)));
        write!(f, "{}", out)
    }
}

impl<const N: usize> Serialize for ByteArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

/// A byte vector for types of unknown byte length
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct ByteVec(#[serde(serialize_with = "str_serialize")] pub Vec<u8>);

impl std::fmt::Display for ByteVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("0x");
        self.0
            .iter()
            .for_each(|x| out.push_str(&format!("{:02x}", x)));
        write!(f, "{}", out)
    }
}

impl Serialize for ByteVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

/// A vector which can pretty-print for JSON serialization
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NiceVec<T>(pub Vec<T>);

impl<T: std::fmt::Display + Serialize> std::fmt::Display for NiceVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl<T> Default for NiceVec<T> {
    fn default() -> Self {
        NiceVec(vec![])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
/// A big uint which can pretty-print for JSON serialization
pub struct NiceBigUint(#[serde(serialize_with = "str_serialize")] BigUint);

impl std::fmt::Display for NiceBigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.to_str_radix(10))
    }
}

pub fn str_serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: serde::ser::Serializer,
{
    serializer.collect_str(value)
}

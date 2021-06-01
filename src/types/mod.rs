pub use header::Header;
use serde::Deserialize;

mod body;
mod difficulty;
mod hash;
mod header;
mod receipt;

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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct ByteVec(pub Vec<u8>);

impl std::fmt::Display for ByteVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("0x");
        self.0
            .iter()
            .for_each(|x| out.push_str(&format!("{:02x}", x)));
        write!(f, "{}", out)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct NiceVec<T>(pub Vec<T>);

impl<T: std::fmt::Display> std::fmt::Display for NiceVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        self.0
            .iter()
            .for_each(|x| out.push_str(&format!("{}\n", x)));
        write!(f, "[\n\t{}\n]", out)
    }
}

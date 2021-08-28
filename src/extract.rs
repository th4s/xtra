use crate::numeric::{u16_from_bytes_be, u32_from_bytes_be, NumericError};
use crate::rlp::RlpDeserializer;
use log::{debug, info, trace};
use serde::de::DeserializeOwned;
use serde::Serialize;
use snap::raw::Decoder;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use thiserror::Error;

// A single index consists of 2 bytes (u16) for the file number and 4 bytes (u32) for the offset
const FILE_NUMBER_BYTE_SIZE: u64 = 2;
const OFFSET_NUMBER_BYTE_SIZE: u64 = 4;

/// Allows to export block parts from the `chaindata/ancient` folder from geth
///
/// The variant decides about which block parts you want to export.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Freezer {
    Bodies,
    Headers,
    Hashes,
    Difficulty,
    Receipts,
}

/// The index struct
///
/// Used to store ancient chaindata folder as well as filenumbers and offsets.
#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub ancient_folder: PathBuf,
    pub offsets: HashMap<u16, Vec<u64>>,
}

impl Freezer {
    /// Loads the index file into memory
    ///
    /// This index will contain the offsets and file numbers we need to load the raw block data.
    pub fn init(
        &self,
        ancient_folder: &Path,
        min_block: u64,
        max_block: u64,
    ) -> Result<Index, FreezerError> {
        if min_block >= max_block {
            return Err(FreezerError::BlockRange);
        }
        info!(
            "Attempting to read blocks {}-{} from freezer {}.",
            min_block, max_block, &self
        );

        info!("Reading index...");
        // Calculate some constans we need and open index file
        let index_size =
            (FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) * (max_block - min_block);
        let index_offset = (FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) * min_block;
        let index_filename = ancient_folder.join(self.index_filename());
        let mut index_file = File::open(index_filename).map_err(FreezerError::OpenFile)?;

        // Load the part of the index we need into a byte buffer
        let mut raw_index: Vec<u8> = Vec::with_capacity(index_size as usize);
        let _ = index_file
            .seek(SeekFrom::Start(index_offset))
            .map_err(FreezerError::SeekFile)?;
        let _ = index_file
            .take(index_size + FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE)
            .read_to_end(&mut raw_index)
            .map_err(FreezerError::ReadFile)?;

        let mut offsets = HashMap::<u16, Vec<u64>>::new();

        // Convert raw index bytes into file number and offsets
        for chunk in raw_index.chunks((FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) as usize) {
            let file_number = u16_from_bytes_be(&chunk[..FILE_NUMBER_BYTE_SIZE as usize])
                .map_err(FreezerError::Conversion)?;
            let offset = u32_from_bytes_be(&chunk[FILE_NUMBER_BYTE_SIZE as usize..])
                .map_err(FreezerError::Conversion)? as u64;

            offsets
                .entry(file_number)
                .or_insert_with(Vec::new)
                .push(offset);
        }

        let index = Index {
            ancient_folder: ancient_folder.into(),
            offsets,
        };
        Ok(index)
    }

    /// Loads the raw block data
    ///
    /// Returns a byte vector containing the requested block data
    pub fn load_data(
        &self,
        ancient_folder: &Path,
        file_number: u16,
        offsets: &[u64],
    ) -> Result<Vec<u8>, FreezerError> {
        info!(
            "Reading raw block data from {}.",
            self.data_filename(file_number)
        );
        let mut block_data: Vec<u8> = Vec::new();

        let data_file_name = ancient_folder.join(self.data_filename(file_number));
        let mut data_file = File::open(data_file_name).map_err(FreezerError::OpenFile)?;

        let read_bytes = seek_and_read(
            &mut data_file,
            &mut block_data,
            *offsets.first().ok_or(FreezerError::BlockOffset)?,
            *offsets.last().ok_or(FreezerError::BlockOffset)?,
        )?;
        debug!("Read {} bytes of data", read_bytes);
        Ok(block_data)
    }

    /// Exports the block data as json
    ///
    /// Returns a string
    pub fn export_json<T: DeserializeOwned + Display + Serialize>(
        &self,
        block_offsets: &[u64],
        block_data: &[u8],
    ) -> Result<String, FreezerError> {
        info!("Decompressing and deserializing output.");
        let offset_offset = block_offsets.first().ok_or(FreezerError::BlockOffset)?;
        let mut block_objects = String::new();

        // This closure decompresses snappy if necessary
        let decompressor = |input: &[u8]| -> Result<Vec<u8>, FreezerError> {
            if self.is_compressed() {
                trace!("Decompressing...");
                Decoder::new()
                    .decompress_vec(input)
                    .map_err(FreezerError::SnappyDecompress)
            } else {
                trace!("Input is not compressed.");
                Ok(input.to_vec())
            }
        };

        // This closure deserializes the rlp bytes
        let rlp_deserialize = |input: &[u8]| -> Result<T, FreezerError> {
            trace!("Deserializing...");
            // Ugly hack to adapt hashes in freezer to RLP format. Somehow geth does not export
            // hashes to the freezer in correct RLP format
            let mut tmp = vec![];
            let input = if *self == Freezer::Hashes {
                tmp.extend_from_slice(&[&[0xa0_u8], input].concat());
                tmp.as_slice()
            } else {
                input
            };
            let mut deserializer =
                RlpDeserializer::new(input).map_err(FreezerError::RlpDeserialization)?;
            T::deserialize(&mut deserializer).map_err(FreezerError::RlpDeserialization)
        };

        for (k, offsets) in block_offsets.windows(2).enumerate() {
            let blob = decompressor(
                &block_data
                    [(offsets[0] - offset_offset) as usize..(offsets[1] - offset_offset) as usize],
            )?;
            block_objects.push_str(&(rlp_deserialize(&blob)?.to_string() + ",\n"));

            if k % 10000 == 0 && k > 0 {
                info!("Processed {}k blocks.", k / 1000);
            }
        }
        Ok(block_objects)
    }

    const fn index_filename(&self) -> &'static str {
        match *self {
            Self::Bodies => "bodies.cidx",
            Self::Headers => "headers.cidx",
            Self::Hashes => "hashes.ridx",
            Self::Difficulty => "diffs.ridx",
            Self::Receipts => "receipts.cidx",
        }
    }

    fn data_filename(&self, file_number: u16) -> String {
        match *self {
            Self::Bodies => format!("bodies.{:04}.cdat", file_number),
            Self::Headers => format!("headers.{:04}.cdat", file_number),
            Self::Hashes => format!("hashes.{:04}.rdat", file_number),
            Self::Difficulty => format!("diffs.{:04}.rdat", file_number),
            Self::Receipts => format!("receipts.{:04}.cdat", file_number),
        }
    }

    const fn is_compressed(&self) -> bool {
        match *self {
            Self::Bodies => true,
            Self::Headers => true,
            Self::Hashes => false,
            Self::Difficulty => false,
            Self::Receipts => true,
        }
    }
}

impl Display for Freezer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Bodies => "bodies",
                Self::Receipts => "receipts",
                Self::Headers => "headers",
                Self::Difficulty => "difficulty",
                Self::Hashes => "hashes",
            }
        )
    }
}

fn seek_and_read(
    file: &mut File,
    buffer: &mut Vec<u8>,
    start: u64,
    end: u64,
) -> Result<usize, FreezerError> {
    trace!("Reading data in file, beginning at byte {}...", start);
    let _ = file
        .seek(SeekFrom::Start(start))
        .map_err(FreezerError::SeekFile)?;
    file.take(end - start)
        .read_to_end(buffer)
        .map_err(FreezerError::ReadFile)
}

/// Collects different errors
#[derive(Debug, Error)]
pub enum FreezerError {
    #[error("Invalid block range. Minimum block is larger than or equal to maximum block")]
    BlockRange,
    #[error("Cannot open file, {0}")]
    OpenFile(#[source] std::io::Error),
    #[error("Cannot seek provided file offset, {0}")]
    SeekFile(#[source] std::io::Error),
    #[error("Cannot read from file, {0}")]
    ReadFile(#[source] std::io::Error),
    #[error("Unable to convert raw bytes into block offsets, {0}")]
    Conversion(#[source] NumericError),
    #[error("Unable to read file metadata, {0}")]
    FileMetadata(#[source] std::io::Error),
    #[error("Cannot determine block offset")]
    BlockOffset,
    #[error("Read error during decompression, {0}")]
    SnappyDecompress(#[source] snap::Error),
    #[error("Error during rlp deserialization, {0}")]
    RlpDeserialization(#[source] crate::rlp::RlpError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_freezer_seek_and_read() {
        let file_name = PathBuf::from("./fixtures/bodies/bodies.0000.cdat");
        let mut file = File::open(file_name).unwrap();
        let mut buffer = Vec::<u8>::new();

        let _ = seek_and_read(&mut file, &mut buffer, 5, 15).unwrap();
        let expected: Vec<u8> = vec![3, 8, 194, 192, 192, 3, 8, 194, 192, 192];
        assert_eq!(buffer, expected);
    }
}

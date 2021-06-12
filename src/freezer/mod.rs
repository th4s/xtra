use crate::numeric::{u16_from_bytes_be, u32_from_bytes_be, NumericError};
use log::{debug, info, trace};
use snap::raw::Decoder;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub mod rlp;

// A single index consists of 2 bytes (u16) for the file number and 4 bytes (u32) for the offset
const FILE_NUMBER_BYTE_SIZE: u64 = 2;
const OFFSET_NUMBER_BYTE_SIZE: u64 = 4;

pub struct Freezer {
    ancient_folder: PathBuf,
}

impl Freezer {
    pub fn new(ancient_folder: PathBuf) -> Self {
        Freezer { ancient_folder }
    }

    pub fn defrost(
        &self,
        block_part: BlockPart,
        min_block: u64,
        max_block: u64,
    ) -> Result<(), FreezerError> {
        todo!()
    }
}

/// Allows to export block parts from the `chaindata/ancient` folder from geth
///
/// The variant decides about which block parts you want to export.
#[derive(Debug, Clone, Copy)]
pub enum BlockPart {
    Bodies,
    Headers,
    Hashes,
    Difficulty,
    Receipts,
}

impl BlockPart {
    /// Loads a range of block parts from min_block (inclusive) to max_block (exclusive).
    /// Returns two vecs. The second vec contains the raw block data and the first vec
    /// contains the byte offset of the first byte for every block in the second vec.
    pub fn load(
        &self,
        ancient_folder: &Path,
        min_block: u64,
        max_block: u64,
    ) -> Result<Vec<u32>, FreezerError> {
        if min_block >= max_block {
            return Err(FreezerError::BlockRange);
        }
        info!(
            "Reading {} of blocks {}-{}...",
            format!("{}", self),
            min_block,
            max_block
        );

        let index_size =
            (FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) * (max_block - min_block);
        let index_shift = (FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) * min_block;
        let index_filename = ancient_folder.join(self.index_filename());
        let mut index_file = File::open(index_filename).map_err(FreezerError::OpenFile)?;

        // Get first and last offset
        let (first_file_number, first_offset) =
            jump_to_block_number_and_read_single_index(&mut index_file, min_block)?;
        let (last_file_number, last_offset) =
            jump_to_block_number_and_read_single_index(&mut index_file, max_block)?;

        // Load the raw index data into RAM
        let block_offsets_raw = self.load_index_raw(index_size, index_shift, &mut index_file)?;

        // Adapt the index offsets
        let block_offsets =
            self.postprocess_index(ancient_folder, index_size, first_offset, block_offsets_raw)?;

        // Load the raw block data into RAM
        let block_data = self.load_data(
            ancient_folder,
            first_file_number,
            last_file_number,
            first_offset,
            last_offset,
        )?;

        // Decompress if necessary and turn into vec of blobs
        let block_parts = self.decompress(block_data.as_slice(), block_offsets.as_slice())?;

        // Next step is to RLP-decode
        // let rlp_objects: Vec<Rlp> = self.rlp_decode(block_parts.as_slice())?;

        info!("Reading successful");
        Ok(vec![])
    }

    fn load_data(
        &self,
        ancient_folder: &Path,
        first_file_number: u16,
        last_file_number: u16,
        first_offset: u64,
        last_offset: u64,
    ) -> Result<Vec<u8>, FreezerError> {
        debug!("Reading raw block data from file number {} with offset {} until file number {} with offset {}...",
               first_file_number, first_offset, last_file_number, last_offset);
        let mut current_file_number = first_file_number;
        let mut block_data: Vec<u8> = Vec::new();

        while current_file_number <= last_file_number {
            let data_file_name = ancient_folder.join(self.data_filename(current_file_number));
            let mut data_file = File::open(data_file_name).map_err(FreezerError::OpenFile)?;

            let start = if current_file_number == first_file_number {
                first_offset
            } else {
                0
            };

            let end = if current_file_number == last_file_number {
                Some(last_offset)
            } else {
                None
            };
            let _ = seek_and_read(&mut data_file, &mut block_data, start, end)?;
            current_file_number += 1;
        }
        debug!("Read {} bytes of data", block_data.len());
        Ok(block_data)
    }

    fn load_index_raw(
        &self,
        index_size: u64,
        index_shift: u64,
        index_file: &mut File,
    ) -> Result<Vec<u8>, FreezerError> {
        debug!(
            "Reading {} bytes starting at byte {} to build index...",
            index_size, index_shift
        );

        let mut raw_index: Vec<u8> = Vec::with_capacity(index_size as usize);

        let _ = index_file
            .seek(SeekFrom::Start(index_shift))
            .map_err(FreezerError::SeekFile)?;
        let _ = index_file
            .take(index_size)
            .read_to_end(&mut raw_index)
            .map_err(FreezerError::ReadFile)?;
        Ok(raw_index)
    }

    fn postprocess_index(
        &self,
        ancient_folder: &Path,
        index_size: u64,
        first_offset: u64,
        raw_index: Vec<u8>,
    ) -> Result<Vec<u64>, FreezerError> {
        debug!("Postprocessing index...");
        let mut block_offsets: Vec<u64> = Vec::with_capacity(index_size as usize);
        let mut offset_shift: i64 = -(first_offset as i64);

        for chunk in raw_index.chunks((FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) as usize) {
            let file_number = u16_from_bytes_be(&chunk[..FILE_NUMBER_BYTE_SIZE as usize])
                .map_err(FreezerError::Conversion)?;
            let offset = u32_from_bytes_be(&chunk[FILE_NUMBER_BYTE_SIZE as usize..])
                .map_err(FreezerError::Conversion)? as i64;

            if offset == 0 && !block_offsets.is_empty() {
                let data_file_name = ancient_folder.join(self.data_filename(file_number - 1));
                let data_file = File::open(data_file_name).map_err(FreezerError::OpenFile)?;
                let file_len = data_file
                    .metadata()
                    .map_err(FreezerError::FileMetadata)?
                    .len();
                offset_shift =
                    (file_len - *block_offsets.last().ok_or(FreezerError::BlockOffset)?) as i64;
            }
            block_offsets.push((offset + offset_shift) as u64);
        }
        Ok(block_offsets)
    }

    fn decompress(
        &self,
        block_data: &[u8],
        block_offsets: &[u64],
    ) -> Result<Vec<Vec<u8>>, FreezerError> {
        let mut block_parts: Vec<Vec<u8>> = Vec::new();
        let decompressor = |input: &[u8]| -> Result<Vec<u8>, FreezerError> {
            if self.is_compressed() {
                debug!("Decompressing data...");
                Decoder::new()
                    .decompress_vec(input)
                    .map_err(FreezerError::SnappyDecompress)
            } else {
                let mut out = Vec::with_capacity(input.len());
                out.copy_from_slice(input);
                Ok(out)
            }
        };

        for offsets in block_offsets.windows(2) {
            let blob = decompressor(&block_data[offsets[0] as usize..offsets[1] as usize])?;
            block_parts.push(blob)
        }
        // We need to add the last block part manually, since we are working with offsets here
        let blob = decompressor(
            &block_data[*block_offsets.last().ok_or(FreezerError::BlockOffset)? as usize..],
        )?;
        block_parts.push(blob);
        Ok(block_parts)
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

impl Display for BlockPart {
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

fn jump_to_block_number_and_read_single_index(
    index_file: &mut File,
    block_number: u64,
) -> Result<(u16, u64), FreezerError> {
    trace!("Reading single index for block number {}", block_number);
    let _ = index_file
        .seek(SeekFrom::Start(
            (FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) * block_number,
        ))
        .map_err(FreezerError::SeekFile)?;
    let mut file_number: [u8; 2] = [0; 2];
    let mut offset: [u8; 4] = [0; 4];
    index_file
        .read_exact(&mut file_number)
        .map_err(FreezerError::ReadFile)?;
    index_file
        .read_exact(&mut offset)
        .map_err(FreezerError::ReadFile)?;
    Ok((
        u16_from_bytes_be(&file_number).map_err(FreezerError::Conversion)?,
        u32_from_bytes_be(&offset).map_err(FreezerError::Conversion)? as u64,
    ))
}

fn seek_and_read(
    file: &mut File,
    buffer: &mut Vec<u8>,
    start: u64,
    end: Option<u64>,
) -> Result<usize, FreezerError> {
    trace!("Reading data in file, beginning at byte {}...", start);
    let _ = file
        .seek(SeekFrom::Start(start))
        .map_err(FreezerError::SeekFile)?;
    match end {
        Some(pos) => file
            .take(pos - start)
            .read_to_end(buffer)
            .map_err(FreezerError::ReadFile),
        None => file.read_to_end(buffer).map_err(FreezerError::ReadFile),
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_freezer_export_bodies() {
        let path_buf = PathBuf::from("./fixtures/bodies");
        // Check if we can read 50k blocks without errors
        let _bodies = BlockPart::Bodies
            .load(path_buf.as_path(), 0, 49999)
            .unwrap();
    }

    #[test]
    fn test_freezer_export_headers() {
        let path_buf = PathBuf::from("./fixtures/headers");
        // Check if we can read some headers without errors
        let headers = BlockPart::Headers.load(path_buf.as_path(), 0, 99).unwrap();
    }

    #[test]
    fn test_freezer_jump_to_block_number_and_read_single_index() {
        let file_name = PathBuf::from("./fixtures/bodies/bodies.cidx");
        let mut file = File::open(file_name).unwrap();
        let (file_number, offset) =
            jump_to_block_number_and_read_single_index(&mut file, 20).unwrap();
        assert_eq!(file_number, 0);
        assert_eq!(offset, 969);
    }

    #[test]
    fn test_freezer_seek_and_read() {
        let file_name = PathBuf::from("./fixtures/bodies/bodies.0000.cdat");
        let mut file = File::open(file_name).unwrap();
        let mut buffer = Vec::<u8>::new();

        let _ = seek_and_read(&mut file, &mut buffer, 5, Some(15)).unwrap();
        let expected: Vec<u8> = vec![3, 8, 194, 192, 192, 3, 8, 194, 192, 192];
        assert_eq!(buffer, expected);
    }
}

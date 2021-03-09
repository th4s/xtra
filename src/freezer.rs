use byteorder::{BigEndian, ReadBytesExt};
use log::{debug, info, trace};
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

// A single index consists of 2 bytes (u16) for the file number and 4 bytes (u32) for the offset
const FILE_NUMBER_BYTE_SIZE: u64 = 2;
const OFFSET_NUMBER_BYTE_SIZE: u64 = 4;

/// Allows to export block parts from the `chaindata/ancient` folder from geth
///
/// The variant decides about which block parts you want to export.
#[derive(Debug, Clone, Copy)]
pub enum Freezer {
    Bodies,
    Headers,
    Hashes,
    Difficulty,
    Receipts,
}

impl Freezer {
    /// Exports a range of block parts from min_block (inclusive) to max_block (exclusive).
    /// Returns two vecs. The second vec contains the raw block data and the first vec
    /// contains the byte offset of the first byte for every block in the second vec.
    pub fn export(
        &self,
        ancient_folder: &Path,
        min_block: u64,
        max_block: u64,
    ) -> Result<(Vec<u64>, Vec<u8>), FreezerError> {
        if min_block >= max_block {
            return Err(FreezerError::BlockRange);
        }
        info!(
            "Exporting {} of blocks {}-{}...",
            format!("{}", self),
            min_block,
            max_block
        );

        let index_filename = ancient_folder.join(self.index_filename());
        let mut index_file = File::open(index_filename).map_err(FreezerError::OpenFile)?;

        // Get first and last offset
        let (first_file_number, first_offset) =
            jump_to_block_number_and_read_single_index(&mut index_file, min_block)?;
        let (last_file_number, last_offset) =
            jump_to_block_number_and_read_single_index(&mut index_file, max_block)?;

        let block_data = self.export_data(
            ancient_folder,
            first_file_number,
            last_file_number,
            first_offset,
            last_offset,
        )?;

        let block_offsets = self.export_index(
            ancient_folder,
            &mut index_file,
            min_block,
            max_block,
            first_offset,
        )?;

        info!("Export successful");
        Ok((block_offsets, block_data))
    }

    fn export_data(
        &self,
        ancient_folder: &Path,
        first_file_number: u16,
        last_file_number: u16,
        first_offset: u64,
        last_offset: u64,
    ) -> Result<Vec<u8>, FreezerError> {
        debug!("Exporting raw data...");
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

    fn export_index(
        &self,
        ancient_folder: &Path,
        index_file: &mut File,
        min_block: u64,
        max_block: u64,
        first_offset: u64,
    ) -> Result<Vec<u64>, FreezerError> {
        debug!("Building index...");
        let index_size =
            (FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) * (max_block - min_block);
        let shift = (FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) * min_block;
        let mut tmp_buffer: Vec<u8> = Vec::with_capacity(index_size as usize);

        let _ = index_file
            .seek(SeekFrom::Start(shift))
            .map_err(FreezerError::SeekFile)?;
        let _ = index_file
            .take(index_size)
            .read_to_end(&mut tmp_buffer)
            .map_err(FreezerError::ReadFile)?;

        let mut block_offsets: Vec<u64> = Vec::with_capacity(index_size as usize);
        let mut offset_shift: i64 = -(first_offset as i64);

        for chunk in tmp_buffer.chunks((FILE_NUMBER_BYTE_SIZE + OFFSET_NUMBER_BYTE_SIZE) as usize) {
            let file_number = u16::from_be_bytes(
                chunk[..FILE_NUMBER_BYTE_SIZE as usize]
                    .try_into()
                    .map_err(FreezerError::ByteConversion)?,
            );
            let offset = u32::from_be_bytes(
                chunk[FILE_NUMBER_BYTE_SIZE as usize..]
                    .try_into()
                    .map_err(FreezerError::ByteConversion)?,
            ) as i64;

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

    fn index_filename(&self) -> &'static str {
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
    Ok((
        index_file
            .read_u16::<BigEndian>()
            .map_err(FreezerError::ReadFile)?,
        index_file
            .read_u32::<BigEndian>()
            .map_err(FreezerError::ReadFile)? as u64,
    ))
}

fn seek_and_read(
    file: &mut File,
    buffer: &mut Vec<u8>,
    start: u64,
    end: Option<u64>,
) -> Result<usize, FreezerError> {
    trace!("Reading data in file...");
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
    #[error("Cannot open file")]
    OpenFile(#[source] std::io::Error),
    #[error("Cannot seek provided file offset")]
    SeekFile(#[source] std::io::Error),
    #[error("Cannot read from file")]
    ReadFile(#[source] std::io::Error),
    #[error("Unable to convert raw bytes into block offsets")]
    ByteConversion(#[source] std::array::TryFromSliceError),
    #[error("Unable to read file metadata")]
    FileMetadata(#[source] std::io::Error),
    #[error("Cannot determine block offset")]
    BlockOffset,
}

// Fixture data in folder `ancient_01` contains the bodies of the first 50k blocks of the
// Ethereum mainnet exported from the geth `chaindata/ancient` folder
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_freezer_export_bodies() {
        let path_buf = PathBuf::from("./tests/fixtures/ancient_01");
        let (offsets, data) = Freezer::Bodies
            .export(path_buf.as_path(), 20000, 49999)
            .unwrap();

        // The number of bytes should be larger than our largest offset
        assert!(data.len() > *offsets.last().unwrap() as usize);

        // Offsets should be increasing
        for elements in offsets.windows(2) {
            assert!(elements[1] > elements[0]);
        }

        let (offsets, _data) = Freezer::Bodies.export(path_buf.as_path(), 1, 25).unwrap();

        // We know that blocks 3, 4, 7, 21 have uncles. Thus, we can check for off-by-one errors, by making sure
        // that the byte size of these blocks is larger
        for (pos, elements) in offsets.windows(2).enumerate() {
            // We have excluded block 0, so we have to shift the positions by one
            if [2, 3, 6, 20].contains(&pos) {
                assert!(elements[1] - elements[0] > 10)
            } else {
                assert!(elements[1] - elements[0] < 10)
            }
        }
    }

    #[test]
    fn test_freezer_jump_to_block_number_and_read_single_index() {
        let file_name = PathBuf::from("./tests/fixtures/ancient_01/bodies.cidx");
        let mut file = File::open(file_name).unwrap();
        let (file_number, offset) =
            jump_to_block_number_and_read_single_index(&mut file, 20).unwrap();
        assert_eq!(file_number, 0);
        assert_eq!(offset, 969);
    }

    #[test]
    fn test_freezer_seek_and_read() {
        let file_name = PathBuf::from("./tests/fixtures/ancient_01/bodies.0000.cdat");
        let mut file = File::open(file_name).unwrap();
        let mut buffer = Vec::<u8>::new();

        let _ = seek_and_read(&mut file, &mut buffer, 5, Some(15)).unwrap();
        let expected: Vec<u8> = vec![3, 8, 194, 192, 192, 3, 8, 194, 192, 192];
        assert_eq!(buffer, expected);
    }
}

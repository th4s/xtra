use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

// A single index consists of 2 bytes (u16) for the file number and 4 bytes (u32) for the offset
const FILE_NUMBER_BYTE_SIZE: u64 = 2;
const OFFSET_NUMBER_BYTE_SIZE: u64 = 4;

/// Specifies which part of a block you want to read
#[derive(Debug, Clone, Copy)]
pub enum Freezer {
    Bodies,
    Headers,
    Hashes,
    Difficulty,
    Receipts,
}

impl Freezer {
    pub fn export(
        &self,
        ancient_folder: &Path,
        (block_offsets, block_data): (&mut Vec<u64>, &mut Vec<u8>),
        min_block: u64,
        max_block: u64,
    ) -> Result<u32, FreezerError> {
        if min_block >= max_block {
            return Err(FreezerError::BlockRange);
        }

        let index_filename = ancient_folder.join(self.index_filename());
        let mut index_file = File::open(index_filename).map_err(FreezerError::OpenFile)?;

        let (last_file_number, last_offset) =
            jump_to_block_number_and_read_index(&mut index_file, max_block)?;
        let (first_file_number, first_offset) =
            jump_to_block_number_and_read_index(&mut index_file, min_block)?;

        let mut current_file_number = first_file_number;
        let mut read_bytes: u64 = 0;

        while current_file_number <= last_file_number {
            let data_file_name = ancient_folder.join(self.data_filename(current_file_number));
            let mut data_file = File::open(data_file_name).map_err(FreezerError::OpenFile)?;
            let read_bytes = if current_file_number == first_file_number {
                seek_and_read(&mut data_file, block_data, first_offset, None)
            } else if current_file_number == last_file_number {
                seek_and_read(&mut data_file, block_data, 0, Some(last_offset))
            } else {
                seek_and_read(&mut data_file, block_data, 0, None)
            }?;
            current_file_number = current_file_number + 1;
        }

        Ok(0)
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

fn jump_to_block_number_and_read_index(
    index_file: &mut File,
    block_number: u64,
) -> Result<(u16, u64), FreezerError> {
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
}

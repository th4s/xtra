use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::ops::Range;
use std::path::{Path, PathBuf};
use thiserror::Error;

// A single index consists of 2 bytes (u16) for the file number and 4 bytes (u32) for the offset
const INDEX_SIZE: usize = 6;

/// Allows to read blockchain data from the geth ancient folder in a convenient way
pub struct BlockReader {
    /// The possible range, in which blocks can be read
    pub block_range: Range<usize>,
    freezer_folder: PathBuf,
    block_part: BlockPart,
    index: (Vec<u16>, Vec<u32>),
}

impl BlockReader {
    /// Create a new BlockReader by specifying
    /// - which part of the blocks you want to read
    /// - the range of blocks
    /// - the ancient folder of the chaindata folder
    pub fn new(
        block_part: BlockPart,
        mut block_range: Range<usize>,
        freezer_folder: PathBuf,
    ) -> Result<Self, FreezerError> {
        let index = block_part.read_index(freezer_folder.as_path(), &mut block_range)?;
        Ok(Self {
            block_range,
            freezer_folder,
            block_part,
            index,
        })
    }

    /// Read a range of blocks.
    pub fn read_range(&self, block_range: Range<usize>) -> Result<BlockData, FreezerError> {
        if !self.block_range.contains(&block_range.start)
            || !self.block_range.contains(&block_range.end)
        {
            return Err(FreezerError::OutOfBound);
        }

        let mut file_handles: HashMap<u16, Result<BufReader<File>, FreezerError>> = HashMap::new();
        let mut data: Vec<u8> = Vec::with_capacity(block_range.len());
        let mut borders: Vec<usize> = Vec::with_capacity(block_range.len());
        let mut last_offset: usize =
            *self.index.1.get(block_range.start - 1).unwrap_or(&0_u32) as usize;

        for (file_number, offset) in self.index.0[block_range.clone()]
            .iter()
            .zip(self.index.1[block_range].iter())
        {
            let current_offset = *offset as usize;
            let current_file = file_handles.entry(*file_number).or_insert_with(|| {
                Ok(BufReader::new(
                    File::open(
                        self.freezer_folder
                            .join(self.block_part.data_filename(*file_number)),
                    )
                    .map_err(FreezerError::DataFileNotFound)?,
                ))
            });

            let _read_bytes = current_file
                .as_mut()
                .expect("File not present in hashmap. Should be impossible!")
                .read_exact(&mut data[last_offset..current_offset]);

            borders.push(current_offset - last_offset);
            last_offset = current_offset
        }

        Ok(BlockData {
            data,
            bounds: borders,
            block_part: self.block_part,
        })
    }
}

/// Specifies which part of a block you want to read
#[derive(Debug, Clone, Copy)]
pub enum BlockPart {
    Bodies,
    Headers,
    Hashes,
    Difficulty,
    Receipts,
}

impl BlockPart {
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

    fn read_index(
        &self,
        chain_folder: &Path,
        block_range: &mut Range<usize>,
    ) -> Result<(Vec<u16>, Vec<u32>), FreezerError> {
        let index_path = chain_folder.join(self.index_filename());
        let index_file = File::open(index_path).map_err(FreezerError::IndexFileNotFound)?;

        let mut buffer = BufReader::new(index_file);
        buffer
            .seek(SeekFrom::Start((INDEX_SIZE * block_range.start) as u64))
            .map_err(FreezerError::Offset)?;

        let mut file_number: Vec<u16> = Vec::with_capacity(block_range.len());
        let mut offset: Vec<u32> = Vec::with_capacity(block_range.len());
        for _ in block_range {
            file_number.push(
                buffer
                    .read_u16::<BigEndian>()
                    .map_err(FreezerError::Buffer)?,
            );
            offset.push(
                buffer
                    .read_u32::<BigEndian>()
                    .map_err(FreezerError::Buffer)?,
            );
        }
        Ok((file_number, offset))
    }
}

/// The raw blockchain data in bytes
pub struct BlockData {
    /// Specifies which blockchain data
    pub block_part: BlockPart,
    /// The raw data in bytes
    pub data: Vec<u8>,
    /// Specifies the boundary bytes between adjacent block data
    pub bounds: Vec<usize>,
}

/// An error type collecting what can go wrong
#[derive(Debug, Error)]
pub enum FreezerError {
    #[error("Could not find index file")]
    IndexFileNotFound(#[source] std::io::Error),
    #[error("Could not find data file")]
    DataFileNotFound(#[source] std::io::Error),
    #[error("Unable to jump to specified start position in index file")]
    Offset(#[source] std::io::Error),
    #[error("Could not read from file")]
    Buffer(#[source] std::io::Error),
    #[error("Index out of bound")]
    OutOfBound,
}

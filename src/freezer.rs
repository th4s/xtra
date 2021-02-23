use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::ops::Range;
use std::path::{Path, PathBuf};
use thiserror::Error;

const INDEX_SIZE: usize = 6;
const CHUNK_SIZE: usize = 2_000_000_000;

pub struct BlockReader(PathBuf);

impl BlockReader {
    pub fn read_raw(
        &self,
        block_part: BlockPart,
        block_range: Range<usize>,
    ) -> Result<Vec<Vec<u8>>, FreezerError> {
        let index = block_part.read_index(self.0.as_path(), block_range)?;
        todo!()
    }
}

#[derive(Debug)]
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

    fn data_filename(&self, file_number: usize) -> String {
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
        block_range: Range<usize>,
    ) -> Result<Vec<(u16, u32)>, FreezerError> {
        let index_path = chain_folder.join(self.index_filename());
        let index_file = File::open(index_path).map_err(FreezerError::IndexFileNotFound)?;

        let mut buffer = BufReader::new(index_file);
        buffer
            .seek(SeekFrom::Start((INDEX_SIZE * block_range.start) as u64))
            .map_err(FreezerError::Offset)?;

        let mut index: Vec<(u16, u32)> = Vec::with_capacity(block_range.len());
        for _ in block_range {
            index.push((
                buffer
                    .read_u16::<BigEndian>()
                    .map_err(FreezerError::Buffer)?,
                buffer
                    .read_u32::<BigEndian>()
                    .map_err(FreezerError::Buffer)?,
            ));
        }
        Ok(index)
    }
}

#[derive(Debug, Error)]
pub enum FreezerError {
    #[error("Could not find index file")]
    IndexFileNotFound(#[source] std::io::Error),
    #[error("Unable to jump to specified start position in index file")]
    Offset(#[source] std::io::Error),
    #[error("Could not read from file")]
    Buffer(#[source] std::io::Error),
}

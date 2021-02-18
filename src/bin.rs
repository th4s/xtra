// Mount chaindata: sshfs thojest@giga:/var/lib/geth/geth/chaindata/ancient ~/chaindata_ancient/ -o ro,reconnect
// Unmount: fusermount -u ~/chaindata_ancient

use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

const CHAINDATA_PATH: &str = "../chaindata_ancient/";
const INDEX_SIZE: usize = 6;
const BLOCK_NUMBER: usize = 10020874;

fn main() {
    let bodies_index_file_path = String::from(CHAINDATA_PATH) + "bodies.cidx";
    let file = File::open(bodies_index_file_path).expect("Unable to open file");

    let mut reader = BufReader::with_capacity(INDEX_SIZE, file);
    let mut buffer: [u8; 6] = [0; 6];
    let _ = reader.seek(SeekFrom::Start((INDEX_SIZE * BLOCK_NUMBER) as u64));
    let _ = reader.read_exact(&mut buffer);

    let file_number = BigEndian::read_u16(&buffer[..2]);
    let offset = BigEndian::read_u32(&buffer[2..]);
    println!("File number is {}, and offset is {}", file_number, offset);
}

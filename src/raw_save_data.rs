use std::io;
use std::io::Read;

const HEADER_SIZE_BYTES: usize = 0x70;
const DATA_SIZE_BYTES: usize = 10613 * 4;
const SAVE_FILE_SIZE_BYTES: usize = HEADER_SIZE_BYTES + DATA_SIZE_BYTES;

#[derive(Clone)]
pub struct RawSaveData {
    pub raw: [u8; SAVE_FILE_SIZE_BYTES]
}

impl RawSaveData {
    pub fn from_sav<R: Read>(file: &mut R) -> io::Result<RawSaveData> {
        let mut raw = [0; SAVE_FILE_SIZE_BYTES];
        file.read_exact(&mut raw)?;

        Ok(RawSaveData { raw })
    }
}
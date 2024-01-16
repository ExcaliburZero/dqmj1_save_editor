pub struct DataLocation {
    offset: usize,
    length: usize,
}

impl DataLocation {
    pub fn new(offset: usize, length: usize) -> DataLocation {
        DataLocation { offset, length }
    }

    pub fn read(&self, raw_save_data: &[u8]) -> Vec<u8> {
        raw_save_data[self.offset..self.offset + self.length].to_vec()
    }

    pub fn write(&self, raw_save_data: &mut [u8], value: Vec<u8>) {
        for (offset, v) in (self.offset..self.offset + self.length).zip(value.iter()) {
            raw_save_data[offset] = *v;
        }
    }
}

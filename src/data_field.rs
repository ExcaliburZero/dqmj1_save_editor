use crate::raw_save_data::RawSaveData;

#[derive(Debug)]
pub enum DataValue {
    U32(u32)
}

#[derive(Debug)]
pub enum ValueReadFailureReason {}

pub trait DataField {
    fn read(&self, raw_save_data: &RawSaveData) -> Result<DataValue, ValueReadFailureReason>;
}

pub struct DataLocation {
    offset: usize,
    length: usize,
}

impl DataLocation {
    pub fn new(offset: usize, length: usize) -> DataLocation {
        DataLocation { offset, length }
    }

    pub fn read(&self, raw_save_data: &RawSaveData) -> Vec<u8> {
        raw_save_data.raw[self.offset..self.offset + self.length].to_vec()
    }

    pub fn write(&self, raw_save_data: &mut RawSaveData, value: Vec<u8>) {
        for (offset, v) in (self.offset..self.offset + self.length).zip(value.iter()) {
            raw_save_data.raw[offset] = *v;
        }
    }
}

pub struct DataFieldU32 {
    location: DataLocation
}

impl DataFieldU32 {
    pub fn new(offset: usize, length: usize) -> DataFieldU32 {
        DataFieldU32 { location: DataLocation::new(offset, length) }
    }
}

impl DataField for DataFieldU32 {
    fn read(&self, raw_save_data: &RawSaveData) -> Result<DataValue, ValueReadFailureReason> {
        Ok(DataValue::U32(u32::from_le_bytes(self.location.read(raw_save_data).try_into().unwrap())))
    }
}
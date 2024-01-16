use crate::data_fields::data_field::{DataField, DataValue, ValueReadFailureReason};
use crate::data_fields::data_location::DataLocation;

pub struct DataFieldU32 {
    location: DataLocation,
}

impl DataFieldU32 {
    pub fn new(offset: usize, length: usize) -> DataFieldU32 {
        DataFieldU32 {
            location: DataLocation::new(offset, length),
        }
    }
}

impl DataField for DataFieldU32 {
    fn read(&self, raw_save_data: &[u8]) -> Result<DataValue, ValueReadFailureReason> {
        Ok(DataValue::U32(u32::from_le_bytes(
            self.location.read(raw_save_data).try_into().unwrap(),
        )))
    }
}

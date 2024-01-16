use crate::data_fields::monster::Monster;

#[derive(Clone, Debug)]
pub enum DataValue {
    U8(u8),
    U32(u32),
    Monster(Monster),
}

#[derive(Debug)]
pub enum ValueReadFailureReason {}

pub trait DataField {
    fn read(&self, raw_save_data: &[u8]) -> Result<DataValue, ValueReadFailureReason>;
    fn write(&self, raw_save_data: &mut [u8], value: &DataValue);
}

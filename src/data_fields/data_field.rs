use crate::data_fields::monster::Monster;

#[derive(Clone, Debug)]
pub enum DataValue {
    U8(u8),
    U32(u32),
    Monster(Monster),
}

impl DataValue {
    pub fn get_u32(&self) -> u32 {
        match self {
            DataValue::U32(v) => *v,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum ValueReadFailureReason {}

pub trait DataField {
    fn read(&self, raw_save_data: &[u8]) -> Result<DataValue, ValueReadFailureReason>;
    fn write(&self, raw_save_data: &mut [u8], value: &DataValue);
}

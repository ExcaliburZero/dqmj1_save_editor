#[derive(Debug)]
pub enum DataValue {
    U32(u32)
}

#[derive(Debug)]
pub enum ValueReadFailureReason {}

pub trait DataField {
    fn read(&self, raw_save_data: &[u8]) -> Result<DataValue, ValueReadFailureReason>;
}
use std::collections::HashMap;

use crate::raw_save_data::RawSaveData;
use crate::data_field::{DataField, DataFieldU32};

pub struct SaveDataManager {
    raw: RawSaveData,
    fields: HashMap<String, Box<dyn DataField>>,
}

impl SaveDataManager {
    pub fn from_raw_save_data(raw: &RawSaveData) -> SaveDataManager {
        let mut fields: HashMap<String, Box<dyn DataField>> = HashMap::new();
        fields.insert(String::from("checksum"), Box::new(DataFieldU32::new(0xC, 4)));

        SaveDataManager { raw: raw.clone(), fields }
    }

    pub fn calculate_checksum(&self) -> u32 {
        self.raw.calculate_checksum()
    }

    pub fn print(&self) {
        println!("calculated checksum: {:?}", self.calculate_checksum());
        for (key, value) in self.fields.iter() {
            println!("{:}: {:?}", key, value.as_ref().read(&self.raw));
        }
    }
}
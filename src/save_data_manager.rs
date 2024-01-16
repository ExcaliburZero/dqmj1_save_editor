use std::collections::BTreeMap;

use crate::data_fields::{DataField, DataFieldMonster, DataFieldU32, DataValue};
use crate::raw_save_data::RawSaveData;

pub struct SaveDataManager {
    raw: RawSaveData,
    fields: BTreeMap<String, Box<dyn DataField>>,
}

impl SaveDataManager {
    pub fn from_raw_save_data(raw: &RawSaveData) -> SaveDataManager {
        let mut fields: BTreeMap<String, Box<dyn DataField>> = BTreeMap::new();
        fields.insert(
            String::from("checksum"),
            Box::new(DataFieldU32::new(0xC, 4)),
        );
        fields.insert(String::from("gold"), Box::new(DataFieldU32::new(0x184, 4)));
        fields.insert(String::from("atm"), Box::new(DataFieldU32::new(0x188, 4)));
        fields.insert(
            String::from("party_monster_1"),
            Box::new(DataFieldMonster::new(0x65C)),
        );

        SaveDataManager {
            raw: raw.clone(),
            fields,
        }
    }

    pub fn calculate_checksum(&self) -> u32 {
        self.raw.calculate_checksum()
    }

    pub fn get(&self, name: &str) -> DataValue {
        self.fields
            .get(name)
            .unwrap()
            .as_ref()
            .read(&self.raw.raw)
            .unwrap()
    }

    pub fn set(&mut self, name: &str, value: &DataValue) {
        self.fields
            .get(name)
            .unwrap()
            .as_ref()
            .write(&mut self.raw.raw, value)
    }

    pub fn print(&self) {
        println!("calculated checksum: {:?}", self.calculate_checksum());
        for (key, value) in self.fields.iter() {
            println!("{:}: {:?}", key, value.as_ref().read(&self.raw.raw));
        }
    }
}

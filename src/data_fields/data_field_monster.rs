use std::collections::HashMap;

use crate::data_fields::data_field::{DataField, DataValue, ValueReadFailureReason};
use crate::data_fields::data_location::DataLocation;
use crate::data_fields::monster::Monster;

const MONSTER_DATA_LENGTH_BYTES: usize = 0x104;

pub struct DataFieldMonster {
    location: DataLocation,
}

impl DataFieldMonster {
    pub fn new(offset: usize) -> DataFieldMonster {
        DataFieldMonster {
            location: DataLocation::new(offset, MONSTER_DATA_LENGTH_BYTES),
        }
    }
}

impl DataFieldMonster {
    fn bytes_to_string(bytes: &[u8]) -> String {
        let mut character_map: HashMap<u8, char> = HashMap::new();
        character_map.insert(0x0B, 'A');
        character_map.insert(0x0C, 'B');
        character_map.insert(0x28, 'd');
        character_map.insert(0x39, 'u');
        character_map.insert(0x3D, 'y');

        let mut chars: Vec<char> = vec![];
        for b in bytes {
            if *b == 0xFF {
                break;
            }

            println!("{:}", b);

            chars.push(*character_map.get(b).unwrap());
        }

        chars.into_iter().collect()
    }
}

impl DataField for DataFieldMonster {
    fn read(&self, raw_save_data: &[u8]) -> Result<DataValue, ValueReadFailureReason> {
        //Ok(DataValue::Monster(Monster::from_le_bytes(
        //    self.location.read(raw_save_data).try_into().unwrap(),
        //)))

        let bytes = self.location.read(raw_save_data);
        let name = DataFieldMonster::bytes_to_string(&bytes[0..9]);

        let monster = Monster::new(&name);

        Ok(DataValue::Monster(monster))
    }

    fn write(&self, _raw_save_data: &mut [u8], _value: &DataValue) {
        panic!()
        /*if let DataValue::Monster(v) = value.clone() {
            let value_bytes = v.to_le_bytes().to_vec();

            self.location.write(raw_save_data, value_bytes);
        } else {
            panic!("Unhandled value type {:?}", value);
        }*/
    }
}

extern crate dqmj1_save_editor;

use std::env;
use std::fs::File;

use dqmj1_save_editor::raw_save_data::RawSaveData;
use dqmj1_save_editor::save_data_manager::SaveDataManager;

fn main() {
    let args: Vec<_> = env::args().collect();
    let filepath = &args[1];
    println!("{:}", filepath);

    let mut file = File::open(filepath).unwrap();
    let save_data_manager = SaveDataManager::from_raw_save_data(&RawSaveData::from_sav(&mut file).unwrap());

    save_data_manager.print();
}

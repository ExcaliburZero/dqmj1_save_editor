use phf::{phf_map, Map};

pub static CHARACTER_MAP: Map<u8, char> = phf_map! {
    0x0Bu8 => 'A',
    0x0Cu8 => 'B',
    0x28u8 => 'd',
    0x39u8 => 'u',
    0x3Du8 => 'y',
};

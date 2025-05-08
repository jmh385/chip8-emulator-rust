use crate::chip8_types::chip8_command::Chip8CommandData;

pub fn zero_precision(data: &Chip8CommandData) -> u16{
    data.nibble_1 as u16
}

pub fn single_precision(data: &Chip8CommandData) -> u16{
    let shifted_nibble_1 = ((data.nibble_1 as u16) << 8) as u16;
    shifted_nibble_1 + data.nibble_4 as u16
}

pub fn double_precision(data: &Chip8CommandData) -> u16{
    let shifted_nibble_1 = ((data.nibble_1 as u16) << 8) as u16;
    shifted_nibble_1 + data.byte_2 as u16
}
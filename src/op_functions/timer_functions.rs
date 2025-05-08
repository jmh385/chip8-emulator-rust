use crate::chip8_types::{chip8_command::Chip8CommandData, chip8_emulator::InMemoryChip8Emulator};


//FX07
pub fn get_delay_timer(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] = emulator.delay_timer as i32;
}

//FX15
pub fn set_delay_timer(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.delay_timer = emulator.registers[data.nibble_2 as usize] as u8;
}

//FX18
pub fn set_sound_timer(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.sound_timer = emulator.registers[data.nibble_2 as usize] as u8;
}
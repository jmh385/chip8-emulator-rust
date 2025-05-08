use crate::chip8_types::{chip8_command::Chip8CommandData, chip8_emulator::InMemoryChip8Emulator};

pub fn assign_y_to_x(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] = emulator.registers[data.nibble_3 as usize];
}

//8XY1
pub fn bitwise_or_x_y(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] |= emulator.registers[data.nibble_3 as usize];
}

//8XY2
pub fn bitwise_and_x_y(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] &= emulator.registers[data.nibble_3 as usize];
}

//8XY3
pub fn bitwise_xor_x_y(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] ^= emulator.registers[data.nibble_3 as usize];
}

//8XY4
pub fn add_x_y(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] += emulator.registers[data.nibble_3 as usize];
}

//8XY5
pub fn negate_x_y(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[15] = 0;
    emulator.registers[data.nibble_2 as usize] -= emulator.registers[data.nibble_3 as usize];
    if emulator.registers[data.nibble_2 as usize] < 0 {
        emulator.registers[data.nibble_2 as usize] = 0;
        emulator.registers[15] = 1;
    }
}

//8XY6
pub fn right_shift_x(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] >>= 1;
}

//8XY7
pub fn negate_y_x(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[15] = 0;
    emulator.registers[data.nibble_2 as usize] =
        emulator.registers[data.nibble_3 as usize] - emulator.registers[data.nibble_2 as usize];
    if emulator.registers[data.nibble_2 as usize] < 0 {
        emulator.registers[data.nibble_2 as usize] = 0;
        emulator.registers[15] = 1;
    }
}

//8XYE
pub fn left_shift_x(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] <<= 1;
}

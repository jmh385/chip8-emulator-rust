use crate::chip8_types::{chip8_command::Chip8CommandData, chip8_emulator::InMemoryChip8Emulator};

//3XNN
pub fn is_equal(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    let register_value = emulator.registers[data.nibble_2 as usize] as u8;
    if register_value == data.byte_2 {
        emulator.program_counter += 2;
    }
}

//4XNN
pub fn is_not_equal(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    let register_value = emulator.registers[data.nibble_2 as usize] as u8;
    if register_value != data.byte_2 {
        emulator.program_counter += 2;
    }
}

//5XY0
pub fn are_registers_equal(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    let register_x_value = emulator.registers[data.nibble_2 as usize];
    let register_y_value = emulator.registers[data.nibble_3 as usize];
    if register_x_value == register_y_value {
        emulator.program_counter += 2;
    }
}

//6XNN
pub fn set_register(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] = data.byte_2 as i32;
}

//7XNN
pub fn add_to_register(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] += data.byte_2 as i32;
}

//9XY0
pub fn are_registers_not_equal(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    let register_x_value = emulator.registers[data.nibble_2 as usize];
    let register_y_value = emulator.registers[data.nibble_3 as usize];
    if register_x_value != register_y_value {
        emulator.program_counter += 2;
    }
}

//ANNN
pub fn set_index_register(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.index_register = data.triple_byte_address;
}

//CXNN
pub fn bitwise_and_with_random(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.registers[data.nibble_2 as usize] = (data.byte_2 & rand::random::<u8>()) as i32;
}

//FX33
pub fn store_bcd_of_x(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    let mut register_x_value = emulator.registers[data.nibble_2 as usize];
    for i in (0..3).rev() {
        emulator.memory[emulator.index_register as usize + i] = register_x_value as u8 % 10;
        register_x_value /= 10;
    }
}

//FX1E
pub fn add_x_to_i(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.index_register += emulator.registers[data.nibble_2 as usize] as u16;
}

//FX55
pub fn dump_registers_to_i(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    for (index, register) in emulator.registers[0..data.nibble_2 as usize]
        .iter()
        .enumerate()
    {
        emulator.memory[emulator.index_register as usize + index * 2] = *register as u8;
    }
}

//FX65
pub fn load_registers_from_i(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    for index in 0..data.nibble_2 as usize {
        emulator.registers[index] =
            emulator.memory[emulator.index_register as usize + index * 2] as i32;
    }
}

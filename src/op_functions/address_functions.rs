use crate::chip8_types::{chip8_command::Chip8CommandData, chip8_emulator::InMemoryChip8Emulator};

//00EE
pub fn return_to_stack(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.program_counter = match emulator.stack.pop() {
        None => panic!("tried to return to a non existing subroutine"),
        Some(number) => number,
    };
}

//1NNN
pub fn jump(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.program_counter = data.triple_byte_address as u64;
}

//2NNN
pub fn call_subroutine(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.stack.push(emulator.program_counter);
    emulator.program_counter = data.triple_byte_address as u64;
}

//BNNN
pub fn jump_to_v0_plus_address(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    emulator.program_counter = (emulator.registers[0] as u16 + data.triple_byte_address) as u64;
}

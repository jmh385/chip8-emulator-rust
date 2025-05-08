use sdl2::keyboard::Scancode;

use crate::chip8_types::{chip8_command::Chip8CommandData, chip8_emulator::InMemoryChip8Emulator};

//EX9E
pub fn register_equal_clicked(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    match emulator.scancode_to_integer.get(&data.clicked_key) {
        None => {
            // println!("unreadable key, scancode was: {}", data.clicked_key);
        }
        Some(scancode_integer) => {
            if emulator.registers[data.nibble_2 as usize] == *scancode_integer as i32 {
                // println!("key: {} was clicked", data.clicked_key);
                emulator.program_counter += 2;
            }
        }
    }
}

//EXA1
pub fn register_not_equal_clicked(
    emulator: &mut InMemoryChip8Emulator,
    data: &mut Chip8CommandData,
) {
    match emulator.scancode_to_integer.get(&data.clicked_key) {
        None => {
            // println!("unreadable key, scancode was: {}\n", data.clicked_key);
        }
        Some(scancode_integer) => {
            // println!("key: {} was clicked ineteger: {scancode_integer}, register: {}\n", data.clicked_key, emulator.registers[data.nibble_2 as usize]);
            if emulator.registers[data.nibble_2 as usize] != *scancode_integer as i32 {
                emulator.program_counter += 2;
            }
        }
    }
}

//FX0A
pub fn set_register_clicked(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    if *data.clicked_key != Scancode::Application {
        match emulator.scancode_to_integer.get(&data.clicked_key) {
            None => (),
            Some(scancode_integer) => {
                emulator.registers[data.nibble_2 as usize] = *scancode_integer as i32
            }
        }
    } else {
        emulator.program_counter -= 2;
    }
}

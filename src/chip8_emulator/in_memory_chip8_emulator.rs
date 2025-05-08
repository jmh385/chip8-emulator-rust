use std::collections::HashMap;

use sdl2::keyboard::Scancode;

use crate::chip8_types::{chip8_command::{Chip8Command, Chip8CommandData, PrecisionCalculator}, chip8_emulator::InMemoryChip8Emulator};

impl InMemoryChip8Emulator {
    pub fn new(registers: Vec<i32>,
            memory: Vec<u8>,
            display_bits: Vec<Vec<bool>>,
            stack: Vec<u64>,
            delay_timer: u8,
            sound_timer: u8,
            program_counter: u64,
            index_register: u16,
            command_map: HashMap<u16, Chip8Command>,
            precision_map: HashMap<u8, PrecisionCalculator>,
            integer_to_scancode: HashMap<u8, Scancode>,
            scancode_to_integer: HashMap<Scancode, u8>,) -> InMemoryChip8Emulator {

        InMemoryChip8Emulator {
            registers,
            memory,
            display_bits,
            stack,
            delay_timer,
            sound_timer,
            program_counter,
            index_register,
            command_map,
            precision_map,
            integer_to_scancode,
            scancode_to_integer
        }
    }

    pub fn perform_action(&mut self, byte_1: u8, byte_2: u8, scancode: &mut Scancode) {
        let first_nibble = byte_1 / 16;
        let second_nibble = byte_1 as u32 % 16;
        let third_nibble = byte_2 / 16;
        let fourth_nibble = byte_2 % 16;

        let nibble_2 = second_nibble as u16;
        let shifted_nibble_2 = nibble_2 << 8;
        let triple_byte_address: u16 =  (shifted_nibble_2 + byte_2 as u16) as u16;

        let mut command_data: Chip8CommandData = Chip8CommandData {
            nibble_1: first_nibble as u8,
            nibble_2: second_nibble as u8,
            nibble_3: third_nibble,
            nibble_4: fourth_nibble,
            byte_1,
            byte_2,
            triple_byte_address,
            clicked_key: scancode,
        };

        let command_type = match self.precision_map.get(&first_nibble){
            None => panic!("this command does not exist"),
            Some(calculator) => calculator(&command_data)
        };

        match self.command_map.get(&command_type){
            None => println!("this command type does not exist"),
            Some(command) => {
                command(self, &mut command_data)
            },
        }
    }
}

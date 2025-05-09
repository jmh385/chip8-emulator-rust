use std::collections::HashMap;

use sdl2::keyboard::Scancode;

use super::chip8_command::{Chip8Command, PrecisionCalculator};

pub struct InMemoryChip8Emulator {
    pub(crate) registers: Vec<i32>,
    pub(crate) memory: Vec<u8>,
    pub(crate) display_bits: Vec<Vec<bool>>,
    pub(crate) stack: Vec<u64>,
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
    pub(crate) program_counter: u64,
    pub(crate) index_register: u16,
    pub(crate) command_map: HashMap<u16, Chip8Command>,
    pub(crate) precision_map: HashMap<u8, PrecisionCalculator>,
    pub(crate) integer_to_scancode: HashMap<u8, Scancode>,
    pub(crate) scancode_to_integer: HashMap<Scancode, u8>,
}

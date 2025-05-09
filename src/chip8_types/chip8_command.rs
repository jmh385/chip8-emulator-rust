use sdl2::keyboard::Scancode;

use super::chip8_emulator::InMemoryChip8Emulator;

pub struct Chip8CommandData<'a> {
    pub(crate) nibble_1: u8,
    pub(crate) nibble_2: u8,
    pub(crate) nibble_3: u8,
    pub(crate) nibble_4: u8,
    pub(crate) byte_1: u8,
    pub(crate) byte_2: u8,
    pub(crate) triple_byte_address: u16,
    pub(crate) clicked_key: &'a mut Scancode,
}

pub type Chip8Command = fn(_: &mut InMemoryChip8Emulator, _: &mut Chip8CommandData) -> ();

pub type PrecisionCalculator = fn(_: &Chip8CommandData) -> u16;

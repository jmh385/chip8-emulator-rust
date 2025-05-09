use std::collections::HashMap;

use chip8_types::{
    chip8_command::{Chip8Command, PrecisionCalculator},
    chip8_emulator::InMemoryChip8Emulator,
};
use emulator_runner::emulator_runner::{Device, Runner};
use op_functions::{
    address_functions::{call_subroutine, jump, jump_to_v0_plus_address, return_to_stack},
    display_functions::{clear_display, display},
    keyboard_functions::{
        register_equal_clicked, register_not_equal_clicked, set_register_clicked,
    },
    math_functions::{
        add_x_y, assign_y_to_x, bitwise_and_x_y, bitwise_or_x_y, bitwise_xor_x_y, left_shift_x,
        negate_x_y, negate_y_x, right_shift_x,
    },
    register_functions::{
        add_to_register, add_x_to_i, are_registers_equal, are_registers_not_equal,
        bitwise_and_with_random, dump_registers_to_i, is_equal, is_not_equal,
        load_registers_from_i, set_index_register, set_register, store_bcd_of_x,
    },
    timer_functions::{get_delay_timer, set_delay_timer, set_sound_timer},
};
use precision_calculators::precision_calculators::{
    double_precision, single_precision, zero_precision,
};
use sdl2::keyboard::Scancode;

mod chip8_emulator;
mod chip8_types;
mod emulator_runner;
mod op_functions;
mod precision_calculators;

fn bootstrap_device() -> Device {
    let mut command_precision: HashMap<u8, PrecisionCalculator> = HashMap::new();
    command_precision.insert(0x0, single_precision);
    command_precision.insert(0x1, zero_precision);
    command_precision.insert(0x2, zero_precision);
    command_precision.insert(0x3, zero_precision);
    command_precision.insert(0x4, zero_precision);
    command_precision.insert(0x5, zero_precision);
    command_precision.insert(0x6, zero_precision);
    command_precision.insert(0x7, zero_precision);
    command_precision.insert(0x8, single_precision);
    command_precision.insert(0x9, zero_precision);
    command_precision.insert(0xa, zero_precision);
    command_precision.insert(0xb, zero_precision);
    command_precision.insert(0xc, zero_precision);
    command_precision.insert(0xd, zero_precision);
    command_precision.insert(0xe, double_precision);
    command_precision.insert(0xf, double_precision);

    let mut command_map: HashMap<u16, Chip8Command> = HashMap::new();

    let shifted0x8: u16 = 0x8 << 8;
    command_map.insert(shifted0x8 + 0x0, assign_y_to_x);
    command_map.insert(shifted0x8 + 0x1, bitwise_or_x_y);
    command_map.insert(shifted0x8 + 0x2, bitwise_and_x_y);
    command_map.insert(shifted0x8 + 0x3, bitwise_xor_x_y);
    command_map.insert(shifted0x8 + 0x4, add_x_y);
    command_map.insert(shifted0x8 + 0x5, negate_x_y);
    command_map.insert(shifted0x8 + 0x6, right_shift_x);
    command_map.insert(shifted0x8 + 0x7, negate_y_x);
    command_map.insert(shifted0x8 + 0xe, left_shift_x);

    let shifted0xe: u16 = 0xe << 8;
    command_map.insert(shifted0xe + 0x9e, register_equal_clicked);
    command_map.insert(shifted0xe + 0xa1, register_not_equal_clicked);

    let shifted0xf: u16 = 0xf << 8;
    command_map.insert(shifted0xf + 0x07, get_delay_timer);
    command_map.insert(shifted0xf + 0x0a, set_register_clicked);
    command_map.insert(shifted0xf + 0x15, set_delay_timer);
    command_map.insert(shifted0xf + 0x18, set_sound_timer);
    command_map.insert(shifted0xf + 0x1e, add_x_to_i);
    command_map.insert(shifted0xf + 0x33, store_bcd_of_x);
    command_map.insert(shifted0xf + 0x55, dump_registers_to_i);
    command_map.insert(shifted0xf + 0x65, load_registers_from_i);

    command_map.insert(0x0, clear_display);
    command_map.insert(0x0e, return_to_stack);

    command_map.insert(0x1, jump);
    command_map.insert(0x2, call_subroutine);
    command_map.insert(0x3, is_equal);
    command_map.insert(0x4, is_not_equal);
    command_map.insert(0x5, are_registers_equal);
    command_map.insert(0x6, set_register);
    command_map.insert(0x7, add_to_register);
    command_map.insert(0x9, are_registers_not_equal);
    command_map.insert(0xa, set_index_register);
    command_map.insert(0xb, jump_to_v0_plus_address);
    command_map.insert(0xc, bitwise_and_with_random);
    command_map.insert(0xd, display);

    let integer_to_scancode: HashMap<u8, Scancode> = HashMap::from([
        (1, Scancode::Num1),
        (2, Scancode::Num2),
        (3, Scancode::Num3),
        (4, Scancode::Q),
        (5, Scancode::W),
        (6, Scancode::E),
        (0xd, Scancode::R),
        (7, Scancode::A),
        (8, Scancode::S),
        (9, Scancode::D),
        (0xe, Scancode::F),
        (0xa, Scancode::Z),
        (0xb, Scancode::C),
        (0xf, Scancode::V),
    ]);

    let scancode_to_integer: HashMap<Scancode, u8> = HashMap::from([
        (Scancode::Num1, 1),
        (Scancode::Num2, 2),
        (Scancode::Num3, 3),
        (Scancode::Num4, 0xc),
        (Scancode::Q, 4),
        (Scancode::W, 5),
        (Scancode::E, 6),
        (Scancode::R, 0xd),
        (Scancode::A, 7),
        (Scancode::S, 8),
        (Scancode::D, 9),
        (Scancode::F, 0xe),
        (Scancode::Z, 0xa),
        (Scancode::X, 0),
        (Scancode::C, 0xb),
        (Scancode::V, 0xf),
    ]);

    let registers: Vec<i32> = vec![0; 16];
    let memory: Vec<u8> = vec![0; 4096];
    let display_bits: Vec<Vec<bool>> = vec![vec![false; 64]; 32];
    let stack: Vec<u64> = Vec::new();
    let delay_timer: u8 = 0;
    let sound_timer: u8 = 0;
    let program_counter: u64 = 0;
    let index_register: u16 = 0;
    let emulator: InMemoryChip8Emulator = InMemoryChip8Emulator::new(
        registers,
        memory,
        display_bits,
        stack,
        delay_timer,
        sound_timer,
        program_counter,
        index_register,
        command_map,
        command_precision,
        integer_to_scancode,
        scancode_to_integer,
    );
    return Device::new(emulator, 800, 400);
}

fn main() -> Result<(), String> {
    bootstrap_device().run("path/to/file")?;
    return Ok(());
}

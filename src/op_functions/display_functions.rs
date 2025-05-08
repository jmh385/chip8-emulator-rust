use crate::chip8_types::{chip8_command::Chip8CommandData, chip8_emulator::InMemoryChip8Emulator};

//00E0
pub fn clear_display(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    for row in 0..emulator.display_bits.len() {
        for col in 0..emulator.display_bits[0].len() {
            emulator.display_bits[row][col] = false;
        }
    }
}

//DXYN
pub fn display(emulator: &mut InMemoryChip8Emulator, data: &mut Chip8CommandData) {
    let register_x = data.nibble_2;
    let register_y = data.nibble_3;
    let height = data.nibble_4;

    emulator.registers[15] = 0;
    let mut x: usize = (emulator.registers[register_x as usize] % 64) as usize;
    let mut y: usize = (emulator.registers[register_y as usize] % 32) as usize;
    // println!("displaying at: x: {x}, y: {y}, height: {height}");
    for row in 0..height {
        let data_from_index = (emulator.index_register + row as u16 as u16) as usize;
        let sprite_data = emulator.memory[data_from_index];
        // println!("sprite data at {data_from_index}, value: {sprite_data:b}");
        for bit_count in (0..8).rev() {
            let bit_value = (sprite_data >> bit_count) % 2;
            if bit_value == 1 {
                if x < emulator.display_bits[0].len() && y < emulator.display_bits.len() {
                    // println!("painted at: {},{}, bit:{}",x, y, emulator.display_bits[y][x]);

                    if emulator.display_bits[y][x] {
                        emulator.registers[15] = 1;
                    }

                    emulator.display_bits[y][x] = !emulator.display_bits[y][x];
                    // println!("display bit after: {}", emulator.display_bits[y][x]);
                }
            }

            x += 1;
        }

        x = (emulator.registers[register_x as usize] % 64) as usize;
        y += 1;
    }
}

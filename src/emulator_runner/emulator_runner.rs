use std::{fs, time::SystemTime};

use sdl2::{event::Event, keyboard::Scancode, pixels::Color, rect::Rect};

use crate::chip8_types::chip8_emulator::InMemoryChip8Emulator;

pub struct Device {
    pub in_memory_chip8_emulator: InMemoryChip8Emulator,
    pub height: u32,
    pub width: u32,
}

impl Device {
    pub fn new(in_memory_chip8_emulator: InMemoryChip8Emulator, height: u32, width: u32) -> Device {
        Device {
            in_memory_chip8_emulator,
            height,
            width,
        }
    }
}

pub trait Runner {
    fn run(&mut self, file_path: &str) -> Result<(), String>;
}

impl Runner for Device {
    fn run(&mut self, file_path: &str) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("display", self.width, self.height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let mut events = sdl_context.event_pump()?;

        let aspect_ratio = self.width / 64;

        let program_bytes: Vec<u8> =
            fs::read(file_path).expect("Should have been able to read the file");

        // println!("contents in instructions: ");
        // let mut instruction_coutner = 0;
        // for byte in program_bytes {
        //     let nib_1 = format!("{:x}", byte / 16);
        //     let nib_2 = format!("{:x}", byte % 16);
        //     print!("{byte:x}");
        //     if instruction_coutner == 1 {
        //         println!();
        //     }
        //     instruction_coutner += 1;
        //     instruction_coutner %= 2;
        // }

        // thread::sleep(Duration::from_secs(1));

        // let emulator: InMemoryChip8Emulator;
        // let actions: HashMap = HashMap::from([0x0, clear_display()]);

        // let data: u32 = 0b10111011;
        // for i in (0..8).rev() {
        //     let bit = (data >> i) % 2;
        //     print!("{bit:b} ");
        // }

        let mut move_count = 0;
        let mut now = SystemTime::now();

        let mut index: usize = 0x200;
        for program_byte in program_bytes {
            self.in_memory_chip8_emulator.memory[index] = program_byte;
            index += 1;
        }

        self.in_memory_chip8_emulator.program_counter = 0x200;

        // for (index, byte) in self.in_memory_chip8_emulator.memory.iter().enumerate() {
        //     if index % 2 == 0 {
        //         print!("{index:x}: {byte:x}");
        //     }
        //     else {
        //         println!(" {byte:x}, ");
        //     }
        // }

        let mut click_key = Scancode::Application;

        'running: loop {
            //fetch
            let mut has_drawn = false;
            let current_pos = self.in_memory_chip8_emulator.program_counter as usize;
            let bytes = &self.in_memory_chip8_emulator.memory[current_pos..current_pos + 2];
            self.in_memory_chip8_emulator.program_counter += 2;

            for event in events.poll_iter() {
                if let Event::KeyDown { scancode, .. } = event {
                    match scancode {
                        None => (),
                        Some(scancode) => click_key = scancode,
                    }
                }
                if let Event::KeyUp { .. } = event {
                    click_key = Scancode::Application;
                }
                if let Event::Quit { .. } = event {
                    break 'running;
                };
            }

            // println!("{move_count}. instruction no: {:x} performing: {:x} {:x}\nregister state: {:?}\nindex: {:x}\n", emulator.program_counter, bytes[0], bytes[1], emulator.registers, emulator.index_register);
            //decode + excute
            if bytes[0] / 16 == 0xd {
                has_drawn = true;
            }

            self.in_memory_chip8_emulator
                .perform_action(bytes[0], bytes[1], &mut click_key);
            if click_key != Scancode::Application {
                println!("clicked_key: {click_key}");
            }

            //draw
            if has_drawn {
                for row in 0..32 {
                    for col in 0..64 {
                        let x = col * aspect_ratio;
                        let y = row * aspect_ratio;
                        if self.in_memory_chip8_emulator.display_bits[row as usize][col as usize] {
                            // println!("got here at {}, {}", row, col);
                            // println!("filling rect at width: {x}, height: {y}");
                            canvas.set_draw_color(Color::RGB(255, 255, 255));
                        } else {
                            canvas.set_draw_color(Color::RGB(0, 0, 0));
                        }
                        canvas.fill_rect(Rect::new(
                            x as i32,
                            y as i32,
                            aspect_ratio as u32,
                            aspect_ratio as u32,
                        ))?;
                    }
                }
            }

            canvas.present();

            match now.elapsed() {
                Ok(elapsed) => {
                    // println!("elapsed: {}, timer: {}", elapsed.as_millis(), emulator.delay_timer);
                    if elapsed.as_millis() >= 1000 {
                        self.in_memory_chip8_emulator.delay_timer =
                            if self.in_memory_chip8_emulator.delay_timer > 0 {
                                self.in_memory_chip8_emulator.delay_timer - 1
                            } else {
                                0
                            };
                        self.in_memory_chip8_emulator.sound_timer =
                            if self.in_memory_chip8_emulator.sound_timer > 0 {
                                self.in_memory_chip8_emulator.sound_timer - 1
                            } else {
                                0
                            };
                        now = SystemTime::now();
                    }
                }
                Err(_) => (),
            }

            //refresh rate
            // if move_count == 91 {
            //     thread::sleep(Duration::from_secs(120));
            // }

            move_count += 1;

            if has_drawn {
                // thread::sleep(Duration::from_millis(1));
            }
        }

        return Ok(());
    }
}

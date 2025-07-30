use std::{thread::sleep, time::Duration, u32, usize};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
#[derive(Clone)]
pub struct Screen {
    // 32x64, should be easier to set it to a continious memory to deal with overflows per line of
    // display
    pub display: [bool; SCREEN_SIZE],
}

impl Screen {
    pub fn clear_screen(&mut self) {
        self.display = [false; SCREEN_SIZE];
    }
}

impl Screen {
    pub fn update_screen_state(&mut self, start_x: u8, start_y: u8, data: Vec<u8>) -> bool {
        let start_target = (start_x as u32) + (start_y as u32 * 64);
        let mut is_flipped = false;

        let binary: Vec<[u8; 1]> = data.into_iter().map(|f| f.to_be_bytes()).collect();
        let flattened_binary = binary.as_flattened();

        for (i, bit) in flattened_binary.into_iter().enumerate() {
            println!("{:?}", bit);
            sleep(Duration::from_millis(400));
            match self.display[start_target as usize + i] {
                true => {
                    if bit.clone() == 0 {
                        is_flipped = true
                    }
                }
                false => {
                    if bit.clone() == 1 {
                        is_flipped = true
                    }
                }
            }

            match bit {
                0 => self.display[start_target as usize + i] = false,
                _ => self.display[start_target as usize + i] = true,
            }
        }
        return is_flipped;
    }
    pub fn draw(&self) {
        // Top border
        print!("╔"); // top-left corner
        for _ in 0..SCREEN_WIDTH {
            print!("═");
        }
        println!("╗"); // top-right corner

        // Each row
        for height in 0..SCREEN_HEIGHT {
            print!("║"); // left side
            for width in 0..SCREEN_WIDTH {
                let pixel = self.display[64 * height + width];
                match pixel {
                    true => print!("*"),
                    false => print!(" "),
                }
            }
            println!("║"); // right side
        }

        // Bottom border
        print!("╚"); // bottom-left corner
        for _ in 0..SCREEN_WIDTH {
            print!("═");
        }
        println!("╝"); // bottom-right corner
    }
}

impl Screen {
    pub fn new() -> Screen {
        return Screen {
            display: [false; SCREEN_SIZE],
        };
    }
}

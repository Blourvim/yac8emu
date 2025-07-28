const SCREEN_SIZE: usize = 2048;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
#[derive(Clone)]
pub struct Screen {
    // 32x64, should be easier to set it to a continious memory to deal with overflows per line of
    // display
    display: [bool; SCREEN_SIZE],
}

impl Screen {
    pub fn clear_screen(&mut self) {
        self.display = [false; SCREEN_SIZE];
    }
}

impl Screen {
    pub fn draw(&self) {
        for height in 0..SCREEN_HEIGHT {
            print!("||");
            for width in 0..SCREEN_WIDTH {
                let pixel = self.display[64 * height + width];
                match pixel {
                    true => print!("X"),
                    false => print!("O"),
                }
            }
            println!("||")
        }
    }
}

impl Screen {
    pub fn new() -> Screen {
        return Screen {
            display: [false; SCREEN_SIZE],
        };
    }
}

const SCREEN_SIZE: usize = 2048;
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
    pub fn new() -> Screen {
        return Screen {
            display: [false; SCREEN_SIZE],
        };
    }
}

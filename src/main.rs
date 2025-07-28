use std::{thread::sleep, time::Duration};

use machine::{rom, screen::Screen};

mod machine;

fn main() {
    // init machine
    let mut machine = machine::machine::Machine::new();
    let mut my_screen = Screen::new();

    my_screen.draw();
    // load rom to memory
    machine = rom::Rom::load("./src/roms/test_opcode.ch8", machine).expect("rom load");

    loop {
        sleep(Duration::from_millis(1000));
        let screen = machine.exec();
    }
}

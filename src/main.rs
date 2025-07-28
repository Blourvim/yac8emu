use std::{thread::sleep, time::Duration};

use machine::{rom, screen::{self, Screen}};

mod machine;

fn main() {
    // init machine
    let mut machine = machine::machine::Machine::new();
    let mut my_screen = Screen::new();
    // load rom to memory
    machine = rom::Rom::load("./src/roms/rom1", machine).expect("rom load");

    loop {
        sleep(Duration::from_millis(1000));
        machine.exec();
    }
}

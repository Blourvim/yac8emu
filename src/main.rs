use std::{thread::sleep, time::Duration};

use machine::{rom, screen::Screen};

mod machine;

fn main() {
    // init machine
    let mut machine = machine::machine::Machine::new();

    // load rom to memory
    machine = rom::Rom::load("./src/roms/2-ibm-logo.ch8", machine).expect("rom load");

    loop {
        sleep(Duration::from_millis(100));
        print!("{}[2J", 27 as char);

        println!("{:#x}", machine.read_index_register());
        println!("{:#x}", machine.read_program_counter());

        machine.exec();
        machine.screen.draw();
    }
}

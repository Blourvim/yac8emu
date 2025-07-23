use machine::rom;

mod machine;

fn main() {
    // init machine
    let machine = machine::machine::Machine::new();
    // load rom to memory
    let machine_loaded= rom::Rom::load("./src/roms/rom1", machine);

    println!("Hello, world!");
}
